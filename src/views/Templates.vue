<template>
  <div class="h-full overflow-auto p-6">
    <!-- Header with tabs -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-4">
        <h1 class="text-2xl font-bold text-dark-100">Templates</h1>
        <div class="flex bg-dark-700 rounded-lg p-1">
          <button
            @click="activeTab = 'builtin'"
            :class="['px-4 py-1.5 rounded-md text-sm font-medium transition-colors', activeTab === 'builtin' ? 'bg-godmode-500 text-white' : 'text-dark-400 hover:text-dark-200']"
          >
            Built-in
          </button>
          <button
            @click="activeTab = 'custom'"
            :class="['px-4 py-1.5 rounded-md text-sm font-medium transition-colors', activeTab === 'custom' ? 'bg-godmode-500 text-white' : 'text-dark-400 hover:text-dark-200']"
          >
            Custom ({{ customTemplates.length }})
          </button>
        </div>
      </div>
      <button
        @click="showBuilder = true"
        class="btn btn-primary flex items-center gap-2"
      >
        <span class="text-lg">🛠️</span>
        Build Custom Template
      </button>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <div class="flex items-center gap-3 text-dark-400">
        <svg class="animate-spin h-8 w-8" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
        </svg>
        <span>Loading templates...</span>
      </div>
    </div>

    <!-- Built-in Templates Grid -->
    <div v-else-if="activeTab === 'builtin'" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="template in templates"
        :key="template.template_type"
        class="card card-hover cursor-pointer group"
        @click="selectTemplate(template)"
      >
        <div class="flex flex-col items-center text-center py-6">
          <!-- Icon -->
          <div class="w-16 h-16 bg-dark-700 rounded-2xl flex items-center justify-center mb-4 group-hover:bg-godmode-500/20 transition-colors">
            <component
              :is="getTemplateIcon(template.template_type)"
              class="w-8 h-8 text-dark-300 group-hover:text-godmode-400 transition-colors"
            />
          </div>

          <!-- Name -->
          <h3 class="text-lg font-semibold text-dark-100 mb-2">{{ template.name }}</h3>

          <!-- Description -->
          <p class="text-sm text-dark-400 mb-4 px-2">{{ template.description }}</p>

          <!-- Category Badge -->
          <span class="badge bg-dark-700 text-dark-300">{{ template.category }}</span>
        </div>
      </div>
    </div>

    <!-- Custom Templates Grid -->
    <div v-else-if="activeTab === 'custom'">
      <!-- Empty State -->
      <div v-if="customTemplates.length === 0" class="flex flex-col items-center justify-center h-64 text-center">
        <span class="text-5xl mb-4">🛠️</span>
        <h3 class="text-lg font-medium text-dark-200 mb-2">No Custom Templates Yet</h3>
        <p class="text-dark-400 mb-4 max-w-md">
          Create your own Docker stack templates with the services you need. Mix and match databases, caches, search engines, and more!
        </p>
        <button @click="showBuilder = true" class="btn btn-primary">
          Build Your First Template
        </button>
      </div>

      <!-- Custom Templates List -->
      <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div
          v-for="custom in customTemplates"
          :key="custom.id"
          class="card group relative"
        >
          <div class="flex flex-col py-4">
            <div class="flex items-start justify-between mb-3">
              <div>
                <h3 class="text-lg font-semibold text-dark-100">{{ custom.name }}</h3>
                <p class="text-xs text-dark-500">
                  Created {{ formatDate(custom.createdAt) }}
                </p>
              </div>
              <div class="flex gap-1">
                <button @click="editCustomTemplate(custom)" class="p-1.5 text-dark-400 hover:text-godmode-400 rounded" title="Edit">
                  ✏️
                </button>
                <button @click="deleteCustomTemplate(custom.id)" class="p-1.5 text-dark-400 hover:text-red-400 rounded" title="Delete">
                  🗑️
                </button>
              </div>
            </div>

            <p class="text-sm text-dark-400 mb-4">{{ custom.description || 'No description' }}</p>

            <!-- Services preview -->
            <div class="flex flex-wrap gap-1.5 mb-4">
              <span
                v-for="block in custom.blocks.slice(0, 6)"
                :key="block.blockId"
                class="text-xs px-2 py-1 bg-dark-700 rounded text-dark-300"
              >
                {{ getBlockIcon(block.blockId) }} {{ getBlockName(block.blockId) }}
              </span>
              <span v-if="custom.blocks.length > 6" class="text-xs px-2 py-1 text-dark-500">
                +{{ custom.blocks.length - 6 }} more
              </span>
            </div>

            <button
              @click="useCustomTemplate(custom)"
              class="btn btn-primary w-full mt-auto"
            >
              Use This Template
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Template Details Modal -->
    <div v-if="selectedTemplate" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="selectedTemplate = null">
      <div class="bg-dark-800 rounded-2xl max-w-2xl w-full max-h-[80vh] overflow-hidden">
        <div class="p-6 border-b border-dark-700">
          <div class="flex items-center justify-between">
            <h2 class="text-xl font-bold text-dark-100">{{ selectedTemplate.name }} Template</h2>
            <button @click="selectedTemplate = null" class="btn btn-ghost">
              <XMarkIcon class="w-5 h-5" />
            </button>
          </div>
        </div>

        <div class="p-6 overflow-auto max-h-[60vh]" v-if="templateDetails">
          <!-- Versions -->
          <div class="mb-6" v-if="templateDetails.versions">
            <h3 class="text-sm font-medium text-godmode-400 mb-3">🔧 Runtime Versions</h3>
            <div class="space-y-3">
              <div v-if="templateDetails.versions.php" class="flex items-center justify-between">
                <span class="text-dark-300">PHP Version</span>
                <span class="text-dark-100 font-mono">{{ templateDetails.versions.php.default }}</span>
              </div>
              <div v-if="templateDetails.versions.node" class="flex items-center justify-between">
                <span class="text-dark-300">Node Version</span>
                <span class="text-dark-100 font-mono">{{ templateDetails.versions.node.default }}</span>
              </div>
            </div>
          </div>

          <!-- Package Managers -->
          <div class="mb-6" v-if="templateDetails.packageManagers">
            <h3 class="text-sm font-medium text-godmode-400 mb-3">📦 Package Managers</h3>
            <div class="flex flex-wrap gap-2">
              <span
                v-for="(pm, key) in templateDetails.packageManagers"
                :key="key"
                class="badge"
                :class="pm.default || pm.required ? 'bg-godmode-500/20 text-godmode-400' : 'bg-dark-700 text-dark-400'"
              >
                {{ key }}
                <span v-if="pm.required" class="text-xs">(required)</span>
              </span>
            </div>
          </div>

          <!-- Services -->
          <div class="mb-6" v-if="templateDetails.services">
            <h3 class="text-sm font-medium text-godmode-400 mb-3">🐳 Services</h3>
            <div class="grid grid-cols-2 gap-3">
              <div
                v-for="(service, key) in templateDetails.services"
                :key="key"
                class="flex items-center gap-2 p-3 bg-dark-700 rounded-lg"
              >
                <span>{{ service.icon || '📦' }}</span>
                <div>
                  <div class="text-sm text-dark-100">{{ key }}</div>
                  <div class="text-xs text-dark-400">Port: {{ service.defaultPort || 'N/A' }}</div>
                </div>
              </div>
            </div>
          </div>

          <!-- Ports -->
          <div v-if="templateDetails.ports">
            <h3 class="text-sm font-medium text-godmode-400 mb-3">🔌 Default Ports</h3>
            <div class="flex flex-wrap gap-2">
              <div v-for="(port, key) in templateDetails.ports" :key="key" class="port-badge">
                <span class="port-badge-label">{{ key }}:</span>
                <span class="port-badge-value">{{ port.default }}</span>
              </div>
            </div>
          </div>
        </div>

        <div class="p-6 border-t border-dark-700 flex justify-end gap-3">
          <button @click="selectedTemplate = null" class="btn btn-secondary">Cancel</button>
          <router-link to="/" class="btn btn-primary" @click="selectedTemplate = null">
            Use This Template
          </router-link>
        </div>
      </div>
    </div>

    <!-- Template Builder Modal -->
    <TemplateBuilder
      v-if="showBuilder"
      :editTemplate="editingTemplate"
      @close="closeBuilder"
      @save="handleSaveTemplate"
      @create-project="handleCreateFromTemplate"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { api } from '@/lib/api'
import type { Template } from '@/lib/types'
import type { CustomTemplate } from '@/lib/blocks'
import { getBlockById } from '@/lib/blocks'
import { XMarkIcon, CodeBracketIcon, ServerIcon } from '@heroicons/vue/24/outline'
import TemplateBuilder from '@/components/TemplateBuilder.vue'

const router = useRouter()

const templates = ref<Template[]>([])
const customTemplates = ref<CustomTemplate[]>([])
const selectedTemplate = ref<Template | null>(null)
const templateDetails = ref<any>(null)
const loading = ref(true)
const activeTab = ref<'builtin' | 'custom'>('builtin')
const showBuilder = ref(false)
const editingTemplate = ref<CustomTemplate | undefined>(undefined)

// Local storage key for custom templates
const CUSTOM_TEMPLATES_KEY = 'laravel-godmode-custom-templates'

onMounted(async () => {
  // Load custom templates from localStorage
  loadCustomTemplates()

  try {
    templates.value = await api.getTemplates()
  } catch (e) {
    console.error('Failed to load templates:', e)
    // Fallback to hardcoded templates if API fails
    templates.value = [
      {
        name: 'Laravel',
        description: 'Full-stack PHP framework with Eloquent ORM, Blade templating, and Artisan CLI',
        icon: '🅻',
        template_type: 'laravel',
        category: 'Backend Framework'
      },
      {
        name: 'Node.js',
        description: 'JavaScript runtime with Express.js framework for building APIs and web applications',
        icon: '⬢',
        template_type: 'nodejs',
        category: 'Backend Runtime'
      }
    ]
  } finally {
    loading.value = false
  }
})

function loadCustomTemplates() {
  try {
    const stored = localStorage.getItem(CUSTOM_TEMPLATES_KEY)
    if (stored) {
      customTemplates.value = JSON.parse(stored)
    }
  } catch (e) {
    console.error('Failed to load custom templates:', e)
  }
}

function saveCustomTemplates() {
  try {
    localStorage.setItem(CUSTOM_TEMPLATES_KEY, JSON.stringify(customTemplates.value))
  } catch (e) {
    console.error('Failed to save custom templates:', e)
  }
}

async function selectTemplate(template: Template) {
  selectedTemplate.value = template
  try {
    templateDetails.value = await api.getTemplate(template.template_type)
  } catch (e) {
    console.error('Failed to load template details:', e)
  }
}

function getTemplateIcon(type: string) {
  switch (type) {
    case 'laravel':
      return CodeBracketIcon
    case 'nodejs':
      return ServerIcon
    default:
      return CodeBracketIcon
  }
}

function getBlockIcon(blockId: string): string {
  return getBlockById(blockId)?.icon || '📦'
}

function getBlockName(blockId: string): string {
  return getBlockById(blockId)?.name || blockId
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })
}

function closeBuilder() {
  showBuilder.value = false
  editingTemplate.value = undefined
}

function handleSaveTemplate(template: CustomTemplate) {
  const existingIndex = customTemplates.value.findIndex(t => t.id === template.id)
  if (existingIndex >= 0) {
    customTemplates.value[existingIndex] = template
  } else {
    customTemplates.value.push(template)
  }
  saveCustomTemplates()
  closeBuilder()
  activeTab.value = 'custom'
}

function handleCreateFromTemplate(template: CustomTemplate) {
  // Save template first
  handleSaveTemplate(template)
  // Navigate to home with the template to use
  // Store in sessionStorage so Home.vue can pick it up
  sessionStorage.setItem('use-custom-template', JSON.stringify(template))
  router.push('/')
}

function editCustomTemplate(template: CustomTemplate) {
  editingTemplate.value = template
  showBuilder.value = true
}

function deleteCustomTemplate(id: string) {
  if (confirm('Are you sure you want to delete this template?')) {
    customTemplates.value = customTemplates.value.filter(t => t.id !== id)
    saveCustomTemplates()
  }
}

function useCustomTemplate(template: CustomTemplate) {
  sessionStorage.setItem('use-custom-template', JSON.stringify(template))
  router.push('/')
}
</script>
