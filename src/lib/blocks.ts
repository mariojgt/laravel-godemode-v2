// Service Block Definitions for Template Builder

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

// ============ SERVICE BLOCK DEFINITIONS ============

export const serviceBlocks: ServiceBlock[] = [
  // ===== RUNTIME =====
  {
    id: 'php-fpm',
    name: 'PHP-FPM',
    icon: '🐘',
    category: 'runtime',
    description: 'PHP FastCGI Process Manager for Laravel applications',
    defaultEnabled: true,
    versions: ['8.4', '8.3', '8.2', '8.1'],
    defaultVersion: '8.4',
    suggestedWith: ['nginx', 'mysql', 'redis'],
    configOptions: [
      {
        key: 'memory_limit',
        label: 'Memory Limit',
        type: 'select',
        default: '256M',
        options: [
          { value: '128M', label: '128 MB' },
          { value: '256M', label: '256 MB' },
          { value: '512M', label: '512 MB' },
          { value: '1G', label: '1 GB' },
        ]
      },
      {
        key: 'max_execution_time',
        label: 'Max Execution Time (seconds)',
        type: 'number',
        default: 30
      },
      {
        key: 'upload_max_filesize',
        label: 'Max Upload Size',
        type: 'select',
        default: '64M',
        options: [
          { value: '2M', label: '2 MB' },
          { value: '8M', label: '8 MB' },
          { value: '64M', label: '64 MB' },
          { value: '128M', label: '128 MB' },
          { value: '256M', label: '256 MB' },
        ]
      },
      {
        key: 'extensions',
        label: 'PHP Extensions',
        type: 'textarea',
        default: 'pdo_mysql, redis, gd, zip, bcmath',
        description: 'Comma-separated list of PHP extensions'
      }
    ]
  },
  {
    id: 'nodejs',
    name: 'Node.js',
    icon: '⬢',
    category: 'runtime',
    description: 'Node.js for frontend builds and Vite dev server',
    versions: ['22', '20', '18'],
    defaultVersion: '20',
    configOptions: [
      {
        key: 'package_manager',
        label: 'Package Manager',
        type: 'select',
        default: 'npm',
        options: [
          { value: 'npm', label: 'NPM' },
          { value: 'yarn', label: 'Yarn' },
          { value: 'pnpm', label: 'PNPM' },
          { value: 'bun', label: 'Bun' },
        ]
      }
    ]
  },

  // ===== WEB SERVER =====
  {
    id: 'nginx',
    name: 'Nginx',
    icon: '🌐',
    category: 'webserver',
    description: 'High-performance web server and reverse proxy',
    defaultEnabled: true,
    requires: ['php-fpm'],
    configOptions: [
      {
        key: 'port',
        label: 'HTTP Port',
        type: 'number',
        default: 80
      },
      {
        key: 'client_max_body_size',
        label: 'Max Body Size',
        type: 'select',
        default: '64M',
        options: [
          { value: '8M', label: '8 MB' },
          { value: '64M', label: '64 MB' },
          { value: '128M', label: '128 MB' },
          { value: '256M', label: '256 MB' },
        ]
      }
    ]
  },
  {
    id: 'caddy',
    name: 'Caddy',
    icon: '🔒',
    category: 'webserver',
    description: 'Modern web server with automatic HTTPS',
    requires: ['php-fpm'],
    incompatibleWith: ['nginx'],
    configOptions: [
      {
        key: 'port',
        label: 'HTTP Port',
        type: 'number',
        default: 80
      },
      {
        key: 'https_port',
        label: 'HTTPS Port',
        type: 'number',
        default: 443
      },
      {
        key: 'auto_https',
        label: 'Auto HTTPS (local)',
        type: 'checkbox',
        default: false
      }
    ]
  },

  // ===== DATABASE =====
  {
    id: 'mysql',
    name: 'MySQL',
    icon: '🗄️',
    category: 'database',
    description: 'Popular relational database',
    defaultEnabled: true,
    versions: ['8.0', '5.7'],
    defaultVersion: '8.0',
    incompatibleWith: ['mariadb', 'postgresql'],
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 3306
      },
      {
        key: 'database',
        label: 'Database Name',
        type: 'text',
        default: 'laravel'
      },
      {
        key: 'username',
        label: 'Username',
        type: 'text',
        default: 'laravel'
      },
      {
        key: 'password',
        label: 'Password',
        type: 'text',
        default: 'secret'
      },
      {
        key: 'root_password',
        label: 'Root Password',
        type: 'text',
        default: 'secret'
      }
    ]
  },
  {
    id: 'mariadb',
    name: 'MariaDB',
    icon: '🗄️',
    category: 'database',
    description: 'MySQL-compatible database with extra features',
    versions: ['11.2', '10.11', '10.6'],
    defaultVersion: '10.11',
    incompatibleWith: ['mysql', 'postgresql'],
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 3306
      },
      {
        key: 'database',
        label: 'Database Name',
        type: 'text',
        default: 'laravel'
      },
      {
        key: 'username',
        label: 'Username',
        type: 'text',
        default: 'laravel'
      },
      {
        key: 'password',
        label: 'Password',
        type: 'text',
        default: 'secret'
      }
    ]
  },
  {
    id: 'postgresql',
    name: 'PostgreSQL',
    icon: '🐘',
    category: 'database',
    description: 'Advanced open-source relational database',
    versions: ['16', '15', '14'],
    defaultVersion: '16',
    incompatibleWith: ['mysql', 'mariadb'],
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 5432
      },
      {
        key: 'database',
        label: 'Database Name',
        type: 'text',
        default: 'laravel'
      },
      {
        key: 'username',
        label: 'Username',
        type: 'text',
        default: 'laravel'
      },
      {
        key: 'password',
        label: 'Password',
        type: 'text',
        default: 'secret'
      }
    ]
  },
  {
    id: 'mongodb',
    name: 'MongoDB',
    icon: '🍃',
    category: 'database',
    description: 'NoSQL document database',
    versions: ['7.0', '6.0'],
    defaultVersion: '7.0',
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 27017
      },
      {
        key: 'database',
        label: 'Database Name',
        type: 'text',
        default: 'laravel'
      }
    ]
  },

  // ===== CACHE =====
  {
    id: 'redis',
    name: 'Redis',
    icon: '⚡',
    category: 'cache',
    description: 'In-memory data store for caching and queues',
    defaultEnabled: true,
    versions: ['7.2', '7.0', '6.2'],
    defaultVersion: '7.2',
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 6379
      },
      {
        key: 'maxmemory',
        label: 'Max Memory',
        type: 'select',
        default: '256mb',
        options: [
          { value: '64mb', label: '64 MB' },
          { value: '128mb', label: '128 MB' },
          { value: '256mb', label: '256 MB' },
          { value: '512mb', label: '512 MB' },
        ]
      }
    ]
  },
  {
    id: 'memcached',
    name: 'Memcached',
    icon: '💾',
    category: 'cache',
    description: 'High-performance memory caching system',
    versions: ['1.6'],
    defaultVersion: '1.6',
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 11211
      },
      {
        key: 'memory',
        label: 'Memory (MB)',
        type: 'number',
        default: 64
      }
    ]
  },

  // ===== SEARCH =====
  {
    id: 'meilisearch',
    name: 'Meilisearch',
    icon: '🔍',
    category: 'search',
    description: 'Fast, typo-tolerant search engine for Laravel Scout',
    versions: ['1.6', '1.5'],
    defaultVersion: '1.6',
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 7700
      },
      {
        key: 'master_key',
        label: 'Master Key',
        type: 'text',
        default: 'masterKey'
      }
    ]
  },
  {
    id: 'elasticsearch',
    name: 'Elasticsearch',
    icon: '🔎',
    category: 'search',
    description: 'Distributed search and analytics engine',
    versions: ['8.12', '8.11', '7.17'],
    defaultVersion: '8.12',
    configOptions: [
      {
        key: 'port',
        label: 'HTTP Port',
        type: 'number',
        default: 9200
      },
      {
        key: 'java_opts',
        label: 'Java Heap Size',
        type: 'select',
        default: '-Xms512m -Xmx512m',
        options: [
          { value: '-Xms256m -Xmx256m', label: '256 MB' },
          { value: '-Xms512m -Xmx512m', label: '512 MB' },
          { value: '-Xms1g -Xmx1g', label: '1 GB' },
        ]
      }
    ]
  },
  {
    id: 'typesense',
    name: 'Typesense',
    icon: '⚡',
    category: 'search',
    description: 'Fast, typo-tolerant search engine',
    versions: ['0.25', '0.24'],
    defaultVersion: '0.25',
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 8108
      },
      {
        key: 'api_key',
        label: 'API Key',
        type: 'text',
        default: 'xyz'
      }
    ]
  },

  // ===== QUEUE =====
  {
    id: 'rabbitmq',
    name: 'RabbitMQ',
    icon: '🐰',
    category: 'queue',
    description: 'Message broker for queue processing',
    versions: ['3.13', '3.12'],
    defaultVersion: '3.13',
    configOptions: [
      {
        key: 'port',
        label: 'AMQP Port',
        type: 'number',
        default: 5672
      },
      {
        key: 'management_port',
        label: 'Management UI Port',
        type: 'number',
        default: 15672
      },
      {
        key: 'username',
        label: 'Username',
        type: 'text',
        default: 'guest'
      },
      {
        key: 'password',
        label: 'Password',
        type: 'text',
        default: 'guest'
      }
    ]
  },
  {
    id: 'beanstalkd',
    name: 'Beanstalkd',
    icon: '🫘',
    category: 'queue',
    description: 'Simple, fast work queue',
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 11300
      }
    ]
  },

  // ===== MAIL =====
  {
    id: 'mailhog',
    name: 'Mailhog',
    icon: '📧',
    category: 'mail',
    description: 'Email testing tool with web UI',
    defaultEnabled: true,
    configOptions: [
      {
        key: 'smtp_port',
        label: 'SMTP Port',
        type: 'number',
        default: 1025
      },
      {
        key: 'ui_port',
        label: 'Web UI Port',
        type: 'number',
        default: 8025
      }
    ]
  },
  {
    id: 'mailpit',
    name: 'Mailpit',
    icon: '📬',
    category: 'mail',
    description: 'Modern email testing tool (Mailhog alternative)',
    incompatibleWith: ['mailhog'],
    configOptions: [
      {
        key: 'smtp_port',
        label: 'SMTP Port',
        type: 'number',
        default: 1025
      },
      {
        key: 'ui_port',
        label: 'Web UI Port',
        type: 'number',
        default: 8025
      }
    ]
  },

  // ===== TOOLS =====
  {
    id: 'phpmyadmin',
    name: 'phpMyAdmin',
    icon: '📊',
    category: 'tools',
    description: 'Web-based MySQL/MariaDB administration',
    defaultEnabled: true,
    requires: ['mysql'],
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 8080
      }
    ]
  },
  {
    id: 'adminer',
    name: 'Adminer',
    icon: '🗃️',
    category: 'tools',
    description: 'Lightweight database management (all databases)',
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 8081
      }
    ]
  },
  {
    id: 'redisinsight',
    name: 'Redis Insight',
    icon: '📈',
    category: 'tools',
    description: 'Visual Redis management and monitoring',
    requires: ['redis'],
    configOptions: [
      {
        key: 'port',
        label: 'Port',
        type: 'number',
        default: 8001
      }
    ]
  },
  {
    id: 'supervisor',
    name: 'Supervisor',
    icon: '🔧',
    category: 'tools',
    description: 'Process manager for queue workers and schedulers',
    configOptions: [
      {
        key: 'queue_workers',
        label: 'Queue Workers',
        type: 'number',
        default: 2
      },
      {
        key: 'scheduler',
        label: 'Enable Scheduler',
        type: 'checkbox',
        default: true
      }
    ]
  },

  // ===== WEBSOCKET =====
  {
    id: 'soketi',
    name: 'Soketi',
    icon: '📡',
    category: 'websocket',
    description: 'Open-source WebSocket server (Pusher compatible)',
    configOptions: [
      {
        key: 'port',
        label: 'WebSocket Port',
        type: 'number',
        default: 6001
      },
      {
        key: 'app_id',
        label: 'App ID',
        type: 'text',
        default: 'app-id'
      },
      {
        key: 'app_key',
        label: 'App Key',
        type: 'text',
        default: 'app-key'
      },
      {
        key: 'app_secret',
        label: 'App Secret',
        type: 'text',
        default: 'app-secret'
      }
    ]
  },
  {
    id: 'reverb',
    name: 'Laravel Reverb',
    icon: '🔊',
    category: 'websocket',
    description: 'Laravel\'s first-party WebSocket server',
    requires: ['php-fpm'],
    configOptions: [
      {
        key: 'port',
        label: 'WebSocket Port',
        type: 'number',
        default: 8080
      }
    ]
  },

  // ===== STORAGE =====
  {
    id: 'minio',
    name: 'MinIO',
    icon: '☁️',
    category: 'storage',
    description: 'S3-compatible object storage',
    configOptions: [
      {
        key: 'port',
        label: 'API Port',
        type: 'number',
        default: 9000
      },
      {
        key: 'console_port',
        label: 'Console Port',
        type: 'number',
        default: 9001
      },
      {
        key: 'root_user',
        label: 'Root User',
        type: 'text',
        default: 'minioadmin'
      },
      {
        key: 'root_password',
        label: 'Root Password',
        type: 'text',
        default: 'minioadmin'
      },
      {
        key: 'default_bucket',
        label: 'Default Bucket',
        type: 'text',
        default: 'laravel'
      }
    ]
  }
]

// ============ PRESET TEMPLATES ============

export const presetTemplates: Omit<CustomTemplate, 'id' | 'createdAt' | 'updatedAt'>[] = [
  {
    name: 'Laravel Full Stack',
    description: 'Complete Laravel setup with all essentials',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'mysql', enabled: true, version: '8.0', config: { port: 3306, database: 'laravel' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
      { blockId: 'mailhog', enabled: true, config: { smtp_port: 1025, ui_port: 8025 } },
      { blockId: 'phpmyadmin', enabled: true, config: { port: 8080 } },
      { blockId: 'supervisor', enabled: true, config: { queue_workers: 2, scheduler: true } },
    ]
  },
  {
    name: 'Laravel API',
    description: 'Lightweight API-focused setup',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'postgresql', enabled: true, version: '16', config: { port: 5432, database: 'laravel' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
    ]
  },
  {
    name: 'Laravel + Meilisearch',
    description: 'Full-text search ready setup',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'mysql', enabled: true, version: '8.0', config: { port: 3306, database: 'laravel' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
      { blockId: 'meilisearch', enabled: true, version: '1.6', config: { port: 7700 } },
      { blockId: 'mailhog', enabled: true, config: { smtp_port: 1025, ui_port: 8025 } },
    ]
  },
  {
    name: 'Laravel + WebSockets',
    description: 'Real-time application setup',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'mysql', enabled: true, version: '8.0', config: { port: 3306, database: 'laravel' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
      { blockId: 'soketi', enabled: true, config: { port: 6001 } },
      { blockId: 'mailhog', enabled: true, config: { smtp_port: 1025, ui_port: 8025 } },
    ]
  },
  {
    name: 'Minimal',
    description: 'Bare minimum for simple projects',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'mysql', enabled: true, version: '8.0', config: { port: 3306, database: 'laravel' } },
    ]
  }
]

// ============ HELPER FUNCTIONS ============

export function getBlockById(id: string): ServiceBlock | undefined {
  return serviceBlocks.find(b => b.id === id)
}

export function getBlocksByCategory(category: ServiceBlock['category']): ServiceBlock[] {
  return serviceBlocks.filter(b => b.category === category)
}

export function getCategories(): { id: ServiceBlock['category']; label: string; icon: string }[] {
  return [
    { id: 'runtime', label: 'Runtime', icon: '⚙️' },
    { id: 'webserver', label: 'Web Server', icon: '🌐' },
    { id: 'database', label: 'Database', icon: '🗄️' },
    { id: 'cache', label: 'Cache', icon: '⚡' },
    { id: 'search', label: 'Search', icon: '🔍' },
    { id: 'queue', label: 'Queue', icon: '📋' },
    { id: 'mail', label: 'Mail', icon: '📧' },
    { id: 'websocket', label: 'WebSocket', icon: '📡' },
    { id: 'storage', label: 'Storage', icon: '☁️' },
    { id: 'tools', label: 'Tools', icon: '🔧' },
  ]
}

export function createDefaultBlockInstance(block: ServiceBlock): BlockInstance {
  const config: Record<string, any> = {}
  for (const opt of block.configOptions) {
    config[opt.key] = opt.default
  }
  return {
    blockId: block.id,
    enabled: block.defaultEnabled ?? false,
    version: block.defaultVersion,
    config
  }
}

export function validateBlocks(instances: BlockInstance[]): { valid: boolean; errors: string[] } {
  const errors: string[] = []
  const enabledIds = instances.filter(i => i.enabled).map(i => i.blockId)

  for (const instance of instances) {
    if (!instance.enabled) continue

    const block = getBlockById(instance.blockId)
    if (!block) {
      errors.push(`Unknown block: ${instance.blockId}`)
      continue
    }

    // Check required dependencies
    if (block.requires) {
      for (const req of block.requires) {
        if (!enabledIds.includes(req)) {
          const reqBlock = getBlockById(req)
          errors.push(`${block.name} requires ${reqBlock?.name || req}`)
        }
      }
    }

    // Check incompatibilities
    if (block.incompatibleWith) {
      for (const inc of block.incompatibleWith) {
        if (enabledIds.includes(inc)) {
          const incBlock = getBlockById(inc)
          errors.push(`${block.name} is incompatible with ${incBlock?.name || inc}`)
        }
      }
    }
  }

  return { valid: errors.length === 0, errors }
}
