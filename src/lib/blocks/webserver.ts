// Web Server Blocks (Nginx, Caddy)

import type { ServiceBlock } from './types'

export const webserverBlocks: ServiceBlock[] = [
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
  }
]
