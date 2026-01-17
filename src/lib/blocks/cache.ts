// Cache Blocks (Redis, Memcached)

import type { ServiceBlock } from './types'

export const cacheBlocks: ServiceBlock[] = [
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
  }
]
