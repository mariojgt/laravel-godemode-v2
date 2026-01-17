<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import {
  serviceBlocks,
  presetTemplates,
  getCategories,
  getBlockById,
  createDefaultBlockInstance,
  validateBlocks,
  type ServiceBlock,
  type BlockInstance,
  type CustomTemplate
} from '@/lib/blocks'

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', template: CustomTemplate): void
  (e: 'create-project', template: CustomTemplate): void
}>()

const props = defineProps<{
  editTemplate?: CustomTemplate
}>()

// State
const step = ref<'select' | 'configure' | 'save'>(props.editTemplate ? 'configure' : 'select')
const templateName = ref(props.editTemplate?.name || '')
const templateDescription = ref(props.editTemplate?.description || '')
const blocks = ref<BlockInstance[]>([])
const selectedBlockId = ref<string | null>(null)
const searchQuery = ref('')
const selectedCategory = ref<string | null>(null)
const errors = ref<string[]>([])

const categories = getCategories()

// Initialize blocks
onMounted(() => {
  if (props.editTemplate) {
    // Load existing template
    blocks.value = JSON.parse(JSON.stringify(props.editTemplate.blocks))
    // Add any missing blocks as disabled
    for (const block of serviceBlocks) {
      if (!blocks.value.find(b => b.blockId === block.id)) {
        blocks.value.push(createDefaultBlockInstance(block))
      }
    }
  } else {
    // Initialize all blocks with defaults
    blocks.value = serviceBlocks.map(b => createDefaultBlockInstance(b))
  }
})

// Computed
const filteredBlocks = computed(() => {
  let result = serviceBlocks

  if (selectedCategory.value) {
    result = result.filter(b => b.category === selectedCategory.value)
  }

  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase()
    result = result.filter(b =>
      b.name.toLowerCase().includes(q) ||
      b.description.toLowerCase().includes(q)
    )
  }

  return result
})

const enabledBlocks = computed(() => {
  return blocks.value.filter(b => b.enabled)
})

const selectedBlock = computed(() => {
  if (!selectedBlockId.value) return null
  return getBlockById(selectedBlockId.value)
})

const selectedBlockInstance = computed(() => {
  if (!selectedBlockId.value) return null
  return blocks.value.find(b => b.blockId === selectedBlockId.value)
})

const validationResult = computed(() => {
  return validateBlocks(blocks.value)
})

const canProceed = computed(() => {
  return enabledBlocks.value.length > 0 && validationResult.value.valid
})

// Methods
function toggleBlock(blockId: string) {
  const instance = blocks.value.find(b => b.blockId === blockId)
  if (!instance) return

  const block = getBlockById(blockId)
  if (!block) return

  // Toggle
  instance.enabled = !instance.enabled

  // If enabling, auto-enable required dependencies
  if (instance.enabled && block.requires) {
    for (const req of block.requires) {
      const reqInstance = blocks.value.find(b => b.blockId === req)
      if (reqInstance && !reqInstance.enabled) {
        reqInstance.enabled = true
      }
    }
  }

  // If enabling, disable incompatible blocks
  if (instance.enabled && block.incompatibleWith) {
    for (const inc of block.incompatibleWith) {
      const incInstance = blocks.value.find(b => b.blockId === inc)
      if (incInstance) {
        incInstance.enabled = false
      }
    }
  }

  // Re-validate
  errors.value = validationResult.value.errors
}

function selectBlock(blockId: string) {
  selectedBlockId.value = selectedBlockId.value === blockId ? null : blockId
}

function updateConfig(key: string, value: any) {
  if (!selectedBlockInstance.value) return
  selectedBlockInstance.value.config[key] = value
}

function updateVersion(version: string) {
  if (!selectedBlockInstance.value) return
  selectedBlockInstance.value.version = version
}

function loadPreset(preset: typeof presetTemplates[0]) {
  templateName.value = preset.name
  templateDescription.value = preset.description

  // Reset all blocks
  blocks.value = serviceBlocks.map(b => createDefaultBlockInstance(b))

  // Apply preset
  for (const presetBlock of preset.blocks) {
    const instance = blocks.value.find(b => b.blockId === presetBlock.blockId)
    if (instance) {
      instance.enabled = presetBlock.enabled
      instance.version = presetBlock.version || instance.version
      instance.config = { ...instance.config, ...presetBlock.config }
    }
  }

  step.value = 'configure'
}

function nextStep() {
  if (step.value === 'select') {
    step.value = 'configure'
  } else if (step.value === 'configure') {
    if (!canProceed.value) {
      errors.value = validationResult.value.errors
      return
    }
    step.value = 'save'
  }
}

function prevStep() {
  if (step.value === 'configure') {
    step.value = 'select'
  } else if (step.value === 'save') {
    step.value = 'configure'
  }
}

function saveTemplate() {
  if (!templateName.value.trim()) {
    errors.value = ['Template name is required']
    return
  }

  const template: CustomTemplate = {
    id: props.editTemplate?.id || `custom-${Date.now()}`,
    name: templateName.value.trim(),
    description: templateDescription.value.trim(),
    blocks: blocks.value.filter(b => b.enabled),
    createdAt: props.editTemplate?.createdAt || new Date().toISOString(),
    updatedAt: new Date().toISOString()
  }

  emit('save', template)
}

function createProject() {
  if (!templateName.value.trim()) {
    errors.value = ['Template name is required']
    return
  }

  const template: CustomTemplate = {
    id: `custom-${Date.now()}`,
    name: templateName.value.trim(),
    description: templateDescription.value.trim(),
    blocks: blocks.value.filter(b => b.enabled),
    createdAt: new Date().toISOString(),
    updatedAt: new Date().toISOString()
  }

  emit('create-project', template)
}

function isBlockEnabled(blockId: string): boolean {
  return blocks.value.find(b => b.blockId === blockId)?.enabled ?? false
}

function isBlockIncompatible(block: ServiceBlock): boolean {
  if (!block.incompatibleWith) return false
  return block.incompatibleWith.some(inc => isBlockEnabled(inc))
}
</script>

<template>
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
    <div class="bg-gray-800 rounded-xl w-full max-w-6xl h-[90vh] flex flex-col">
      <!-- Header -->
      <div class="px-6 py-4 border-b border-gray-700 flex items-center justify-between">
        <div class="flex items-center gap-4">
          <button
            v-if="step !== 'select'"
            @click="prevStep"
            class="text-gray-400 hover:text-white"
          >
            ← Back
          </button>
          <h2 class="text-xl font-bold text-white">
            {{ step === 'select' ? 'Choose a Starting Point' : step === 'configure' ? 'Configure Services' : 'Save Template' }}
          </h2>
        </div>
        <div class="flex items-center gap-3">
          <!-- Step indicators -->
          <div class="flex items-center gap-2 mr-4">
            <div :class="['w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium', step === 'select' ? 'bg-blue-600 text-white' : 'bg-gray-600 text-gray-300']">1</div>
            <div class="w-8 h-0.5 bg-gray-600"></div>
            <div :class="['w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium', step === 'configure' ? 'bg-blue-600 text-white' : 'bg-gray-600 text-gray-300']">2</div>
            <div class="w-8 h-0.5 bg-gray-600"></div>
            <div :class="['w-8 h-8 rounded-full flex items-center justify-center text-sm font-medium', step === 'save' ? 'bg-blue-600 text-white' : 'bg-gray-600 text-gray-300']">3</div>
          </div>
          <button @click="emit('close')" class="text-gray-400 hover:text-white text-2xl">&times;</button>
        </div>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-hidden">
        <!-- Step 1: Select Preset or Start Fresh -->
        <div v-if="step === 'select'" class="h-full p-6 overflow-auto">
          <p class="text-gray-400 mb-6">Start from a preset template or build from scratch.</p>

          <!-- Presets -->
          <div class="grid grid-cols-3 gap-4 mb-8">
            <button
              v-for="preset in presetTemplates"
              :key="preset.name"
              @click="loadPreset(preset)"
              class="p-4 bg-gray-700 rounded-lg hover:bg-gray-600 transition-colors text-left group"
            >
              <div class="flex items-center justify-between mb-2">
                <span class="text-white font-medium group-hover:text-blue-400">{{ preset.name }}</span>
                <span class="text-xs text-gray-500">{{ preset.blocks.filter(b => b.enabled).length }} services</span>
              </div>
              <p class="text-sm text-gray-400 mb-3">{{ preset.description }}</p>
              <div class="flex flex-wrap gap-1">
                <span
                  v-for="block in preset.blocks.filter(b => b.enabled).slice(0, 5)"
                  :key="block.blockId"
                  class="text-xs px-2 py-0.5 bg-gray-800 rounded text-gray-300"
                >
                  {{ getBlockById(block.blockId)?.icon }} {{ getBlockById(block.blockId)?.name }}
                </span>
                <span v-if="preset.blocks.filter(b => b.enabled).length > 5" class="text-xs text-gray-500">
                  +{{ preset.blocks.filter(b => b.enabled).length - 5 }} more
                </span>
              </div>
            </button>
          </div>

          <!-- Start Fresh -->
          <div class="border-t border-gray-700 pt-6">
            <button
              @click="step = 'configure'"
              class="w-full p-4 border-2 border-dashed border-gray-600 rounded-lg hover:border-blue-500 hover:bg-gray-700/50 transition-colors text-center"
            >
              <span class="text-2xl mb-2 block">🛠️</span>
              <span class="text-white font-medium">Start from Scratch</span>
              <p class="text-sm text-gray-400 mt-1">Build your own custom stack</p>
            </button>
          </div>
        </div>

        <!-- Step 2: Configure Blocks -->
        <div v-if="step === 'configure'" class="h-full flex">
          <!-- Available Blocks -->
          <div class="w-1/2 border-r border-gray-700 flex flex-col">
            <div class="p-4 border-b border-gray-700">
              <input
                v-model="searchQuery"
                type="text"
                placeholder="Search services..."
                class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white placeholder-gray-400"
              />
              <!-- Category filters -->
              <div class="flex flex-wrap gap-2 mt-3">
                <button
                  @click="selectedCategory = null"
                  :class="['px-2 py-1 rounded text-xs', !selectedCategory ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600']"
                >
                  All
                </button>
                <button
                  v-for="cat in categories"
                  :key="cat.id"
                  @click="selectedCategory = cat.id"
                  :class="['px-2 py-1 rounded text-xs', selectedCategory === cat.id ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600']"
                >
                  {{ cat.icon }} {{ cat.label }}
                </button>
              </div>
            </div>

            <div class="flex-1 overflow-auto p-4">
              <div class="space-y-2">
                <div
                  v-for="block in filteredBlocks"
                  :key="block.id"
                  @click="selectBlock(block.id)"
                  :class="[
                    'p-3 rounded-lg cursor-pointer transition-all',
                    selectedBlockId === block.id ? 'bg-blue-600/30 border-2 border-blue-500' : 'bg-gray-700 hover:bg-gray-600 border-2 border-transparent',
                    isBlockIncompatible(block) ? 'opacity-50' : ''
                  ]"
                >
                  <div class="flex items-center justify-between">
                    <div class="flex items-center gap-3">
                      <span class="text-2xl">{{ block.icon }}</span>
                      <div>
                        <div class="flex items-center gap-2">
                          <span class="text-white font-medium">{{ block.name }}</span>
                          <span v-if="block.versions" class="text-xs text-gray-500">
                            v{{ blocks.find(b => b.blockId === block.id)?.version || block.defaultVersion }}
                          </span>
                        </div>
                        <p class="text-xs text-gray-400">{{ block.description }}</p>
                      </div>
                    </div>
                    <button
                      @click.stop="toggleBlock(block.id)"
                      :disabled="isBlockIncompatible(block) && !isBlockEnabled(block.id)"
                      :class="[
                        'px-3 py-1 rounded text-sm font-medium transition-colors',
                        isBlockEnabled(block.id)
                          ? 'bg-green-600 text-white hover:bg-green-700'
                          : 'bg-gray-600 text-gray-300 hover:bg-gray-500'
                      ]"
                    >
                      {{ isBlockEnabled(block.id) ? '✓ Added' : 'Add' }}
                    </button>
                  </div>

                  <!-- Requirements/Incompatibilities -->
                  <div v-if="block.requires || block.incompatibleWith" class="mt-2 flex flex-wrap gap-2">
                    <span v-if="block.requires" class="text-xs text-amber-400">
                      Requires: {{ block.requires.map(r => getBlockById(r)?.name).join(', ') }}
                    </span>
                    <span v-if="block.incompatibleWith && isBlockIncompatible(block)" class="text-xs text-red-400">
                      Incompatible with enabled services
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Configuration Panel -->
          <div class="w-1/2 flex flex-col">
            <div class="p-4 border-b border-gray-700">
              <h3 class="font-medium text-white">Your Stack</h3>
              <p class="text-sm text-gray-400">{{ enabledBlocks.length }} services selected</p>
            </div>

            <div class="flex-1 overflow-auto p-4">
              <!-- Selected block config -->
              <div v-if="selectedBlock && selectedBlockInstance" class="mb-6">
                <div class="flex items-center gap-2 mb-4">
                  <span class="text-2xl">{{ selectedBlock.icon }}</span>
                  <span class="text-lg font-medium text-white">{{ selectedBlock.name }} Configuration</span>
                </div>

                <!-- Version selector -->
                <div v-if="selectedBlock.versions" class="mb-4">
                  <label class="block text-sm text-gray-400 mb-1">Version</label>
                  <select
                    :value="selectedBlockInstance.version"
                    @change="updateVersion(($event.target as HTMLSelectElement).value)"
                    class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white"
                  >
                    <option v-for="v in selectedBlock.versions" :key="v" :value="v">{{ v }}</option>
                  </select>
                </div>

                <!-- Config options -->
                <div class="space-y-4">
                  <div v-for="opt in selectedBlock.configOptions" :key="opt.key">
                    <label class="block text-sm text-gray-400 mb-1">{{ opt.label }}</label>

                    <input
                      v-if="opt.type === 'text'"
                      type="text"
                      :value="selectedBlockInstance.config[opt.key]"
                      @input="updateConfig(opt.key, ($event.target as HTMLInputElement).value)"
                      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white"
                    />

                    <input
                      v-if="opt.type === 'number'"
                      type="number"
                      :value="selectedBlockInstance.config[opt.key]"
                      @input="updateConfig(opt.key, parseInt(($event.target as HTMLInputElement).value))"
                      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white"
                    />

                    <select
                      v-if="opt.type === 'select'"
                      :value="selectedBlockInstance.config[opt.key]"
                      @change="updateConfig(opt.key, ($event.target as HTMLSelectElement).value)"
                      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white"
                    >
                      <option v-for="o in opt.options" :key="o.value" :value="o.value">{{ o.label }}</option>
                    </select>

                    <label v-if="opt.type === 'checkbox'" class="flex items-center gap-2">
                      <input
                        type="checkbox"
                        :checked="selectedBlockInstance.config[opt.key]"
                        @change="updateConfig(opt.key, ($event.target as HTMLInputElement).checked)"
                        class="w-4 h-4 bg-gray-700 border-gray-600 rounded"
                      />
                      <span class="text-sm text-gray-300">Enabled</span>
                    </label>

                    <textarea
                      v-if="opt.type === 'textarea'"
                      :value="selectedBlockInstance.config[opt.key]"
                      @input="updateConfig(opt.key, ($event.target as HTMLTextAreaElement).value)"
                      rows="3"
                      class="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded text-white text-sm"
                    />

                    <p v-if="opt.description" class="text-xs text-gray-500 mt-1">{{ opt.description }}</p>
                  </div>
                </div>
              </div>

              <!-- Stack summary -->
              <div v-else>
                <h4 class="text-sm font-medium text-gray-400 mb-3">Selected Services</h4>
                <div v-if="enabledBlocks.length === 0" class="text-gray-500 text-center py-8">
                  <p class="text-4xl mb-2">📦</p>
                  <p>No services selected yet</p>
                  <p class="text-sm">Click "Add" on services to add them to your stack</p>
                </div>
                <div v-else class="space-y-2">
                  <div
                    v-for="instance in enabledBlocks"
                    :key="instance.blockId"
                    @click="selectBlock(instance.blockId)"
                    class="flex items-center justify-between p-2 bg-gray-700 rounded cursor-pointer hover:bg-gray-600"
                  >
                    <div class="flex items-center gap-2">
                      <span>{{ getBlockById(instance.blockId)?.icon }}</span>
                      <span class="text-white">{{ getBlockById(instance.blockId)?.name }}</span>
                      <span v-if="instance.version" class="text-xs text-gray-500">v{{ instance.version }}</span>
                    </div>
                    <button
                      @click.stop="toggleBlock(instance.blockId)"
                      class="text-red-400 hover:text-red-300 text-sm"
                    >
                      Remove
                    </button>
                  </div>
                </div>
              </div>

              <!-- Errors -->
              <div v-if="errors.length > 0" class="mt-4 p-3 bg-red-900/30 border border-red-700 rounded">
                <p class="text-red-400 font-medium text-sm mb-1">Configuration Issues:</p>
                <ul class="list-disc list-inside text-red-300 text-sm">
                  <li v-for="(error, i) in errors" :key="i">{{ error }}</li>
                </ul>
              </div>
            </div>

            <!-- Next button -->
            <div class="p-4 border-t border-gray-700">
              <button
                @click="nextStep"
                :disabled="!canProceed"
                :class="[
                  'w-full py-2 rounded font-medium transition-colors',
                  canProceed
                    ? 'bg-blue-600 text-white hover:bg-blue-700'
                    : 'bg-gray-600 text-gray-400 cursor-not-allowed'
                ]"
              >
                Continue →
              </button>
            </div>
          </div>
        </div>

        <!-- Step 3: Save Template -->
        <div v-if="step === 'save'" class="h-full p-6 overflow-auto">
          <div class="max-w-xl mx-auto">
            <div class="mb-6">
              <label class="block text-gray-400 mb-2">Template Name *</label>
              <input
                v-model="templateName"
                type="text"
                placeholder="My Custom Stack"
                class="w-full px-4 py-3 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400"
              />
            </div>

            <div class="mb-6">
              <label class="block text-gray-400 mb-2">Description</label>
              <textarea
                v-model="templateDescription"
                rows="3"
                placeholder="A brief description of this template..."
                class="w-full px-4 py-3 bg-gray-700 border border-gray-600 rounded-lg text-white placeholder-gray-400"
              />
            </div>

            <!-- Summary -->
            <div class="mb-6 p-4 bg-gray-700 rounded-lg">
              <h4 class="text-white font-medium mb-3">Stack Summary</h4>
              <div class="grid grid-cols-2 gap-2">
                <div
                  v-for="instance in enabledBlocks"
                  :key="instance.blockId"
                  class="flex items-center gap-2 text-sm"
                >
                  <span>{{ getBlockById(instance.blockId)?.icon }}</span>
                  <span class="text-gray-300">{{ getBlockById(instance.blockId)?.name }}</span>
                  <span v-if="instance.version" class="text-gray-500">v{{ instance.version }}</span>
                </div>
              </div>
            </div>

            <!-- Errors -->
            <div v-if="errors.length > 0" class="mb-6 p-3 bg-red-900/30 border border-red-700 rounded">
              <ul class="list-disc list-inside text-red-300 text-sm">
                <li v-for="(error, i) in errors" :key="i">{{ error }}</li>
              </ul>
            </div>

            <!-- Actions -->
            <div class="flex gap-3">
              <button
                @click="saveTemplate"
                class="flex-1 py-3 bg-gray-600 text-white rounded-lg hover:bg-gray-500 font-medium"
              >
                💾 Save as Template
              </button>
              <button
                @click="createProject"
                class="flex-1 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 font-medium"
              >
                🚀 Create Project Now
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
