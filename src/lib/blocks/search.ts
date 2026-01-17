// Search Blocks (Meilisearch, Elasticsearch, Typesense)

import type { ServiceBlock } from './types'

export const searchBlocks: ServiceBlock[] = [
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
  }
]
