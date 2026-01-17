use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;
use chrono::Utc;
use crate::state::{Project, ProjectConfig, ProjectStatus, PortConfig, ServiceConfig};
use crate::template::save_project;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInstance {
    #[serde(rename = "blockId")]
    pub block_id: String,
    pub enabled: bool,
    pub version: Option<String>,
    pub config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub blocks: Vec<BlockInstance>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

pub fn generate_docker_compose_from_blocks(project_name: &str, blocks: &[BlockInstance]) -> String {
    let mut services = String::new();
    let mut volumes = Vec::new();
    let mut networks_needed = true;

    for block in blocks {
        if !block.enabled {
            continue;
        }

        match block.block_id.as_str() {
            "php-fpm" => {
                let version = block.version.as_deref().unwrap_or("8.4");
                let memory_limit = get_config_str(&block.config, "memory_limit", "256M");
                let max_execution_time = get_config_int(&block.config, "max_execution_time", 30);
                let upload_max = get_config_str(&block.config, "upload_max_filesize", "64M");

                services.push_str(&format!(r#"
  app:
    build:
      context: .
      dockerfile: Dockerfile
      args:
        PHP_VERSION: "{}"
    container_name: {}_app
    volumes:
      - ./src:/var/www/html
      - ./docker/php.ini:/usr/local/etc/php/conf.d/custom.ini
    environment:
      PHP_MEMORY_LIMIT: "{}"
      PHP_MAX_EXECUTION_TIME: "{}"
      PHP_UPLOAD_MAX_FILESIZE: "{}"
    networks:
      - {}_network
    restart: unless-stopped
"#, version, project_name, memory_limit, max_execution_time, upload_max, project_name));
                volumes.push("app_data".to_string());
            }

            "nginx" => {
                let port = get_config_int(&block.config, "port", 80);
                services.push_str(&format!(r#"
  nginx:
    image: nginx:alpine
    container_name: {}_nginx
    ports:
      - "{}:80"
    volumes:
      - ./src:/var/www/html:ro
      - ./docker/nginx.conf:/etc/nginx/conf.d/default.conf:ro
    depends_on:
      - app
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, port, project_name));
            }

            "caddy" => {
                let port = get_config_int(&block.config, "port", 80);
                let https_port = get_config_int(&block.config, "https_port", 443);
                services.push_str(&format!(r#"
  caddy:
    image: caddy:alpine
    container_name: {}_caddy
    ports:
      - "{}:80"
      - "{}:443"
    volumes:
      - ./src:/srv
      - ./docker/Caddyfile:/etc/caddy/Caddyfile:ro
      - caddy_data:/data
      - caddy_config:/config
    depends_on:
      - app
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, port, https_port, project_name));
                volumes.push("caddy_data".to_string());
                volumes.push("caddy_config".to_string());
            }

            "mysql" => {
                let version = block.version.as_deref().unwrap_or("8.0");
                let port = get_config_int(&block.config, "port", 3306);
                let database = get_config_str(&block.config, "database", "laravel");
                let username = get_config_str(&block.config, "username", "laravel");
                let password = get_config_str(&block.config, "password", "secret");
                let root_password = get_config_str(&block.config, "root_password", "secret");

                services.push_str(&format!(r#"
  db:
    image: mysql:{}
    container_name: {}_db
    ports:
      - "{}:3306"
    environment:
      MYSQL_DATABASE: "{}"
      MYSQL_USER: "{}"
      MYSQL_PASSWORD: "{}"
      MYSQL_ROOT_PASSWORD: "{}"
    volumes:
      - mysql_data:/var/lib/mysql
      - ./docker/mysql.cnf:/etc/mysql/conf.d/custom.cnf:ro
    networks:
      - {}_network
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost", "-u", "root", "-p{}"]
      interval: 5s
      timeout: 5s
      retries: 10
      start_period: 30s
    restart: unless-stopped
"#, version, project_name, port, database, username, password, root_password, project_name, root_password));
                volumes.push("mysql_data".to_string());
            }

            "mariadb" => {
                let version = block.version.as_deref().unwrap_or("10.11");
                let port = get_config_int(&block.config, "port", 3306);
                let database = get_config_str(&block.config, "database", "laravel");
                let username = get_config_str(&block.config, "username", "laravel");
                let password = get_config_str(&block.config, "password", "secret");

                services.push_str(&format!(r#"
  db:
    image: mariadb:{}
    container_name: {}_db
    ports:
      - "{}:3306"
    environment:
      MARIADB_DATABASE: "{}"
      MARIADB_USER: "{}"
      MARIADB_PASSWORD: "{}"
      MARIADB_ROOT_PASSWORD: "{}"
    volumes:
      - mariadb_data:/var/lib/mysql
    networks:
      - {}_network
    restart: unless-stopped
"#, version, project_name, port, database, username, password, password, project_name));
                volumes.push("mariadb_data".to_string());
            }

            "postgresql" => {
                let version = block.version.as_deref().unwrap_or("16");
                let port = get_config_int(&block.config, "port", 5432);
                let database = get_config_str(&block.config, "database", "laravel");
                let username = get_config_str(&block.config, "username", "laravel");
                let password = get_config_str(&block.config, "password", "secret");

                services.push_str(&format!(r#"
  db:
    image: postgres:{}-alpine
    container_name: {}_db
    ports:
      - "{}:5432"
    environment:
      POSTGRES_DB: "{}"
      POSTGRES_USER: "{}"
      POSTGRES_PASSWORD: "{}"
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - {}_network
    restart: unless-stopped
"#, version, project_name, port, database, username, password, project_name));
                volumes.push("postgres_data".to_string());
            }

            "mongodb" => {
                let version = block.version.as_deref().unwrap_or("7.0");
                let port = get_config_int(&block.config, "port", 27017);

                services.push_str(&format!(r#"
  mongodb:
    image: mongo:{}
    container_name: {}_mongodb
    ports:
      - "{}:27017"
    volumes:
      - mongodb_data:/data/db
    networks:
      - {}_network
    restart: unless-stopped
"#, version, project_name, port, project_name));
                volumes.push("mongodb_data".to_string());
            }

            "redis" => {
                let version = block.version.as_deref().unwrap_or("7.2");
                let port = get_config_int(&block.config, "port", 6379);
                let maxmemory = get_config_str(&block.config, "maxmemory", "256mb");

                services.push_str(&format!(r#"
  redis:
    image: redis:{}-alpine
    container_name: {}_redis
    ports:
      - "{}:6379"
    command: redis-server --appendonly yes --maxmemory {} --maxmemory-policy allkeys-lru
    volumes:
      - redis_data:/data
    networks:
      - {}_network
    restart: unless-stopped
"#, version, project_name, port, maxmemory, project_name));
                volumes.push("redis_data".to_string());
            }

            "memcached" => {
                let port = get_config_int(&block.config, "port", 11211);
                let memory = get_config_int(&block.config, "memory", 64);

                services.push_str(&format!(r#"
  memcached:
    image: memcached:alpine
    container_name: {}_memcached
    ports:
      - "{}:11211"
    command: memcached -m {}
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, port, memory, project_name));
            }

            "meilisearch" => {
                let version = block.version.as_deref().unwrap_or("1.6");
                let port = get_config_int(&block.config, "port", 7700);
                let master_key = get_config_str(&block.config, "master_key", "masterKey");

                services.push_str(&format!(r#"
  meilisearch:
    image: getmeili/meilisearch:v{}
    container_name: {}_meilisearch
    ports:
      - "{}:7700"
    environment:
      MEILI_MASTER_KEY: "{}"
    volumes:
      - meilisearch_data:/meili_data
    networks:
      - {}_network
    restart: unless-stopped
"#, version, project_name, port, master_key, project_name));
                volumes.push("meilisearch_data".to_string());
            }

            "elasticsearch" => {
                let version = block.version.as_deref().unwrap_or("8.12");
                let port = get_config_int(&block.config, "port", 9200);
                let java_opts = get_config_str(&block.config, "java_opts", "-Xms512m -Xmx512m");

                services.push_str(&format!(r#"
  elasticsearch:
    image: elasticsearch:{}
    container_name: {}_elasticsearch
    ports:
      - "{}:9200"
      - "9300:9300"
    environment:
      discovery.type: single-node
      ES_JAVA_OPTS: "{}"
      xpack.security.enabled: "false"
    volumes:
      - elasticsearch_data:/usr/share/elasticsearch/data
    networks:
      - {}_network
    restart: unless-stopped
"#, version, project_name, port, java_opts, project_name));
                volumes.push("elasticsearch_data".to_string());
            }

            "typesense" => {
                let version = block.version.as_deref().unwrap_or("0.25");
                let port = get_config_int(&block.config, "port", 8108);
                let api_key = get_config_str(&block.config, "api_key", "xyz");

                services.push_str(&format!(r#"
  typesense:
    image: typesense/typesense:{}
    container_name: {}_typesense
    ports:
      - "{}:8108"
    environment:
      TYPESENSE_API_KEY: "{}"
      TYPESENSE_DATA_DIR: /data
    volumes:
      - typesense_data:/data
    networks:
      - {}_network
    restart: unless-stopped
"#, version, project_name, port, api_key, project_name));
                volumes.push("typesense_data".to_string());
            }

            "rabbitmq" => {
                let version = block.version.as_deref().unwrap_or("3.13");
                let port = get_config_int(&block.config, "port", 5672);
                let mgmt_port = get_config_int(&block.config, "management_port", 15672);
                let username = get_config_str(&block.config, "username", "guest");
                let password = get_config_str(&block.config, "password", "guest");

                services.push_str(&format!(r#"
  rabbitmq:
    image: rabbitmq:{}-management-alpine
    container_name: {}_rabbitmq
    ports:
      - "{}:5672"
      - "{}:15672"
    environment:
      RABBITMQ_DEFAULT_USER: "{}"
      RABBITMQ_DEFAULT_PASS: "{}"
    volumes:
      - rabbitmq_data:/var/lib/rabbitmq
    networks:
      - {}_network
    restart: unless-stopped
"#, version, project_name, port, mgmt_port, username, password, project_name));
                volumes.push("rabbitmq_data".to_string());
            }

            "beanstalkd" => {
                let port = get_config_int(&block.config, "port", 11300);

                services.push_str(&format!(r#"
  beanstalkd:
    image: schickling/beanstalkd
    container_name: {}_beanstalkd
    ports:
      - "{}:11300"
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, port, project_name));
            }

            "mailhog" => {
                let smtp_port = get_config_int(&block.config, "smtp_port", 1025);
                let ui_port = get_config_int(&block.config, "ui_port", 8025);

                services.push_str(&format!(r#"
  mailhog:
    image: mailhog/mailhog:latest
    container_name: {}_mailhog
    ports:
      - "{}:1025"
      - "{}:8025"
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, smtp_port, ui_port, project_name));
            }

            "mailpit" => {
                let smtp_port = get_config_int(&block.config, "smtp_port", 1025);
                let ui_port = get_config_int(&block.config, "ui_port", 8025);

                services.push_str(&format!(r#"
  mailpit:
    image: axllent/mailpit:latest
    container_name: {}_mailpit
    ports:
      - "{}:1025"
      - "{}:8025"
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, smtp_port, ui_port, project_name));
            }

            "phpmyadmin" => {
                let port = get_config_int(&block.config, "port", 8080);

                services.push_str(&format!(r#"
  phpmyadmin:
    image: phpmyadmin:latest
    container_name: {}_phpmyadmin
    environment:
      PMA_HOST: db
      PMA_USER: root
      PMA_PASSWORD: secret
      PMA_ARBITRARY: 1
    ports:
      - "{}:80"
    depends_on:
      db:
        condition: service_healthy
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, port, project_name));
            }

            "adminer" => {
                let port = get_config_int(&block.config, "port", 8081);

                services.push_str(&format!(r#"
  adminer:
    image: adminer:latest
    container_name: {}_adminer
    ports:
      - "{}:8080"
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, port, project_name));
            }

            "redisinsight" => {
                let port = get_config_int(&block.config, "port", 8001);

                services.push_str(&format!(r#"
  redisinsight:
    image: redislabs/redisinsight:latest
    container_name: {}_redisinsight
    ports:
      - "{}:8001"
    volumes:
      - redisinsight_data:/db
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, port, project_name));
                volumes.push("redisinsight_data".to_string());
            }

            "soketi" => {
                let port = get_config_int(&block.config, "port", 6001);
                let app_id = get_config_str(&block.config, "app_id", "app-id");
                let app_key = get_config_str(&block.config, "app_key", "app-key");
                let app_secret = get_config_str(&block.config, "app_secret", "app-secret");

                services.push_str(&format!(r#"
  soketi:
    image: quay.io/soketi/soketi:latest
    container_name: {}_soketi
    ports:
      - "{}:6001"
    environment:
      SOKETI_DEFAULT_APP_ID: "{}"
      SOKETI_DEFAULT_APP_KEY: "{}"
      SOKETI_DEFAULT_APP_SECRET: "{}"
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, port, app_id, app_key, app_secret, project_name));
            }

            "minio" => {
                let port = get_config_int(&block.config, "port", 9000);
                let console_port = get_config_int(&block.config, "console_port", 9001);
                let root_user = get_config_str(&block.config, "root_user", "minioadmin");
                let root_password = get_config_str(&block.config, "root_password", "minioadmin");

                services.push_str(&format!(r#"
  minio:
    image: minio/minio:latest
    container_name: {}_minio
    ports:
      - "{}:9000"
      - "{}:9001"
    environment:
      MINIO_ROOT_USER: "{}"
      MINIO_ROOT_PASSWORD: "{}"
    command: server /data --console-address ":9001"
    volumes:
      - minio_data:/data
    networks:
      - {}_network
    restart: unless-stopped
"#, project_name, port, console_port, root_user, root_password, project_name));
                volumes.push("minio_data".to_string());
            }

            "supervisor" => {
                // Supervisor is embedded in the app container for Laravel
                // This is handled in the Dockerfile
            }

            "nodejs" | "reverb" => {
                // These are typically part of the main app container
            }

            _ => {
                println!("Unknown block: {}", block.block_id);
            }
        }
    }

    // Build the final docker-compose
    let mut compose = String::from("version: '3.8'\n\nservices:");
    compose.push_str(&services);

    // Add volumes
    if !volumes.is_empty() {
        compose.push_str("\n\nvolumes:");
        for vol in &volumes {
            compose.push_str(&format!("\n  {}:\n    driver: local", vol));
        }
    }

    // Add networks
    if networks_needed {
        compose.push_str(&format!(r#"

networks:
  {}_network:
    driver: bridge
"#, project_name));
    }

    compose
}

fn get_config_str(config: &HashMap<String, serde_json::Value>, key: &str, default: &str) -> String {
    config.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or(default)
        .to_string()
}

fn get_config_int(config: &HashMap<String, serde_json::Value>, key: &str, default: i64) -> i64 {
    config.get(key)
        .and_then(|v| v.as_i64())
        .unwrap_or(default)
}

pub fn create_project_from_custom_template(
    project_name: &str,
    template: &CustomTemplate,
    base_path: &str,
) -> Result<Project, String> {
    use std::path::Path;

    let project_id = Uuid::new_v4().to_string();
    let project_path = Path::new(base_path).join(project_name);

    // Create project directory structure
    fs::create_dir_all(&project_path)
        .map_err(|e| format!("Failed to create project directory: {}", e))?;
    fs::create_dir_all(project_path.join("src"))
        .map_err(|e| format!("Failed to create src directory: {}", e))?;
    fs::create_dir_all(project_path.join("docker"))
        .map_err(|e| format!("Failed to create docker directory: {}", e))?;

    // Generate docker-compose.yml
    let docker_compose = generate_docker_compose_from_blocks(project_name, &template.blocks);
    fs::write(project_path.join("docker-compose.yml"), &docker_compose)
        .map_err(|e| format!("Failed to write docker-compose.yml: {}", e))?;

    // Generate basic Dockerfile for PHP if php-fpm is enabled
    let has_php = template.blocks.iter().any(|b| b.block_id == "php-fpm" && b.enabled);
    if has_php {
        let php_version = template.blocks.iter()
            .find(|b| b.block_id == "php-fpm" && b.enabled)
            .and_then(|b| b.version.clone())
            .unwrap_or_else(|| "8.4".to_string());

        let dockerfile = generate_php_dockerfile(&php_version);
        fs::write(project_path.join("Dockerfile"), &dockerfile)
            .map_err(|e| format!("Failed to write Dockerfile: {}", e))?;

        // Generate php.ini
        fs::write(project_path.join("docker/php.ini"), generate_php_ini())
            .map_err(|e| format!("Failed to write php.ini: {}", e))?;

        // Generate nginx.conf if nginx is enabled
        let has_nginx = template.blocks.iter().any(|b| b.block_id == "nginx" && b.enabled);
        if has_nginx {
            fs::write(project_path.join("docker/nginx.conf"), generate_nginx_conf())
                .map_err(|e| format!("Failed to write nginx.conf: {}", e))?;
        }
    }

    // Generate basic .env file
    let env_content = generate_env_file(project_name, &template.blocks);
    fs::write(project_path.join(".env"), &env_content)
        .map_err(|e| format!("Failed to write .env: {}", e))?;

    // Generate Makefile
    let makefile = generate_makefile(project_name);
    fs::write(project_path.join("Makefile"), &makefile)
        .map_err(|e| format!("Failed to write Makefile: {}", e))?;

    // Generate mysql.cnf if MySQL/MariaDB is enabled
    let has_mysql = template.blocks.iter().any(|b| (b.block_id == "mysql" || b.block_id == "mariadb") && b.enabled);
    if has_mysql {
        fs::write(project_path.join("docker/mysql.cnf"), generate_mysql_cnf())
            .map_err(|e| format!("Failed to write mysql.cnf: {}", e))?;
    }

    // Create project config
    let config = create_project_config_from_blocks(&template.blocks);
    let now = Utc::now();

    let project = Project {
        id: project_id,
        name: project_name.to_string(),
        template: format!("custom:{}", template.name),
        path: project_path.to_string_lossy().to_string(),
        created_at: now,
        updated_at: now,
        config,
        status: ProjectStatus::Stopped,
    };

    // Save project metadata
    save_project(&project)?;

    Ok(project)
}

fn generate_php_dockerfile(php_version: &str) -> String {
    format!(r#"FROM php:{}-fpm

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    libpng-dev \
    libonig-dev \
    libxml2-dev \
    zip \
    unzip \
    libzip-dev \
    libpq-dev \
    supervisor \
    cron

# Clear cache
RUN apt-get clean && rm -rf /var/lib/apt/lists/*

# Install PHP extensions
RUN docker-php-ext-install pdo pdo_mysql pdo_pgsql mbstring exif pcntl bcmath gd zip

# Install Redis extension
RUN pecl install redis && docker-php-ext-enable redis

# Install Composer
COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

# Install Node.js 20
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g npm@latest

# Set working directory
WORKDIR /var/www/html

# Copy custom PHP configuration
COPY docker/php.ini /usr/local/etc/php/conf.d/custom.ini

EXPOSE 9000

CMD ["php-fpm"]
"#, php_version)
}

fn generate_php_ini() -> &'static str {
    r#"upload_max_filesize = 64M
post_max_size = 64M
memory_limit = 256M
max_execution_time = 30
expose_php = Off

[opcache]
opcache.enable=1
opcache.memory_consumption=128
opcache.interned_strings_buffer=8
opcache.max_accelerated_files=10000
opcache.validate_timestamps=1
opcache.revalidate_freq=2
"#
}

fn generate_nginx_conf() -> &'static str {
    r#"server {
    listen 80;
    index index.php index.html;
    error_log  /var/log/nginx/error.log;
    access_log /var/log/nginx/access.log;
    root /var/www/html/public;

    location / {
        try_files $uri $uri/ /index.php?$query_string;
    }

    location ~ \.php$ {
        fastcgi_split_path_info ^(.+\.php)(/.+)$;
        fastcgi_pass app:9000;
        fastcgi_index index.php;
        include fastcgi_params;
        fastcgi_param SCRIPT_FILENAME $document_root$fastcgi_script_name;
        fastcgi_param PATH_INFO $fastcgi_path_info;
        fastcgi_buffering off;
    }

    location ~ /\.ht {
        deny all;
    }

    client_max_body_size 64M;
}
"#
}

fn generate_mysql_cnf() -> &'static str {
    r#"[mysqld]
general_log = 1
general_log_file = /var/lib/mysql/general.log
character-set-server = utf8mb4
collation-server = utf8mb4_unicode_ci
max_allowed_packet = 64M
sql_mode = STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION

[client]
default-character-set = utf8mb4
"#
}

fn generate_env_file(project_name: &str, blocks: &[BlockInstance]) -> String {
    let mut env = format!(r#"APP_NAME={}
APP_ENV=local
APP_DEBUG=true
APP_URL=http://localhost

LOG_CHANNEL=stack

"#, project_name);

    // Database config
    if let Some(block) = blocks.iter().find(|b| b.block_id == "mysql" && b.enabled) {
        let database = get_config_str(&block.config, "database", "laravel");
        let username = get_config_str(&block.config, "username", "laravel");
        let password = get_config_str(&block.config, "password", "secret");
        let port = get_config_int(&block.config, "port", 3306);

        env.push_str(&format!(r#"DB_CONNECTION=mysql
DB_HOST=db
DB_PORT={}
DB_DATABASE={}
DB_USERNAME={}
DB_PASSWORD={}

"#, port, database, username, password));
    } else if let Some(block) = blocks.iter().find(|b| b.block_id == "postgresql" && b.enabled) {
        let database = get_config_str(&block.config, "database", "laravel");
        let username = get_config_str(&block.config, "username", "laravel");
        let password = get_config_str(&block.config, "password", "secret");
        let port = get_config_int(&block.config, "port", 5432);

        env.push_str(&format!(r#"DB_CONNECTION=pgsql
DB_HOST=db
DB_PORT={}
DB_DATABASE={}
DB_USERNAME={}
DB_PASSWORD={}

"#, port, database, username, password));
    } else if let Some(block) = blocks.iter().find(|b| b.block_id == "mariadb" && b.enabled) {
        let database = get_config_str(&block.config, "database", "laravel");
        let username = get_config_str(&block.config, "username", "laravel");
        let password = get_config_str(&block.config, "password", "secret");
        let port = get_config_int(&block.config, "port", 3306);

        env.push_str(&format!(r#"DB_CONNECTION=mysql
DB_HOST=db
DB_PORT={}
DB_DATABASE={}
DB_USERNAME={}
DB_PASSWORD={}

"#, port, database, username, password));
    }

    // Redis config
    if let Some(block) = blocks.iter().find(|b| b.block_id == "redis" && b.enabled) {
        let port = get_config_int(&block.config, "port", 6379);
        env.push_str(&format!(r#"REDIS_HOST=redis
REDIS_PASSWORD=null
REDIS_PORT={}

CACHE_DRIVER=redis
SESSION_DRIVER=redis
QUEUE_CONNECTION=redis

"#, port));
    }

    // Mail config
    if blocks.iter().any(|b| b.block_id == "mailhog" && b.enabled) {
        env.push_str(r#"MAIL_MAILER=smtp
MAIL_HOST=mailhog
MAIL_PORT=1025
MAIL_USERNAME=null
MAIL_PASSWORD=null
MAIL_ENCRYPTION=null

"#);
    } else if blocks.iter().any(|b| b.block_id == "mailpit" && b.enabled) {
        env.push_str(r#"MAIL_MAILER=smtp
MAIL_HOST=mailpit
MAIL_PORT=1025
MAIL_USERNAME=null
MAIL_PASSWORD=null
MAIL_ENCRYPTION=null

"#);
    }

    // Meilisearch config
    if let Some(block) = blocks.iter().find(|b| b.block_id == "meilisearch" && b.enabled) {
        let port = get_config_int(&block.config, "port", 7700);
        let master_key = get_config_str(&block.config, "master_key", "masterKey");
        env.push_str(&format!(r#"SCOUT_DRIVER=meilisearch
MEILISEARCH_HOST=http://meilisearch:{}
MEILISEARCH_KEY={}

"#, port, master_key));
    }

    // Soketi/WebSocket config
    if let Some(block) = blocks.iter().find(|b| b.block_id == "soketi" && b.enabled) {
        let app_id = get_config_str(&block.config, "app_id", "app-id");
        let app_key = get_config_str(&block.config, "app_key", "app-key");
        let app_secret = get_config_str(&block.config, "app_secret", "app-secret");
        let port = get_config_int(&block.config, "port", 6001);

        env.push_str(&format!(r#"BROADCAST_DRIVER=pusher
PUSHER_APP_ID={}
PUSHER_APP_KEY={}
PUSHER_APP_SECRET={}
PUSHER_HOST=soketi
PUSHER_PORT={}
PUSHER_SCHEME=http

"#, app_id, app_key, app_secret, port));
    }

    // MinIO config
    if let Some(block) = blocks.iter().find(|b| b.block_id == "minio" && b.enabled) {
        let port = get_config_int(&block.config, "port", 9000);
        let root_user = get_config_str(&block.config, "root_user", "minioadmin");
        let root_password = get_config_str(&block.config, "root_password", "minioadmin");
        let bucket = get_config_str(&block.config, "default_bucket", "laravel");

        env.push_str(&format!(r#"FILESYSTEM_DISK=s3
AWS_ACCESS_KEY_ID={}
AWS_SECRET_ACCESS_KEY={}
AWS_DEFAULT_REGION=us-east-1
AWS_BUCKET={}
AWS_ENDPOINT=http://minio:{}
AWS_USE_PATH_STYLE_ENDPOINT=true

"#, root_user, root_password, bucket, port));
    }

    env
}

fn generate_makefile(project_name: &str) -> String {
    format!(r#".PHONY: up down build rebuild shell logs

up:
	docker compose up -d

down:
	docker compose down

build:
	docker compose build

rebuild:
	docker compose down
	docker compose build --no-cache
	docker compose up -d

shell:
	docker compose exec app bash

logs:
	docker compose logs -f

# Laravel specific
artisan:
	docker compose exec app php artisan $(filter-out $@,$(MAKECMDGOALS))

composer:
	docker compose exec app composer $(filter-out $@,$(MAKECMDGOALS))

npm:
	docker compose exec app npm $(filter-out $@,$(MAKECMDGOALS))

migrate:
	docker compose exec app php artisan migrate

fresh:
	docker compose exec app php artisan migrate:fresh --seed

tinker:
	docker compose exec app php artisan tinker

test:
	docker compose exec app php artisan test

# Catch all for passing arguments
%:
	@:
"#)
}

fn create_project_config_from_blocks(blocks: &[BlockInstance]) -> ProjectConfig {
    let php_version = blocks.iter()
        .find(|b| b.block_id == "php-fpm" && b.enabled)
        .and_then(|b| b.version.clone());

    let node_version = blocks.iter()
        .find(|b| b.block_id == "nodejs" && b.enabled)
        .and_then(|b| b.version.clone())
        .unwrap_or_else(|| "20".to_string());

    // Get ports from configs
    let app_port = blocks.iter()
        .find(|b| b.block_id == "nginx" && b.enabled)
        .map(|b| get_config_int(&b.config, "port", 80) as u16)
        .unwrap_or(80);

    let db_port = blocks.iter()
        .find(|b| (b.block_id == "mysql" || b.block_id == "mariadb" || b.block_id == "postgresql") && b.enabled)
        .map(|b| get_config_int(&b.config, "port", 3306) as u16)
        .unwrap_or(3306);

    let redis_port = blocks.iter()
        .find(|b| b.block_id == "redis" && b.enabled)
        .map(|b| get_config_int(&b.config, "port", 6379) as u16)
        .unwrap_or(6379);

    let phpmyadmin_port = blocks.iter()
        .find(|b| b.block_id == "phpmyadmin" && b.enabled)
        .map(|b| get_config_int(&b.config, "port", 8080) as u16)
        .unwrap_or(8080);

    let mailhog_port = blocks.iter()
        .find(|b| (b.block_id == "mailhog" || b.block_id == "mailpit") && b.enabled)
        .map(|b| get_config_int(&b.config, "ui_port", 8025) as u16)
        .unwrap_or(8025);

    ProjectConfig {
        php_version,
        node_version,
        install_bun: false,
        install_pnpm: false,
        install_yarn: false,
        install_laravel: true,
        ports: PortConfig {
            app: app_port,
            vite: 5173,
            db: db_port,
            redis: redis_port,
            phpmyadmin: phpmyadmin_port,
            mailhog: mailhog_port,
        },
        services: ServiceConfig {
            mysql: blocks.iter().any(|b| b.block_id == "mysql" && b.enabled),
            redis: blocks.iter().any(|b| b.block_id == "redis" && b.enabled),
            mailhog: blocks.iter().any(|b| (b.block_id == "mailhog" || b.block_id == "mailpit") && b.enabled),
            phpmyadmin: blocks.iter().any(|b| b.block_id == "phpmyadmin" && b.enabled),
            nginx: blocks.iter().any(|b| b.block_id == "nginx" && b.enabled),
        },
    }
}
