use crate::state::{Project, ProjectConfig, ProjectStatus, PortConfig, ServiceConfig};
use handlebars::Handlebars;
use serde_json::json;
use std::fs;
use std::path::Path;
use uuid::Uuid;
use chrono::Utc;

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);
        Self { handlebars }
    }

    pub fn render_template(&self, template_content: &str, data: &serde_json::Value) -> Result<String, String> {
        self.handlebars
            .render_template(template_content, data)
            .map_err(|e| e.to_string())
    }
}

pub fn create_project_from_template(
    name: &str,
    template_type: &str,
    base_path: &str,
    templates_path: &str,
    config: ProjectConfig,
) -> Result<Project, String> {
    println!("[Template] Starting project creation: {}", name);
    
    let project_id = Uuid::new_v4().to_string();
    let project_path = Path::new(base_path).join(name);
    println!("[Template] Project path: {:?}", project_path);

    // Create project directory structure
    fs::create_dir_all(&project_path).map_err(|e| format!("Failed to create project directory: {}", e))?;
    println!("[Template] Created project directory");
    
    fs::create_dir_all(project_path.join("src")).map_err(|e| format!("Failed to create src directory: {}", e))?;
    fs::create_dir_all(project_path.join("docker")).map_err(|e| format!("Failed to create docker directory: {}", e))?;
    println!("[Template] Created subdirectories");

    // Load template config
    let template_dir = Path::new(templates_path).join(template_type);
    let stubs_dir = template_dir.join("stubs");
    println!("[Template] Template dir: {:?}", template_dir);
    println!("[Template] Stubs dir: {:?}", stubs_dir);

    if !template_dir.exists() {
        println!("[Template] ERROR: Template directory does not exist!");
        return Err(format!("Template '{}' not found at {:?}", template_type, template_dir));
    }

    // Prepare template data
    let template_data = prepare_template_data(name, &config);
    let _engine = TemplateEngine::new();

    // Process each stub file
    let stub_mappings = get_stub_mappings(template_type);
    println!("[Template] Processing {} stub files", stub_mappings.len());

    for (stub_name, output_path) in stub_mappings {
        let stub_path = stubs_dir.join(stub_name);
        println!("[Template] Processing stub: {} -> {}", stub_name, output_path);
        
        if stub_path.exists() {
            let stub_content = fs::read_to_string(&stub_path)
                .map_err(|e| format!("Failed to read stub {}: {}", stub_name, e))?;

            // Replace handlebars-style placeholders with actual values
            let rendered = render_stub(&stub_content, &template_data);

            let output_file = project_path.join(output_path);
            if let Some(parent) = output_file.parent() {
                fs::create_dir_all(parent).ok();
            }

            fs::write(&output_file, rendered)
                .map_err(|e| format!("Failed to write {}: {}", output_path, e))?;
            println!("[Template] Written: {}", output_path);
        } else {
            println!("[Template] WARN: Stub not found: {:?}", stub_path);
        }
    }

    let now = Utc::now();

    Ok(Project {
        id: project_id,
        name: name.to_string(),
        template: template_type.to_string(),
        path: project_path.to_string_lossy().to_string(),
        created_at: now,
        updated_at: now,
        config,
        status: ProjectStatus::Stopped,
    })
}

fn prepare_template_data(name: &str, config: &ProjectConfig) -> serde_json::Value {
    // Generate Redis service block
    let redis_service = if config.services.redis {
        format!(r#"
  redis:
    image: redis:7-alpine
    container_name: {}_redis
    ports:
      - "{}:6379"
    volumes:
      - redis_data:/data
    networks:
      - {}_network
    restart: unless-stopped
    command: redis-server --appendonly yes
"#, name, config.ports.redis, name)
    } else {
        String::new()
    };

    // Generate phpMyAdmin service block
    let phpmyadmin_service = if config.services.phpmyadmin {
        format!(r#"
  phpmyadmin:
    image: phpmyadmin:latest
    container_name: {}_phpmyadmin
    environment:
      PMA_HOST: db
      PMA_USER: root
      PMA_PASSWORD: password
      PMA_ARBITRARY: 1
    ports:
      - "{}:80"
    depends_on:
      db:
        condition: service_healthy
    networks:
      - {}_network
    restart: unless-stopped
"#, name, config.ports.phpmyadmin, name)
    } else {
        String::new()
    };

    // Generate Mailhog service block
    let mailhog_service = if config.services.mailhog {
        format!(r#"
  mailhog:
    image: mailhog/mailhog:latest
    container_name: {}_mailhog
    ports:
      - "1025:1025"
      - "{}:8025"
    networks:
      - {}_network
    restart: unless-stopped
"#, name, config.ports.mailhog, name)
    } else {
        String::new()
    };

    json!({
        "PROJECT_NAME": name,
        "PHP_VERSION": config.php_version.clone().unwrap_or("8.4".to_string()),
        "NODE_VERSION": config.node_version,
        "INSTALL_BUN": config.install_bun.to_string(),
        "INSTALL_PNPM": config.install_pnpm.to_string(),
        "APP_PORT": config.ports.app,
        "VITE_PORT": config.ports.vite,
        "DB_PORT": config.ports.db,
        "REDIS_PORT": config.ports.redis,
        "PHPMYADMIN_PORT": config.ports.phpmyadmin,
        "MAILHOG_PORT": config.ports.mailhog,
        "REDIS_DEPENDS": if config.services.redis { "\n      - redis" } else { "" },
        "REDIS_SERVICE": redis_service,
        "REDIS_VOLUME": if config.services.redis { "\n  redis_data:\n    driver: local" } else { "" },
        "PHPMYADMIN_SERVICE": phpmyadmin_service,
        "MAILHOG_SERVICE": mailhog_service,
    })
}

fn render_stub(content: &str, data: &serde_json::Value) -> String {
    let mut result = content.to_string();

    if let Some(obj) = data.as_object() {
        for (key, value) in obj {
            let placeholder = format!("{{{{{}}}}}", key);
            let replacement = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &replacement);
        }
    }

    result
}

fn get_stub_mappings(template_type: &str) -> Vec<(&'static str, &'static str)> {
    match template_type {
        "laravel" => vec![
            ("docker-compose.yml.stub", "docker-compose.yml"),
            ("Dockerfile.stub", "Dockerfile"),
            ("Makefile.stub", "Makefile"),
            (".env.stub", ".env"),
            ("php.ini.stub", "docker/php.ini"),
            ("nginx.conf.stub", "docker/nginx.conf"),
            ("mysql.cnf.stub", "docker/mysql.cnf"),
            ("mysql-client.cnf.stub", "docker/mysql-client.cnf"),
            ("supervisor.conf.stub", "docker/supervisor.conf"),
            ("init.sql.stub", "docker/init.sql"),
        ],
        "nodejs" => vec![
            ("docker-compose.yml.stub", "docker-compose.yml"),
            ("Dockerfile.stub", "Dockerfile"),
            ("Makefile.stub", "Makefile"),
            (".env.stub", ".env"),
            ("mysql.cnf.stub", "docker/mysql.cnf"),
            ("package.json.stub", "src/package.json"),
            ("index.js.stub", "src/index.js"),
        ],
        _ => vec![],
    }
}

pub fn load_project_from_path(path: &str) -> Result<Project, String> {
    let project_path = Path::new(path);
    let config_path = project_path.join(".godmode.json");

    if config_path.exists() {
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read project config: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse project config: {}", e))
    } else {
        Err("Project config not found".to_string())
    }
}

pub fn save_project(project: &Project) -> Result<(), String> {
    let config_path = Path::new(&project.path).join(".godmode.json");
    let content = serde_json::to_string_pretty(project)
        .map_err(|e| format!("Failed to serialize project: {}", e))?;
    fs::write(&config_path, content)
        .map_err(|e| format!("Failed to save project config: {}", e))
}
