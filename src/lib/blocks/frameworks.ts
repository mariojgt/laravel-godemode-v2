// Framework Blocks (Astro, Next.js, Nuxt)

import type { ServiceBlock } from './types'

export const frameworkBlocks: ServiceBlock[] = [
  {
    id: 'astro',
    name: 'Astro',
    icon: '🚀',
    category: 'runtime',
    description: 'The web framework for content-driven websites',
    versions: ['4.x', '3.x'],
    defaultVersion: '4.x',
    suggestedWith: ['nodejs'],
    requires: ['nodejs'],
    configOptions: [
      {
        key: 'port',
        label: 'Dev Server Port',
        type: 'number',
        default: 4321
      },
      {
        key: 'output',
        label: 'Output Mode',
        type: 'select',
        default: 'static',
        options: [
          { value: 'static', label: 'Static (Pre-rendered)' },
          { value: 'server', label: 'Server (SSR)' },
          { value: 'hybrid', label: 'Hybrid (Mixed)' },
        ],
        description: 'How your site is rendered'
      },
      {
        key: 'integrations',
        label: 'Integrations',
        type: 'textarea',
        default: 'tailwind, sitemap',
        description: 'Comma-separated list of Astro integrations'
      }
    ]
  },
  {
    id: 'nextjs',
    name: 'Next.js',
    icon: '▲',
    category: 'runtime',
    description: 'The React framework for the web',
    versions: ['15', '14', '13'],
    defaultVersion: '15',
    suggestedWith: ['nodejs', 'postgresql', 'redis'],
    requires: ['nodejs'],
    configOptions: [
      {
        key: 'port',
        label: 'Dev Server Port',
        type: 'number',
        default: 3000
      },
      {
        key: 'app_router',
        label: 'Use App Router',
        type: 'checkbox',
        default: true,
        description: 'Use the new App Router (recommended)'
      },
      {
        key: 'typescript',
        label: 'TypeScript',
        type: 'checkbox',
        default: true
      },
      {
        key: 'tailwind',
        label: 'Tailwind CSS',
        type: 'checkbox',
        default: true
      },
      {
        key: 'src_dir',
        label: 'Use src/ directory',
        type: 'checkbox',
        default: true
      }
    ]
  },
  {
    id: 'nuxt',
    name: 'Nuxt',
    icon: '💚',
    category: 'runtime',
    description: 'The intuitive Vue framework',
    versions: ['3.x'],
    defaultVersion: '3.x',
    suggestedWith: ['nodejs', 'postgresql', 'redis'],
    requires: ['nodejs'],
    configOptions: [
      {
        key: 'port',
        label: 'Dev Server Port',
        type: 'number',
        default: 3000
      },
      {
        key: 'ssr',
        label: 'Server-Side Rendering',
        type: 'checkbox',
        default: true
      },
      {
        key: 'typescript',
        label: 'TypeScript',
        type: 'checkbox',
        default: true
      },
      {
        key: 'modules',
        label: 'Nuxt Modules',
        type: 'textarea',
        default: '@nuxtjs/tailwindcss, @pinia/nuxt',
        description: 'Comma-separated list of Nuxt modules'
      }
    ]
  }
]
