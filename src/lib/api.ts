import { invoke } from '@tauri-apps/api/core'
import type { Project, ProjectConfig, Template, Settings, ServiceStatus, SupervisorStatus } from './types'

// Project API
export const api = {
  // Projects
  async getProjects(): Promise<Project[]> {
    return await invoke('get_projects')
  },

  async createProject(name: string, template: string, config: ProjectConfig): Promise<Project> {
    return await invoke('create_project', {
      request: { name, template, config }
    })
  },

  async deleteProject(projectId: string, deleteFiles: boolean = true): Promise<void> {
    return await invoke('delete_project', { projectId, deleteFiles })
  },

  async getProject(projectId: string): Promise<Project> {
    return await invoke('get_project', { projectId })
  },

  async getProjectEnv(projectId: string): Promise<string> {
    return await invoke('get_project_env', { projectId })
  },

  async updateProjectEnv(projectId: string, envContent: string): Promise<void> {
    return await invoke('update_project_env', { projectId, envContent })
  },

  async openProjectFolder(projectId: string): Promise<void> {
    return await invoke('open_project_folder', { projectId })
  },

  async openProjectInEditor(projectId: string, editor: string = 'code'): Promise<void> {
    return await invoke('open_project_in_editor', { projectId, editor })
  },

  // Docker
  async startProject(projectId: string): Promise<string> {
    return await invoke('start_project', { projectId })
  },

  async stopProject(projectId: string): Promise<string> {
    return await invoke('stop_project', { projectId })
  },

  async restartProject(projectId: string): Promise<string> {
    return await invoke('restart_project', { projectId })
  },

  async rebuildProject(projectId: string): Promise<string> {
    return await invoke('rebuild_project', { projectId })
  },

  // Streaming versions (with real-time output)
  async startProjectStreaming(projectId: string): Promise<string> {
    return await invoke('start_project_streaming', { projectId })
  },

  async stopProjectStreaming(projectId: string): Promise<string> {
    return await invoke('stop_project_streaming', { projectId })
  },

  async rebuildProjectStreaming(projectId: string): Promise<string> {
    return await invoke('rebuild_project_streaming', { projectId })
  },

  async installLaravelStreaming(projectId: string): Promise<string> {
    return await invoke('install_laravel_streaming', { projectId })
  },

  async getProjectStatus(projectId: string): Promise<ServiceStatus[]> {
    return await invoke('get_project_status', { projectId })
  },

  async getContainerLogs(projectId: string, service: string, lines: number = 100): Promise<string> {
    return await invoke('get_container_logs', { projectId, service, lines })
  },

  async getServicesStatus(projectId: string): Promise<ServiceStatus[]> {
    return await invoke('get_services_status', { projectId })
  },

  // Templates
  async getTemplates(): Promise<Template[]> {
    return await invoke('get_templates')
  },

  async getTemplate(templateType: string): Promise<any> {
    return await invoke('get_template', { templateType })
  },

  // Artisan
  async runArtisanCommand(projectId: string, command: string): Promise<string> {
    return await invoke('run_artisan_command', { projectId, command })
  },

  async runMakeCommand(projectId: string, target: string): Promise<string> {
    return await invoke('run_make_command', { projectId, target })
  },

  // Queue
  async startQueueWorker(projectId: string): Promise<string> {
    return await invoke('start_queue_worker', { projectId })
  },

  async stopQueueWorker(projectId: string): Promise<string> {
    return await invoke('stop_queue_worker', { projectId })
  },

  async restartQueueWorker(projectId: string): Promise<string> {
    return await invoke('restart_queue_worker', { projectId })
  },

  async getQueueStatus(projectId: string): Promise<string> {
    return await invoke('get_queue_status', { projectId })
  },

  async getFailedJobs(projectId: string): Promise<string> {
    return await invoke('get_failed_jobs', { projectId })
  },

  async retryFailedJob(projectId: string, jobId: string): Promise<string> {
    return await invoke('retry_failed_job', { projectId, jobId })
  },

  async retryAllFailedJobs(projectId: string): Promise<string> {
    return await invoke('retry_all_failed_jobs', { projectId })
  },

  async clearFailedJobs(projectId: string): Promise<string> {
    return await invoke('clear_failed_jobs', { projectId })
  },

  // Scheduler
  async getScheduledTasks(projectId: string): Promise<string> {
    return await invoke('get_scheduled_tasks', { projectId })
  },

  async runScheduler(projectId: string): Promise<string> {
    return await invoke('run_scheduler', { projectId })
  },

  async runScheduledTask(projectId: string, command: string): Promise<string> {
    return await invoke('run_scheduled_task', { projectId, command })
  },

  async getSchedulerStatus(projectId: string): Promise<string> {
    return await invoke('get_scheduler_status', { projectId })
  },

  async startScheduler(projectId: string): Promise<string> {
    return await invoke('start_scheduler', { projectId })
  },

  async stopScheduler(projectId: string): Promise<string> {
    return await invoke('stop_scheduler', { projectId })
  },

  // Project Cloning
  async cloneProject(projectId: string, newName: string): Promise<Project> {
    return await invoke('clone_project', { projectId, newName })
  },

  async importProject(sourcePath: string, name: string): Promise<Project> {
    return await invoke('import_project', { sourcePath, name })
  },

  // Cache
  async clearCache(projectId: string, cacheType: string = 'all'): Promise<string> {
    return await invoke('clear_cache', { projectId, cacheType })
  },

  async optimizeApp(projectId: string): Promise<string> {
    return await invoke('optimize_app', { projectId })
  },

  // Database
  async runMigrations(projectId: string): Promise<string> {
    return await invoke('run_migrations', { projectId })
  },

  async runSeeders(projectId: string): Promise<string> {
    return await invoke('run_seeders', { projectId })
  },

  async freshDatabase(projectId: string): Promise<string> {
    return await invoke('fresh_database', { projectId })
  },

  // Supervisor
  async getSupervisorStatus(projectId: string): Promise<SupervisorStatus> {
    return await invoke('get_supervisor_status', { projectId })
  },

  async reloadSupervisor(projectId: string): Promise<string> {
    return await invoke('reload_supervisor', { projectId })
  },

  async restartSupervisor(projectId: string): Promise<string> {
    return await invoke('restart_supervisor', { projectId })
  },

  // Settings
  async getSettings(): Promise<Settings> {
    return await invoke('get_settings')
  },

  async saveSettings(settings: Settings): Promise<void> {
    return await invoke('save_settings', { settings })
  },

  // System
  async checkDockerInstalled(): Promise<boolean> {
    return await invoke('check_docker_installed')
  },

  async getDockerVersion(): Promise<string> {
    return await invoke('get_docker_version')
  }
}
