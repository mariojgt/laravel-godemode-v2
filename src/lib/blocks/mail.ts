// Mail Blocks (Mailhog, Mailpit)

import type { ServiceBlock } from './types'

export const mailBlocks: ServiceBlock[] = [
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
  }
]
