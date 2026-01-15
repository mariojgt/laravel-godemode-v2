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
}
