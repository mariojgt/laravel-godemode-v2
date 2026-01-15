use crate::docker::DockerManager;
use crate::project::ProjectManager;
use crate::state::{AppState, Project, ProjectConfig, ProjectStatus, Settings, ServiceStatus, SupervisorStatus};
use crate::template::{create_project_from_template, save_project};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub template: String,
    pub config: ProjectConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub icon: String,
    pub template_type: String,
    pub category: String,
}

// ============ Project Commands ============

#[tauri::command]
pub fn get_projects() -> Result<Vec<Project>, String> {
    let projects = ProjectManager::load_all_projects()?;
    Ok(projects.into_values().collect())
}

#[tauri::command]
pub fn create_project(request: CreateProjectRequest) -> Result<Project, String> {
    println!("[GodMode] Creating project: {}", request.name);
    println!("[GodMode] Template: {}", request.template);
    
    let projects_dir = ProjectManager::ensure_projects_dir()?;
    println!("[GodMode] Projects dir: {}", projects_dir);
    
    let templates_path = get_templates_path();
    println!("[GodMode] Templates path: {}", templates_path);

    let project = create_project_from_template(
        &request.name,
        &request.template,
        &projects_dir,
        &templates_path,
        request.config,
    )?;
    
    println!("[GodMode] Project created at: {}", project.path);

    save_project(&project)?;
    println!("[GodMode] Project saved!");
    
    Ok(project)
}

#[tauri::command]
pub fn delete_project(project_id: String, delete_files: bool) -> Result<(), String> {
    ProjectManager::delete_project(&project_id, delete_files)
}

#[tauri::command]
pub fn get_project(project_id: String) -> Result<Project, String> {
    ProjectManager::get_project(&project_id)
}

#[tauri::command]
pub fn get_project_env(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    ProjectManager::get_env_file(&project.path)
}

#[tauri::command]
pub fn update_project_env(project_id: String, env_content: String) -> Result<(), String> {
    let project = ProjectManager::get_project(&project_id)?;
    ProjectManager::update_env_file(&project.path, &env_content)
}

#[tauri::command]
pub fn open_project_folder(project_id: String) -> Result<(), String> {
    let project = ProjectManager::get_project(&project_id)?;

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&project.path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(&project.path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&project.path)
            .spawn()
            .map_err(|e| format!("Failed to open folder: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub fn open_project_in_editor(project_id: String, editor: String) -> Result<(), String> {
    let project = ProjectManager::get_project(&project_id)?;

    let editor_cmd = match editor.as_str() {
        "vscode" | "code" => "code",
        "cursor" => "cursor",
        "phpstorm" => "phpstorm",
        "sublime" => "subl",
        _ => "code",
    };

    Command::new(editor_cmd)
        .arg(&project.path)
        .spawn()
        .map_err(|e| format!("Failed to open in editor: {}", e))?;

    Ok(())
}

// ============ Docker Commands ============

#[tauri::command]
pub fn start_project(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    ProjectManager::update_project_status(&project_id, ProjectStatus::Starting)?;

    let result = DockerManager::start_project(&project.path);

    if result.is_ok() {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Running)?;
    } else {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Error)?;
    }

    result
}

#[tauri::command]
pub fn stop_project(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    ProjectManager::update_project_status(&project_id, ProjectStatus::Stopping)?;

    let result = DockerManager::stop_project(&project.path);

    if result.is_ok() {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Stopped)?;
    }

    result
}

#[tauri::command]
pub fn restart_project(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::restart_project(&project.path)
}

#[tauri::command]
pub fn rebuild_project(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    ProjectManager::update_project_status(&project_id, ProjectStatus::Building)?;

    let result = DockerManager::rebuild_project(&project.path);

    if result.is_ok() {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Running)?;
    } else {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Error)?;
    }

    result
}

// Streaming versions for real-time output - run in background thread
#[tauri::command]
pub async fn start_project_streaming(app: tauri::AppHandle, project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    ProjectManager::update_project_status(&project_id, ProjectStatus::Starting)?;

    let app_clone = app.clone();
    let project_id_clone = project_id.clone();
    let project_path = project.path.clone();
    let project_name = project.name.clone();
    let install_laravel = project.config.install_laravel;

    // Run docker command in a background thread
    let result = tokio::task::spawn_blocking(move || {
        DockerManager::start_project_streaming(&app_clone, &project_id_clone, &project_path)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?;

    if result.is_ok() {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Running)?;
        
        // If install_laravel is enabled, check if src folder is empty and install Laravel
        if install_laravel {
            let src_path = std::path::Path::new(&project.path).join("src");
            let is_empty = src_path.read_dir().map(|mut d| d.next().is_none()).unwrap_or(true);
            
            if is_empty {
                let app_clone2 = app.clone();
                let project_id_clone2 = project_id.clone();
                let project_path2 = project.path.clone();
                let project_name2 = project_name.clone();
                
                // Run Laravel installation in background
                tokio::task::spawn_blocking(move || {
                    DockerManager::install_laravel_streaming(&app_clone2, &project_id_clone2, &project_path2, &project_name2)
                })
                .await
                .map_err(|e| format!("Laravel install failed: {}", e))??;
            }
        }
    } else {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Error)?;
    }

    result
}

#[tauri::command]
pub async fn stop_project_streaming(app: tauri::AppHandle, project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    ProjectManager::update_project_status(&project_id, ProjectStatus::Stopping)?;

    let app_clone = app.clone();
    let project_id_clone = project_id.clone();
    let project_path = project.path.clone();

    // Run docker command in a background thread
    let result = tokio::task::spawn_blocking(move || {
        DockerManager::stop_project_streaming(&app_clone, &project_id_clone, &project_path)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?;

    if result.is_ok() {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Stopped)?;
    }

    result
}

#[tauri::command]
pub async fn rebuild_project_streaming(app: tauri::AppHandle, project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    ProjectManager::update_project_status(&project_id, ProjectStatus::Building)?;

    let app_clone = app.clone();
    let project_id_clone = project_id.clone();
    let project_path = project.path.clone();

    // Run docker command in a background thread
    let result = tokio::task::spawn_blocking(move || {
        DockerManager::rebuild_project_streaming(&app_clone, &project_id_clone, &project_path)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?;

    if result.is_ok() {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Running)?;
    } else {
        ProjectManager::update_project_status(&project_id, ProjectStatus::Error)?;
    }

    result
}

#[tauri::command]
pub async fn install_laravel_streaming(app: tauri::AppHandle, project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;

    let app_clone = app.clone();
    let project_id_clone = project_id.clone();
    let project_path = project.path.clone();
    let project_name = project.name.clone();

    // Run composer install in a background thread
    let result = tokio::task::spawn_blocking(move || {
        DockerManager::install_laravel_streaming(&app_clone, &project_id_clone, &project_path, &project_name)
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?;

    result
}

#[tauri::command]
pub fn get_project_status(project_id: String) -> Result<Vec<ServiceStatus>, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::get_project_status(&project.path, &project.name)
}

#[tauri::command]
pub fn get_container_logs(project_id: String, service: String, lines: u32) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::get_container_logs(&project.path, &service, lines)
}

#[tauri::command]
pub fn get_services_status(project_id: String) -> Result<Vec<ServiceStatus>, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::get_project_status(&project.path, &project.name)
}

// ============ Template Commands ============

#[tauri::command]
pub fn get_templates() -> Result<Vec<TemplateInfo>, String> {
    let templates_path = get_templates_path();
    let mut templates = Vec::new();

    let entries = fs::read_dir(&templates_path)
        .map_err(|e| format!("Failed to read templates directory: {}", e))?;

    for entry in entries.flatten() {
        if entry.path().is_dir() {
            let config_path = entry.path().join("config.json");
            if config_path.exists() {
                if let Ok(content) = fs::read_to_string(&config_path) {
                    if let Ok(config) = serde_json::from_str::<serde_json::Value>(&content) {
                        templates.push(TemplateInfo {
                            name: config["name"].as_str().unwrap_or("").to_string(),
                            description: config["description"].as_str().unwrap_or("").to_string(),
                            icon: config["icon"].as_str().unwrap_or("📦").to_string(),
                            template_type: config["type"].as_str().unwrap_or("").to_string(),
                            category: config["category"].as_str().unwrap_or("").to_string(),
                        });
                    }
                }
            }
        }
    }

    Ok(templates)
}

#[tauri::command]
pub fn get_template(template_type: String) -> Result<serde_json::Value, String> {
    let templates_path = get_templates_path();
    let config_path = Path::new(&templates_path).join(&template_type).join("config.json");

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read template config: {}", e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse template config: {}", e))
}

// ============ Artisan Commands ============

#[tauri::command]
pub fn run_artisan_command(project_id: String, command: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::run_artisan(&project.path, &command)
}

#[tauri::command]
pub fn run_make_command(project_id: String, target: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::run_make(&project.path, &target)
}

// ============ Queue Commands ============

#[tauri::command]
pub fn start_queue_worker(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::exec_in_container(&project.path, "app", "supervisorctl start laravel-queue")
}

#[tauri::command]
pub fn stop_queue_worker(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::exec_in_container(&project.path, "app", "supervisorctl stop laravel-queue")
}

#[tauri::command]
pub fn get_queue_status(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::run_artisan(&project.path, "queue:monitor")
}

// ============ Cache Commands ============

#[tauri::command]
pub fn clear_cache(project_id: String, cache_type: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;

    let command = match cache_type.as_str() {
        "all" => "optimize:clear",
        "config" => "config:clear",
        "route" => "route:clear",
        "view" => "view:clear",
        "cache" => "cache:clear",
        "compiled" => "clear-compiled",
        _ => return Err("Invalid cache type".to_string()),
    };

    DockerManager::run_artisan(&project.path, command)
}

#[tauri::command]
pub fn optimize_app(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::run_artisan(&project.path, "optimize")
}

// ============ Database Commands ============

#[tauri::command]
pub fn run_migrations(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::run_artisan(&project.path, "migrate --force")
}

#[tauri::command]
pub fn run_seeders(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::run_artisan(&project.path, "db:seed --force")
}

#[tauri::command]
pub fn fresh_database(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::run_artisan(&project.path, "migrate:fresh --seed --force")
}

// ============ Supervisor Commands ============

#[tauri::command]
pub fn get_supervisor_status(project_id: String) -> Result<SupervisorStatus, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::get_supervisor_status(&project.path)
}

#[tauri::command]
pub fn reload_supervisor(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::reload_supervisor(&project.path)
}

#[tauri::command]
pub fn restart_supervisor(project_id: String) -> Result<String, String> {
    let project = ProjectManager::get_project(&project_id)?;
    DockerManager::restart_supervisor(&project.path)
}

// ============ Settings Commands ============

#[tauri::command]
pub fn get_settings() -> Result<Settings, String> {
    let settings_path = get_settings_path();

    if Path::new(&settings_path).exists() {
        let content = fs::read_to_string(&settings_path)
            .map_err(|e| format!("Failed to read settings: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse settings: {}", e))
    } else {
        Ok(Settings::default())
    }
}

#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<(), String> {
    let settings_path = get_settings_path();

    if let Some(parent) = Path::new(&settings_path).parent() {
        fs::create_dir_all(parent).ok();
    }

    let content = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    fs::write(&settings_path, content)
        .map_err(|e| format!("Failed to save settings: {}", e))
}

// ============ System Commands ============

#[tauri::command]
pub fn check_docker_installed() -> bool {
    DockerManager::is_docker_installed()
}

#[tauri::command]
pub fn get_docker_version() -> Result<String, String> {
    DockerManager::get_docker_version()
}

// ============ Helper Functions ============

fn get_templates_path() -> String {
    // First check if running in development - templates are in parent directory
    if let Ok(cwd) = std::env::current_dir() {
        // Check current directory (for when running from project root)
        let dev_path = cwd.join("templates");
        if dev_path.exists() {
            println!("[GodMode] Found templates at: {:?}", dev_path);
            return dev_path.to_string_lossy().to_string();
        }
        
        // Check parent directory (for when running from src-tauri)
        let parent_path = cwd.parent().map(|p| p.join("templates"));
        if let Some(ref path) = parent_path {
            if path.exists() {
                println!("[GodMode] Found templates at parent: {:?}", path);
                return path.to_string_lossy().to_string();
            }
        }
    }

    // Then check resource path
    let home = dirs::home_dir().unwrap_or_default();
    let fallback = home.join(".laravel-godmode")
        .join("templates");
    println!("[GodMode] Using fallback templates path: {:?}", fallback);
    fallback.to_string_lossy().to_string()
}

fn get_settings_path() -> String {
    let home = dirs::home_dir().unwrap_or_default();
    home.join(".laravel-godmode")
        .join("settings.json")
        .to_string_lossy()
        .to_string()
}
