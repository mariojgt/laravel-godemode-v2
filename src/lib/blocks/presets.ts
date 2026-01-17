// Preset Templates

import type { CustomTemplate } from './types'

export const presetTemplates: Omit<CustomTemplate, 'id' | 'createdAt' | 'updatedAt'>[] = [
  {
    name: 'Laravel Full Stack',
    description: 'Complete Laravel setup with all essentials',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'mysql', enabled: true, version: '8.0', config: { port: 3306, database: 'laravel' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
      { blockId: 'mailhog', enabled: true, config: { smtp_port: 1025, ui_port: 8025 } },
      { blockId: 'phpmyadmin', enabled: true, config: { port: 8080 } },
      { blockId: 'supervisor', enabled: true, config: { queue_workers: 2, scheduler: true } },
    ]
  },
  {
    name: 'Laravel API',
    description: 'Lightweight API-focused setup',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'postgresql', enabled: true, version: '16', config: { port: 5432, database: 'laravel' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
    ]
  },
  {
    name: 'Laravel + Meilisearch',
    description: 'Full-text search ready setup',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'mysql', enabled: true, version: '8.0', config: { port: 3306, database: 'laravel' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
      { blockId: 'meilisearch', enabled: true, version: '1.6', config: { port: 7700 } },
      { blockId: 'mailhog', enabled: true, config: { smtp_port: 1025, ui_port: 8025 } },
    ]
  },
  {
    name: 'Laravel + WebSockets',
    description: 'Real-time application setup',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'mysql', enabled: true, version: '8.0', config: { port: 3306, database: 'laravel' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
      { blockId: 'soketi', enabled: true, config: { port: 6001 } },
      { blockId: 'mailhog', enabled: true, config: { smtp_port: 1025, ui_port: 8025 } },
    ]
  },
  {
    name: 'Minimal',
    description: 'Bare minimum for simple projects',
    blocks: [
      { blockId: 'php-fpm', enabled: true, version: '8.4', config: {} },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'mysql', enabled: true, version: '8.0', config: { port: 3306, database: 'laravel' } },
    ]
  },

  // ===== JAVASCRIPT FRAMEWORKS =====
  {
    name: 'Astro Static Site',
    description: 'Content-driven static site with Astro',
    blocks: [
      { blockId: 'nodejs', enabled: true, version: '20', config: { package_manager: 'npm' } },
      { blockId: 'astro', enabled: true, version: '4.x', config: { port: 4321, output: 'static' } },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
    ]
  },
  {
    name: 'Astro + Database',
    description: 'Astro with SSR and database backend',
    blocks: [
      { blockId: 'nodejs', enabled: true, version: '20', config: { package_manager: 'npm' } },
      { blockId: 'astro', enabled: true, version: '4.x', config: { port: 4321, output: 'server' } },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'postgresql', enabled: true, version: '16', config: { port: 5432, database: 'astro' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
    ]
  },
  {
    name: 'Next.js Full Stack',
    description: 'Complete Next.js setup with database',
    blocks: [
      { blockId: 'nodejs', enabled: true, version: '20', config: { package_manager: 'npm' } },
      { blockId: 'nextjs', enabled: true, version: '15', config: { port: 3000, app_router: true, typescript: true, tailwind: true } },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'postgresql', enabled: true, version: '16', config: { port: 5432, database: 'nextjs' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
      { blockId: 'mailhog', enabled: true, config: { smtp_port: 1025, ui_port: 8025 } },
    ]
  },
  {
    name: 'Next.js Static',
    description: 'Next.js for static site generation',
    blocks: [
      { blockId: 'nodejs', enabled: true, version: '20', config: { package_manager: 'npm' } },
      { blockId: 'nextjs', enabled: true, version: '15', config: { port: 3000, app_router: true, typescript: true, tailwind: true } },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
    ]
  },
  {
    name: 'Nuxt Full Stack',
    description: 'Complete Nuxt 3 setup with database',
    blocks: [
      { blockId: 'nodejs', enabled: true, version: '20', config: { package_manager: 'npm' } },
      { blockId: 'nuxt', enabled: true, version: '3.x', config: { port: 3000, ssr: true, typescript: true } },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
      { blockId: 'postgresql', enabled: true, version: '16', config: { port: 5432, database: 'nuxt' } },
      { blockId: 'redis', enabled: true, version: '7.2', config: { port: 6379 } },
      { blockId: 'mailhog', enabled: true, config: { smtp_port: 1025, ui_port: 8025 } },
    ]
  },
  {
    name: 'Nuxt Static',
    description: 'Nuxt 3 for static site generation',
    blocks: [
      { blockId: 'nodejs', enabled: true, version: '20', config: { package_manager: 'npm' } },
      { blockId: 'nuxt', enabled: true, version: '3.x', config: { port: 3000, ssr: false, typescript: true } },
      { blockId: 'nginx', enabled: true, config: { port: 80 } },
    ]
  }
]
