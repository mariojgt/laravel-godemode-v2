// Block Types and Interfaces

export interface ConfigOption {
  key: string
  label: string
  type: 'text' | 'number' | 'select' | 'checkbox' | 'textarea'
  default: any
  options?: { value: string; label: string }[]
  description?: string
}

export interface ServiceBlock {
  id: string
  name: string
  icon: string
  category: 'runtime' | 'webserver' | 'database' | 'cache' | 'search' | 'queue' | 'mail' | 'tools' | 'websocket' | 'storage'
  description: string
  configOptions: ConfigOption[]
  requires?: string[]
  suggestedWith?: string[]
  incompatibleWith?: string[]
  defaultEnabled?: boolean
  versions?: string[]
  defaultVersion?: string
}

export interface BlockInstance {
  blockId: string
  enabled: boolean
  version?: string
  config: Record<string, any>
}

export interface CustomTemplate {
  id: string
  name: string
  description: string
  blocks: BlockInstance[]
  createdAt: string
  updatedAt: string
}

export type BlockCategory = ServiceBlock['category']
