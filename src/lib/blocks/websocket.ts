// WebSocket Blocks (Soketi, Laravel Reverb)

import type { ServiceBlock } from './types'

export const websocketBlocks: ServiceBlock[] = [
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
  }
]
