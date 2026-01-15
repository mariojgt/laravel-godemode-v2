<template>
  <div class="h-full overflow-auto p-6">
    <h1 class="text-2xl font-bold text-dark-100 mb-6">Available Templates</h1>

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

    <!-- Templates Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { api } from '@/lib/api'
import type { Template } from '@/lib/types'
import { XMarkIcon, CodeBracketIcon, ServerIcon } from '@heroicons/vue/24/outline'

const templates = ref<Template[]>([])
const selectedTemplate = ref<Template | null>(null)
const templateDetails = ref<any>(null)
const loading = ref(true)

onMounted(async () => {
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
</script>
