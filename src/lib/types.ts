export interface Project {
  id: string
  name: string
  template: string
  path: string
  created_at: string
  updated_at: string
  config: ProjectConfig
  status: ProjectStatus
}

export type ProjectStatus = 'running' | 'stopped' | 'error' | 'building' | 'starting' | 'stopping'

export interface ProjectConfig {
  php_version?: string
  node_version: string
  install_bun: boolean
  install_pnpm: boolean
  install_yarn: boolean
  install_laravel: boolean
  services: ServiceConfig
  ports: PortConfig
}

export interface ServiceConfig {
  mysql: boolean
  redis: boolean
  phpmyadmin: boolean
  mailhog: boolean
  nginx: boolean
}

export interface PortConfig {
  app: number
  vite: number
  db: number
  redis: number
  phpmyadmin: number
  mailhog: number
}

export interface ServiceStatus {
  name: string
  status: string
  container_id?: string
  ports: string[]
  health?: string
}

export interface SupervisorProgram {
  name: string
  status: string
  pid?: number
  uptime?: string
}

export interface SupervisorStatus {
  total_programs: number
  running: number
  stopped: number
  failed: number
  programs: SupervisorProgram[]
}

export interface Template {
  name: string
  description: string
  icon: string
  template_type: string
  category: string
}

export interface Settings {
  projects_path: string
  auto_start_projects: boolean
  preferred_editor: string
  default_php_version: string
  default_node_version: string
  theme: string
}

export interface BackupInfo {
  name: string
  size: number
  created_at: string
}
