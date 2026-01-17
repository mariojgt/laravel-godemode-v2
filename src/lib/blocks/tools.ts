// Tools Blocks (phpMyAdmin, Adminer, Redis Insight, Supervisor)

import type { ServiceBlock } from './types'

export const toolsBlocks: ServiceBlock[] = [
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
  }
]
