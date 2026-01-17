// Queue Blocks (RabbitMQ, Beanstalkd)

import type { ServiceBlock } from './types'

export const queueBlocks: ServiceBlock[] = [
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
  }
]
