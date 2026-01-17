// Main export file for blocks module

// Types
export type {
  ConfigOption,
  ServiceBlock,
  BlockInstance,
  CustomTemplate,
  BlockCategory
} from './types'

// Block definitions by category
import { runtimeBlocks } from './runtime'
import { frameworkBlocks } from './frameworks'
import { webserverBlocks } from './webserver'
import { databaseBlocks } from './database'
import { cacheBlocks } from './cache'
import { searchBlocks } from './search'
import { queueBlocks } from './queue'
import { mailBlocks } from './mail'
import { websocketBlocks } from './websocket'
import { storageBlocks } from './storage'
import { toolsBlocks } from './tools'

// Combine all blocks into a single array
export const serviceBlocks = [
  ...runtimeBlocks,
  ...frameworkBlocks,
  ...webserverBlocks,
  ...databaseBlocks,
  ...cacheBlocks,
  ...searchBlocks,
  ...queueBlocks,
  ...mailBlocks,
  ...websocketBlocks,
  ...storageBlocks,
  ...toolsBlocks
]

// Export individual category arrays for direct access
export {
  runtimeBlocks,
  frameworkBlocks,
  webserverBlocks,
  databaseBlocks,
  cacheBlocks,
  searchBlocks,
  queueBlocks,
  mailBlocks,
  websocketBlocks,
  storageBlocks,
  toolsBlocks
}

// Preset templates
export { presetTemplates } from './presets'

// Helper functions
export {
  getBlockById,
  getBlocksByCategory,
  getCategories,
  createDefaultBlockInstance,
  validateBlocks,
  getSuggestedBlocks,
  canEnableBlock,
  getMissingRequirements
} from './helpers'
