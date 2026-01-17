// Helper Functions for Blocks

import type { ServiceBlock, BlockInstance, BlockCategory } from './types'
import { serviceBlocks } from './index'

/**
 * Get a block by its ID
 */
export function getBlockById(id: string): ServiceBlock | undefined {
  return serviceBlocks.find(b => b.id === id)
}

/**
 * Get all blocks in a specific category
 */
export function getBlocksByCategory(category: BlockCategory): ServiceBlock[] {
  return serviceBlocks.filter(b => b.category === category)
}

/**
 * Get all available categories with their metadata
 */
export function getCategories(): { id: BlockCategory; label: string; icon: string }[] {
  return [
    { id: 'runtime', label: 'Runtime', icon: '⚙️' },
    { id: 'webserver', label: 'Web Server', icon: '🌐' },
    { id: 'database', label: 'Database', icon: '🗄️' },
    { id: 'cache', label: 'Cache', icon: '⚡' },
    { id: 'search', label: 'Search', icon: '🔍' },
    { id: 'queue', label: 'Queue', icon: '📋' },
    { id: 'mail', label: 'Mail', icon: '📧' },
    { id: 'websocket', label: 'WebSocket', icon: '📡' },
    { id: 'storage', label: 'Storage', icon: '☁️' },
    { id: 'tools', label: 'Tools', icon: '🔧' },
  ]
}

/**
 * Create a default block instance with all config options set to defaults
 */
export function createDefaultBlockInstance(block: ServiceBlock): BlockInstance {
  const config: Record<string, any> = {}
  for (const opt of block.configOptions) {
    config[opt.key] = opt.default
  }
  return {
    blockId: block.id,
    enabled: block.defaultEnabled ?? false,
    version: block.defaultVersion,
    config
  }
}

/**
 * Validate a set of block instances for compatibility and required dependencies
 */
export function validateBlocks(instances: BlockInstance[]): { valid: boolean; errors: string[] } {
  const errors: string[] = []
  const enabledIds = instances.filter(i => i.enabled).map(i => i.blockId)

  for (const instance of instances) {
    if (!instance.enabled) continue

    const block = getBlockById(instance.blockId)
    if (!block) {
      errors.push(`Unknown block: ${instance.blockId}`)
      continue
    }

    // Check required dependencies
    if (block.requires) {
      for (const req of block.requires) {
        if (!enabledIds.includes(req)) {
          const reqBlock = getBlockById(req)
          errors.push(`${block.name} requires ${reqBlock?.name || req}`)
        }
      }
    }

    // Check incompatibilities
    if (block.incompatibleWith) {
      for (const inc of block.incompatibleWith) {
        if (enabledIds.includes(inc)) {
          const incBlock = getBlockById(inc)
          errors.push(`${block.name} is incompatible with ${incBlock?.name || inc}`)
        }
      }
    }
  }

  return { valid: errors.length === 0, errors }
}

/**
 * Get suggested blocks based on currently enabled blocks
 */
export function getSuggestedBlocks(enabledBlockIds: string[]): ServiceBlock[] {
  const suggestions = new Set<string>()

  for (const id of enabledBlockIds) {
    const block = getBlockById(id)
    if (block?.suggestedWith) {
      for (const suggested of block.suggestedWith) {
        if (!enabledBlockIds.includes(suggested)) {
          suggestions.add(suggested)
        }
      }
    }
  }

  return Array.from(suggestions)
    .map(id => getBlockById(id))
    .filter((b): b is ServiceBlock => b !== undefined)
}

/**
 * Check if a block can be enabled based on incompatibilities
 */
export function canEnableBlock(blockId: string, enabledBlockIds: string[]): { canEnable: boolean; conflicts: string[] } {
  const block = getBlockById(blockId)
  if (!block) {
    return { canEnable: false, conflicts: ['Block not found'] }
  }

  const conflicts: string[] = []

  if (block.incompatibleWith) {
    for (const inc of block.incompatibleWith) {
      if (enabledBlockIds.includes(inc)) {
        const incBlock = getBlockById(inc)
        conflicts.push(incBlock?.name || inc)
      }
    }
  }

  return { canEnable: conflicts.length === 0, conflicts }
}

/**
 * Get missing required blocks for an enabled block
 */
export function getMissingRequirements(blockId: string, enabledBlockIds: string[]): ServiceBlock[] {
  const block = getBlockById(blockId)
  if (!block?.requires) {
    return []
  }

  return block.requires
    .filter(req => !enabledBlockIds.includes(req))
    .map(req => getBlockById(req))
    .filter((b): b is ServiceBlock => b !== undefined)
}
