// Database Blocks (MySQL, MariaDB, PostgreSQL, MongoDB)

import type { ServiceBlock } from './types'

export const databaseBlocks: ServiceBlock[] = [
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
  }
]
