use crate::state::{ServiceStatus, SupervisorStatus, SupervisorProgram};
use std::process::{Command, Stdio};
use std::path::Path;
use std::io::{BufRead, BufReader};
use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct DockerOutputEvent {
    pub project_id: String,
    pub line: String,
    pub stream_type: String, // "stdout", "stderr", "status"
}

pub struct DockerManager;

impl DockerManager {
    pub fn is_docker_installed() -> bool {
        Command::new("docker")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn get_docker_version() -> Result<String, String> {
        let output = Command::new("docker")
            .arg("--version")
            .output()
            .map_err(|e| format!("Failed to get Docker version: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err("Docker not installed or not running".to_string())
        }
    }

    pub fn start_project(project_path: &str) -> Result<String, String> {
        let path = Path::new(project_path);

        let output = Command::new("docker-compose")
            .args(["up", "-d"])
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to start project: {}", e))?;

        if output.status.success() {
            Ok("Project started successfully".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn stop_project(project_path: &str) -> Result<String, String> {
        let path = Path::new(project_path);

        let output = Command::new("docker-compose")
            .args(["down"])
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to stop project: {}", e))?;

        if output.status.success() {
            Ok("Project stopped successfully".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn restart_project(project_path: &str) -> Result<String, String> {
        let path = Path::new(project_path);

        let output = Command::new("docker-compose")
            .args(["restart"])
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to restart project: {}", e))?;

        if output.status.success() {
            Ok("Project restarted successfully".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn rebuild_project(project_path: &str) -> Result<String, String> {
        let path = Path::new(project_path);

        // First stop
        let _ = Command::new("docker-compose")
            .args(["down"])
            .current_dir(path)
            .output();

        // Rebuild
        let output = Command::new("docker-compose")
            .args(["build", "--no-cache"])
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to rebuild project: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        // Start again
        let output = Command::new("docker-compose")
            .args(["up", "-d"])
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to start rebuilt project: {}", e))?;

        if output.status.success() {
            Ok("Project rebuilt and started successfully".to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    // Streaming versions for real-time output
    pub fn start_project_streaming(app: &AppHandle, project_id: &str, project_path: &str) -> Result<String, String> {
        Self::run_docker_compose_streaming(app, project_id, project_path, &["up", "-d", "--build"])
    }

    pub fn stop_project_streaming(app: &AppHandle, project_id: &str, project_path: &str) -> Result<String, String> {
        Self::run_docker_compose_streaming(app, project_id, project_path, &["down"])
    }

    pub fn rebuild_project_streaming(app: &AppHandle, project_id: &str, project_path: &str) -> Result<String, String> {
        let path = Path::new(project_path);

        // First stop
        Self::emit_output(app, project_id, "Stopping existing containers...", "status");
        let _ = Command::new("docker-compose")
            .args(["down"])
            .current_dir(path)
            .output();

        // Rebuild with streaming
        Self::emit_output(app, project_id, "Building containers (this may take a while)...", "status");
        Self::run_docker_compose_streaming(app, project_id, project_path, &["up", "-d", "--build", "--force-recreate"])
    }

    pub fn install_laravel_streaming(app: &AppHandle, project_id: &str, project_path: &str, project_name: &str) -> Result<String, String> {
        let path = Path::new(project_path);

        Self::emit_output(app, project_id, "🚀 Installing fresh Laravel application...", "status");
        Self::emit_output(app, project_id, "This may take a few minutes...", "status");

        // Run composer create-project inside the app container
        let container_name = format!("{}_app", project_name);

        Self::emit_output(app, project_id, &format!("Running: composer create-project laravel/laravel . in container {}", container_name), "status");

        let mut child = Command::new("docker")
            .args([
                "exec", "-w", "/var/www/html", &container_name,
                "composer", "create-project", "laravel/laravel", ".", "--prefer-dist", "--no-interaction"
            ])
            .current_dir(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to run composer: {}", e))?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        // Read stdout in a thread
        let app_clone = app.clone();
        let project_id_clone = project_id.to_string();
        let stdout_handle = std::thread::spawn(move || {
            if let Some(stdout) = stdout {
                let reader = BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    Self::emit_output(&app_clone, &project_id_clone, &line, "stdout");
                }
            }
        });

        // Read stderr
        let mut all_stderr = String::new();
        if let Some(stderr) = stderr {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                Self::emit_output(app, project_id, &line, "stderr");
                all_stderr.push_str(&line);
                all_stderr.push('\n');
            }
        }

        stdout_handle.join().ok();

        let status = child.wait().map_err(|e| format!("Failed to wait for composer: {}", e))?;

        if status.success() {
            Self::emit_output(app, project_id, "✓ Laravel installed successfully!", "status");

            // Run additional setup commands
            Self::emit_output(app, project_id, "Running php artisan key:generate...", "status");
            let _ = Command::new("docker")
                .args(["exec", "-w", "/var/www/html", &container_name, "php", "artisan", "key:generate", "--force"])
                .current_dir(path)
                .output();

            Self::emit_output(app, project_id, "Setting storage permissions...", "status");
            let _ = Command::new("docker")
                .args(["exec", "-w", "/var/www/html", &container_name, "chmod", "-R", "777", "storage", "bootstrap/cache"])
                .current_dir(path)
                .output();

            Self::emit_output(app, project_id, "✓ Laravel setup complete!", "status");
            Ok("Laravel installed successfully".to_string())
        } else {
            Self::emit_output(app, project_id, &format!("✗ Laravel installation failed: {}", all_stderr), "status");
            Err(format!("Composer failed: {}", all_stderr))
        }
    }

    fn run_docker_compose_streaming(app: &AppHandle, project_id: &str, project_path: &str, args: &[&str]) -> Result<String, String> {
        let path = Path::new(project_path);

        Self::emit_output(app, project_id, &format!("Running: docker-compose {}", args.join(" ")), "status");

        let mut child = Command::new("docker-compose")
            .args(args)
            .current_dir(path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn docker-compose: {}", e))?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        // Read stdout in a thread
        let app_clone = app.clone();
        let project_id_clone = project_id.to_string();
        let stdout_handle = std::thread::spawn(move || {
            if let Some(stdout) = stdout {
                let reader = BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    Self::emit_output(&app_clone, &project_id_clone, &line, "stdout");
                }
            }
        });

        // Read stderr in main thread
        let mut all_stderr = String::new();
        if let Some(stderr) = stderr {
            let reader = BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                Self::emit_output(app, project_id, &line, "stderr");
                all_stderr.push_str(&line);
                all_stderr.push('\n');
            }
        }

        stdout_handle.join().ok();

        let status = child.wait().map_err(|e| format!("Failed to wait for process: {}", e))?;

        if status.success() {
            Self::emit_output(app, project_id, "✓ Command completed successfully", "status");
            Ok("Command completed successfully".to_string())
        } else {
            Self::emit_output(app, project_id, &format!("✗ Command failed with exit code: {:?}", status.code()), "status");
            Err(format!("Command failed: {}", all_stderr))
        }
    }

    fn emit_output(app: &AppHandle, project_id: &str, line: &str, stream_type: &str) {
        let event = DockerOutputEvent {
            project_id: project_id.to_string(),
            line: line.to_string(),
            stream_type: stream_type.to_string(),
        };
        let _ = app.emit("docker-output", event);
    }

    pub fn get_project_status(project_path: &str, project_name: &str) -> Result<Vec<ServiceStatus>, String> {
        let path = Path::new(project_path);

        let output = Command::new("docker-compose")
            .args(["ps", "--format", "json"])
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to get project status: {}", e))?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut services = Vec::new();

            // Parse docker-compose ps output
            for line in stdout.lines() {
                if let Ok(container) = serde_json::from_str::<serde_json::Value>(line) {
                    let name = container["Name"].as_str().unwrap_or("").to_string();
                    let state = container["State"].as_str().unwrap_or("unknown").to_string();
                    let ports = container["Ports"].as_str().unwrap_or("").to_string();

                    services.push(ServiceStatus {
                        name,
                        status: state,
                        container_id: container["ID"].as_str().map(|s| s.to_string()),
                        ports: ports.split(',').map(|s| s.trim().to_string()).collect(),
                        health: container["Health"].as_str().map(|s| s.to_string()),
                    });
                }
            }

            // If JSON parsing didn't work, try regular ps
            if services.is_empty() {
                services = Self::get_services_fallback(project_path, project_name)?;
            }

            Ok(services)
        } else {
            // Project might not be running
            Ok(vec![])
        }
    }

    fn get_services_fallback(project_path: &str, project_name: &str) -> Result<Vec<ServiceStatus>, String> {
        let output = Command::new("docker")
            .args(["ps", "-a", "--filter", &format!("name={}_", project_name), "--format", "{{.Names}}\t{{.Status}}\t{{.Ports}}\t{{.ID}}"])
            .output()
            .map_err(|e| format!("Failed to get services: {}", e))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut services = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 4 {
                let status_str = parts[1].to_lowercase();
                let status = if status_str.contains("up") {
                    "running".to_string()
                } else if status_str.contains("exited") {
                    "stopped".to_string()
                } else {
                    status_str
                };

                services.push(ServiceStatus {
                    name: parts[0].to_string(),
                    status,
                    container_id: Some(parts[3].to_string()),
                    ports: parts[2].split(',').map(|s| s.trim().to_string()).collect(),
                    health: None,
                });
            }
        }

        Ok(services)
    }

    pub fn get_container_logs(project_path: &str, service: &str, lines: u32) -> Result<String, String> {
        let path = Path::new(project_path);

        let output = Command::new("docker-compose")
            .args(["logs", "--tail", &lines.to_string(), service])
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to get logs: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string() + &String::from_utf8_lossy(&output.stderr))
    }

    pub fn exec_in_container(project_path: &str, service: &str, command: &str) -> Result<String, String> {
        let path = Path::new(project_path);

        let output = Command::new("docker-compose")
            .args(["exec", "-T", service, "sh", "-c", command])
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn run_artisan(project_path: &str, command: &str) -> Result<String, String> {
        Self::exec_in_container(project_path, "app", &format!("php artisan {}", command))
    }

    pub fn run_make(project_path: &str, target: &str) -> Result<String, String> {
        let path = Path::new(project_path);

        let output = Command::new("make")
            .arg(target)
            .current_dir(path)
            .output()
            .map_err(|e| format!("Failed to run make: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    pub fn get_supervisor_status(project_path: &str) -> Result<SupervisorStatus, String> {
        let output = Self::exec_in_container(project_path, "app", "supervisorctl status")?;

        let mut programs = Vec::new();
        let mut running = 0;
        let mut stopped = 0;
        let mut failed = 0;

        for line in output.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0].to_string();
                let status = parts[1].to_lowercase();

                match status.as_str() {
                    "running" => running += 1,
                    "stopped" | "exited" => stopped += 1,
                    "fatal" | "backoff" => failed += 1,
                    _ => {}
                }

                programs.push(SupervisorProgram {
                    name,
                    status: parts[1].to_string(),
                    pid: parts.get(3).and_then(|s| s.parse().ok()),
                    uptime: parts.get(4).map(|s| s.to_string()),
                });
            }
        }

        Ok(SupervisorStatus {
            total_programs: programs.len() as i32,
            running,
            stopped,
            failed,
            programs,
        })
    }

    pub fn reload_supervisor(project_path: &str) -> Result<String, String> {
        Self::exec_in_container(project_path, "app", "supervisorctl reread && supervisorctl update")
    }

    pub fn restart_supervisor(project_path: &str) -> Result<String, String> {
        Self::exec_in_container(project_path, "app", "supervisorctl restart all")
    }

    // ============ Queue Management ============

    pub fn start_queue_worker(project_path: &str) -> Result<String, String> {
        Self::exec_in_container(project_path, "app", "supervisorctl start laravel-worker:*")
    }

    pub fn stop_queue_worker(project_path: &str) -> Result<String, String> {
        Self::exec_in_container(project_path, "app", "supervisorctl stop laravel-worker:*")
    }

    pub fn restart_queue_worker(project_path: &str) -> Result<String, String> {
        Self::exec_in_container(project_path, "app", "php artisan queue:restart")
    }

    pub fn get_failed_jobs(project_path: &str) -> Result<String, String> {
        Self::run_artisan(project_path, "queue:failed --json")
    }

    pub fn retry_failed_job(project_path: &str, job_id: &str) -> Result<String, String> {
        Self::run_artisan(project_path, &format!("queue:retry {}", job_id))
    }

    pub fn retry_all_failed_jobs(project_path: &str) -> Result<String, String> {
        Self::run_artisan(project_path, "queue:retry all")
    }

    pub fn clear_failed_jobs(project_path: &str) -> Result<String, String> {
        Self::run_artisan(project_path, "queue:flush")
    }

    pub fn get_queue_size(project_path: &str, queue: &str) -> Result<String, String> {
        Self::run_artisan(project_path, &format!("queue:monitor {} --json", queue))
    }

    // ============ Scheduler Management ============

    pub fn get_scheduled_tasks(project_path: &str) -> Result<String, String> {
        Self::run_artisan(project_path, "schedule:list")
    }

    pub fn run_scheduler(project_path: &str) -> Result<String, String> {
        Self::run_artisan(project_path, "schedule:run")
    }

    pub fn run_scheduled_task(project_path: &str, command: &str) -> Result<String, String> {
        // Run a specific artisan command that's in the scheduler
        Self::run_artisan(project_path, command)
    }

    pub fn get_scheduler_status(project_path: &str) -> Result<String, String> {
        // Check if scheduler is running via supervisor
        Self::exec_in_container(project_path, "app", "supervisorctl status laravel-scheduler 2>/dev/null || echo 'not_configured'")
    }

    pub fn start_scheduler(project_path: &str) -> Result<String, String> {
        Self::exec_in_container(project_path, "app", "supervisorctl start laravel-scheduler")
    }

    pub fn stop_scheduler(project_path: &str) -> Result<String, String> {
        Self::exec_in_container(project_path, "app", "supervisorctl stop laravel-scheduler")
    }

    // ============ Database Backup & Restore ============

    pub fn backup_database(project_path: &str, project_name: &str) -> Result<String, String> {
        let path = Path::new(project_path);
        let backups_dir = path.join("backups");

        // Create backups directory if it doesn't exist
        std::fs::create_dir_all(&backups_dir)
            .map_err(|e| format!("Failed to create backups directory: {}", e))?;

        // Generate backup filename with timestamp
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("backup_{}.sql", timestamp);
        let backup_path = format!("/var/www/html/backups/{}", backup_name);

        // Run mysqldump inside the app container
        let dump_cmd = format!(
            "mysqldump -h db -u laravel -plaravel {} > {}",
            project_name, backup_path
        );

        Self::exec_in_container(project_path, "app", &dump_cmd)?;

        Ok(format!("Backup created: {}", backup_name))
    }

    pub fn restore_database(project_path: &str, project_name: &str, backup_name: &str) -> Result<String, String> {
        let backup_path = format!("/var/www/html/backups/{}", backup_name);

        // Restore from backup file
        let restore_cmd = format!(
            "mysql -h db -u laravel -plaravel {} < {}",
            project_name, backup_path
        );

        Self::exec_in_container(project_path, "app", &restore_cmd)?;

        Ok(format!("Database restored from: {}", backup_name))
    }

    pub fn list_backups(project_path: &str) -> Result<Vec<String>, String> {
        let path = Path::new(project_path);
        let backups_dir = path.join("backups");

        if !backups_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups: Vec<String> = std::fs::read_dir(&backups_dir)
            .map_err(|e| format!("Failed to read backups directory: {}", e))?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    if name.ends_with(".sql") {
                        Some(name)
                    } else {
                        None
                    }
                })
            })
            .collect();

        // Sort by name (which includes timestamp) in descending order
        backups.sort_by(|a, b| b.cmp(a));

        Ok(backups)
    }

    pub fn delete_backup(project_path: &str, backup_name: &str) -> Result<String, String> {
        let path = Path::new(project_path);
        let backup_path = path.join("backups").join(backup_name);

        if !backup_path.exists() {
            return Err(format!("Backup not found: {}", backup_name));
        }

        std::fs::remove_file(&backup_path)
            .map_err(|e| format!("Failed to delete backup: {}", e))?;

        Ok(format!("Backup deleted: {}", backup_name))
    }

    pub fn get_backup_size(project_path: &str, backup_name: &str) -> Result<u64, String> {
        let path = Path::new(project_path);
        let backup_path = path.join("backups").join(backup_name);

        let metadata = std::fs::metadata(&backup_path)
            .map_err(|e| format!("Failed to get backup size: {}", e))?;

        Ok(metadata.len())
    }

    // ============ Terminal / Exec ============

    pub fn exec_interactive_command(project_path: &str, service: &str, command: &str) -> Result<String, String> {
        Self::exec_in_container(project_path, service, command)
    }
}
