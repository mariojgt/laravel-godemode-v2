use crate::state::{Project, ProjectConfig, ProjectStatus, Settings};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;
use chrono::Utc;

pub struct ProjectManager;

impl ProjectManager {
    fn get_settings_path() -> String {
        let home = dirs::home_dir().unwrap_or_default();
        home.join(".laravel-godmode")
            .join("settings.json")
            .to_string_lossy()
            .to_string()
    }

    pub fn get_projects_dir() -> String {
        // Try to read from settings first
        let settings_path = Self::get_settings_path();
        if Path::new(&settings_path).exists() {
            if let Ok(content) = fs::read_to_string(&settings_path) {
                if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                    if !settings.projects_path.is_empty() {
                        println!("[ProjectManager] Using projects path from settings: {}", settings.projects_path);
                        return settings.projects_path;
                    }
                }
            }
        }
        
        // Fallback to default
        let home = dirs::home_dir().unwrap_or_default();
        let default_path = home.join("Documents")
            .join("laravel-godmode")
            .join("projects")
            .to_string_lossy()
            .to_string();
        println!("[ProjectManager] Using default projects path: {}", default_path);
        default_path
    }

    pub fn ensure_projects_dir() -> Result<String, String> {
        let projects_dir = Self::get_projects_dir();
        fs::create_dir_all(&projects_dir)
            .map_err(|e| format!("Failed to create projects directory: {}", e))?;
        Ok(projects_dir)
    }

    pub fn load_all_projects() -> Result<HashMap<String, Project>, String> {
        let projects_dir = Self::ensure_projects_dir()?;
        let mut projects = HashMap::new();

        let entries = fs::read_dir(&projects_dir)
            .map_err(|e| format!("Failed to read projects directory: {}", e))?;

        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let config_path = entry.path().join(".godmode.json");
                if config_path.exists() {
                    if let Ok(content) = fs::read_to_string(&config_path) {
                        if let Ok(project) = serde_json::from_str::<Project>(&content) {
                            projects.insert(project.id.clone(), project);
                        }
                    }
                }
            }
        }

        Ok(projects)
    }

    pub fn get_project(project_id: &str) -> Result<Project, String> {
        let projects = Self::load_all_projects()?;
        projects.get(project_id)
            .cloned()
            .ok_or_else(|| "Project not found".to_string())
    }

    pub fn save_project(project: &Project) -> Result<(), String> {
        let config_path = Path::new(&project.path).join(".godmode.json");
        let content = serde_json::to_string_pretty(project)
            .map_err(|e| format!("Failed to serialize project: {}", e))?;
        fs::write(&config_path, content)
            .map_err(|e| format!("Failed to save project: {}", e))
    }

    pub fn delete_project(project_id: &str, delete_files: bool) -> Result<(), String> {
        let project = Self::get_project(project_id)?;

        if delete_files {
            // Stop containers first
            let _ = std::process::Command::new("docker-compose")
                .args(["down", "-v"])
                .current_dir(&project.path)
                .output();

            // Delete project directory
            fs::remove_dir_all(&project.path)
                .map_err(|e| format!("Failed to delete project files: {}", e))?;
        } else {
            // Just remove the .godmode.json file
            let config_path = Path::new(&project.path).join(".godmode.json");
            fs::remove_file(&config_path)
                .map_err(|e| format!("Failed to remove project config: {}", e))?;
        }

        Ok(())
    }

    pub fn update_project_status(project_id: &str, status: ProjectStatus) -> Result<(), String> {
        let mut project = Self::get_project(project_id)?;
        project.status = status;
        project.updated_at = Utc::now();
        Self::save_project(&project)
    }

    pub fn update_env_file(project_path: &str, env_content: &str) -> Result<(), String> {
        let env_path = Path::new(project_path).join(".env");
        fs::write(&env_path, env_content)
            .map_err(|e| format!("Failed to update .env file: {}", e))
    }

    pub fn get_env_file(project_path: &str) -> Result<String, String> {
        let env_path = Path::new(project_path).join(".env");
        fs::read_to_string(&env_path)
            .map_err(|e| format!("Failed to read .env file: {}", e))
    }

    pub fn clone_project(source_project_id: &str, new_name: &str) -> Result<Project, String> {
        let source_project = Self::get_project(source_project_id)?;
        let projects_dir = Self::ensure_projects_dir()?;
        
        // Create new project directory
        let new_project_path = Path::new(&projects_dir).join(&new_name);
        if new_project_path.exists() {
            return Err(format!("Project '{}' already exists", new_name));
        }
        
        // Copy all files from source project
        Self::copy_dir_recursive(&source_project.path, new_project_path.to_str().unwrap())?;
        
        // Create new project with new ID
        let new_project = Project {
            id: Uuid::new_v4().to_string(),
            name: new_name.to_string(),
            path: new_project_path.to_string_lossy().to_string(),
            template: source_project.template.clone(),
            config: source_project.config.clone(),
            status: ProjectStatus::Stopped,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Update docker-compose and other files with new project name
        Self::update_cloned_project_files(&new_project)?;
        
        // Save the new project config
        Self::save_project(&new_project)?;
        
        Ok(new_project)
    }
    
    fn copy_dir_recursive(src: &str, dst: &str) -> Result<(), String> {
        let src_path = Path::new(src);
        let dst_path = Path::new(dst);
        
        fs::create_dir_all(dst_path)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
        
        for entry in fs::read_dir(src_path).map_err(|e| format!("Failed to read dir: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            let file_name = path.file_name().unwrap();
            let dst_file = dst_path.join(file_name);
            
            // Skip .godmode.json as we'll create a new one
            if file_name == ".godmode.json" {
                continue;
            }
            
            if path.is_dir() {
                Self::copy_dir_recursive(path.to_str().unwrap(), dst_file.to_str().unwrap())?;
            } else {
                fs::copy(&path, &dst_file)
                    .map_err(|e| format!("Failed to copy file: {}", e))?;
            }
        }
        
        Ok(())
    }
    
    fn update_cloned_project_files(project: &Project) -> Result<(), String> {
        // Update docker-compose.yml with new container names
        let compose_path = Path::new(&project.path).join("docker-compose.yml");
        if compose_path.exists() {
            let content = fs::read_to_string(&compose_path)
                .map_err(|e| format!("Failed to read docker-compose.yml: {}", e))?;
            
            // Replace container_name references with new project name
            // This is a simple approach - for more complex setups, might need YAML parsing
            let updated = content.replace(
                &format!("container_name: "),
                &format!("# Original container names replaced\n      container_name: {}_", project.name)
            );
            
            fs::write(&compose_path, updated)
                .map_err(|e| format!("Failed to update docker-compose.yml: {}", e))?;
        }
        
        // Update .env file
        let env_path = Path::new(&project.path).join(".env");
        if env_path.exists() {
            let content = fs::read_to_string(&env_path)
                .map_err(|e| format!("Failed to read .env: {}", e))?;
            
            // Update APP_NAME
            let mut updated = content.clone();
            for line in content.lines() {
                if line.starts_with("APP_NAME=") {
                    updated = updated.replace(line, &format!("APP_NAME={}", project.name));
                }
            }
            
            fs::write(&env_path, updated)
                .map_err(|e| format!("Failed to update .env: {}", e))?;
        }
        
        Ok(())
    }

    pub fn import_project(source_path: &str, name: &str) -> Result<Project, String> {
        let source = Path::new(source_path);
        
        // Verify it's a Laravel project
        let artisan_path = source.join("artisan");
        let composer_path = source.join("composer.json");
        
        if !artisan_path.exists() && !composer_path.exists() {
            return Err("This doesn't appear to be a Laravel project (no artisan or composer.json found)".to_string());
        }
        
        let projects_dir = Self::ensure_projects_dir()?;
        let new_project_path = Path::new(&projects_dir).join(name);
        
        if new_project_path.exists() {
            return Err(format!("Project '{}' already exists", name));
        }
        
        // Copy the project
        Self::copy_dir_recursive(source_path, new_project_path.to_str().unwrap())?;
        
        // Create default project config
        let config = ProjectConfig {
            php_version: Some("8.3".to_string()),
            node_version: "20".to_string(),
            install_bun: false,
            install_pnpm: false,
            install_yarn: false,
            install_laravel: false,
            ports: crate::state::PortConfig {
                app: 8080,
                vite: 5173,
                db: 3306,
                phpmyadmin: 8081,
                mailhog: 8025,
                redis: 6379,
            },
            services: crate::state::ServiceConfig {
                mysql: true,
                redis: true,
                mailhog: true,
                phpmyadmin: true,
                nginx: true,
            },
        };
        
        let project = Project {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            path: new_project_path.to_string_lossy().to_string(),
            template: "laravel".to_string(),
            config,
            status: ProjectStatus::Stopped,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Save project config
        Self::save_project(&project)?;
        
        Ok(project)
    }
}
