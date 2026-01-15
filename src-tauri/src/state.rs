use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub template: String,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub config: ProjectConfig,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub php_version: Option<String>,
    pub node_version: String,
    pub install_bun: bool,
    pub install_pnpm: bool,
    pub install_yarn: bool,
    #[serde(default)]
    pub install_laravel: bool,
    pub services: ServiceConfig,
    pub ports: PortConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub mysql: bool,
    pub redis: bool,
    pub phpmyadmin: bool,
    pub mailhog: bool,
    pub nginx: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {
    pub app: u16,
    pub vite: u16,
    pub db: u16,
    pub redis: u16,
    pub phpmyadmin: u16,
    pub mailhog: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Running,
    Stopped,
    Error,
    Building,
    Starting,
    Stopping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: String,
    pub container_id: Option<String>,
    pub ports: Vec<String>,
    pub health: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorProgram {
    pub name: String,
    pub status: String,
    pub pid: Option<i32>,
    pub uptime: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorStatus {
    pub total_programs: i32,
    pub running: i32,
    pub stopped: i32,
    pub failed: i32,
    pub programs: Vec<SupervisorProgram>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub projects_path: String,
    pub auto_start_projects: bool,
    pub preferred_editor: String,
    pub default_php_version: String,
    pub default_node_version: String,
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_default();
        Self {
            projects_path: home.join("Documents").join("laravel-godmode").join("projects").to_string_lossy().to_string(),
            auto_start_projects: false,
            preferred_editor: "code".to_string(),
            default_php_version: "8.4".to_string(),
            default_node_version: "18".to_string(),
            theme: "dark".to_string(),
        }
    }
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            php_version: Some("8.4".to_string()),
            node_version: "18".to_string(),
            install_bun: true,
            install_pnpm: false,
            install_yarn: false,
            install_laravel: true,
            services: ServiceConfig::default(),
            ports: PortConfig::default(),
        }
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            mysql: true,
            redis: true,
            phpmyadmin: false,
            mailhog: false,
            nginx: true,
        }
    }
}

impl Default for PortConfig {
    fn default() -> Self {
        Self {
            app: 8000,
            vite: 5173,
            db: 3306,
            redis: 6379,
            phpmyadmin: 8080,
            mailhog: 8025,
        }
    }
}

pub struct AppState {
    pub projects: HashMap<String, Project>,
    pub settings: Settings,
    pub templates_path: String,
}

impl AppState {
    pub fn new() -> Self {
        let templates_path = std::env::current_dir()
            .unwrap_or_default()
            .join("templates")
            .to_string_lossy()
            .to_string();

        Self {
            projects: HashMap::new(),
            settings: Settings::default(),
            templates_path,
        }
    }
}
