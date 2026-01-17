// Storage Blocks (MinIO)

import type { ServiceBlock } from './types'

export const storageBlocks: ServiceBlock[] = [
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
