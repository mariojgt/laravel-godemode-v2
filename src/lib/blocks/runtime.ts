// Runtime Blocks (PHP, Node.js)

import type { ServiceBlock } from './types'

export const runtimeBlocks: ServiceBlock[] = [
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
  }
]
