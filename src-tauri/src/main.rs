// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod docker;
mod project;
mod template;
mod state;

use state::AppState;
use std::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_notification::init())
        .manage(Mutex::new(AppState::new()))
        .invoke_handler(tauri::generate_handler![
            // Project commands
            commands::get_projects,
            commands::create_project,
            commands::delete_project,
            commands::get_project,
            commands::get_project_env,
            commands::update_project_env,
            commands::open_project_folder,
            commands::open_project_in_editor,
            // Docker commands
            commands::start_project,
            commands::stop_project,
            commands::restart_project,
            commands::rebuild_project,
            commands::start_project_streaming,
            commands::stop_project_streaming,
            commands::rebuild_project_streaming,
            commands::install_laravel_streaming,
            commands::get_project_status,
            commands::get_container_logs,
            commands::get_services_status,
            // Template commands
            commands::get_templates,
            commands::get_template,
            // Artisan commands
            commands::run_artisan_command,
            commands::run_make_command,
            // Queue commands
            commands::start_queue_worker,
            commands::stop_queue_worker,
            commands::restart_queue_worker,
            commands::get_queue_status,
            commands::get_failed_jobs,
            commands::retry_failed_job,
            commands::retry_all_failed_jobs,
            commands::clear_failed_jobs,
            // Cache commands
            commands::clear_cache,
            commands::optimize_app,
            // Database commands
            commands::run_migrations,
            commands::run_seeders,
            commands::fresh_database,
            // Supervisor commands
            commands::get_supervisor_status,
            commands::reload_supervisor,
            commands::restart_supervisor,
            // Scheduler commands
            commands::get_scheduled_tasks,
            commands::run_scheduler,
            commands::run_scheduled_task,
            commands::get_scheduler_status,
            commands::start_scheduler,
            commands::stop_scheduler,
            // Project cloning commands
            commands::clone_project,
            commands::import_project,
            // Database backup commands
            commands::backup_database,
            commands::restore_database,
            commands::list_backups,
            commands::delete_backup,
            commands::get_backups_with_info,
            // Terminal / Exec commands
            commands::exec_container_command,
            commands::run_tinker_command,
            commands::run_composer_command,
            commands::run_npm_command,
            // Settings commands
            commands::get_settings,
            commands::save_settings,
            // System commands
            commands::check_docker_installed,
            commands::get_docker_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
