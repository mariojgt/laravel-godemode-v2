<template>
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="$emit('close')">
    <div class="bg-dark-800 rounded-2xl max-w-2xl w-full max-h-[90vh] overflow-hidden">
      <!-- Header -->
      <div class="p-6 border-b border-dark-700">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold text-dark-100">Create New Project</h2>
          <button @click="$emit('close')" class="btn btn-ghost">
            <XMarkIcon class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="p-6 overflow-auto max-h-[60vh]">
        <!-- Step 1: Project Name & Template -->
        <div class="space-y-6">
          <!-- Project Name -->
          <div>
            <label class="block text-dark-200 font-medium mb-2">Project Name</label>
            <input
              type="text"
              v-model="projectName"
              class="input"
              placeholder="my-awesome-project"
              :class="{ 'border-red-500': nameError }"
            >
            <p v-if="nameError" class="text-red-400 text-sm mt-1">{{ nameError }}</p>
          </div>

          <!-- Template Selection -->
          <div>
            <label class="block text-dark-200 font-medium mb-2">Template</label>
            <div class="grid grid-cols-2 gap-4">
              <button
                v-for="template in templates"
                :key="template.template_type"
                @click="selectedTemplate = template.template_type"
                class="p-4 rounded-xl border-2 transition-all text-left"
                :class="selectedTemplate === template.template_type
                  ? 'border-godmode-500 bg-godmode-500/10'
                  : 'border-dark-600 hover:border-dark-500'"
              >
                <div class="flex items-center gap-3">
                  <div class="w-10 h-10 bg-dark-700 rounded-lg flex items-center justify-center">
                    <component :is="getTemplateIcon(template.template_type)" class="w-5 h-5 text-godmode-400" />
                  </div>
                  <div>
                    <div class="font-medium text-dark-100">{{ template.name }}</div>
                    <div class="text-xs text-dark-400">{{ template.category }}</div>
                  </div>
                </div>
              </button>
            </div>
          </div>

          <!-- Runtime Versions -->
          <div class="grid grid-cols-2 gap-4">
            <div v-if="selectedTemplate === 'laravel'">
              <label class="block text-dark-200 font-medium mb-2">PHP Version</label>
              <select v-model="config.php_version" class="select">
                <option value="8.5">PHP 8.5 (Latest)</option>
                <option value="8.4">PHP 8.4 (Recommended)</option>
                <option value="8.3">PHP 8.3</option>
              </select>
            </div>
            <div>
              <label class="block text-dark-200 font-medium mb-2">Node Version</label>
              <select v-model="config.node_version" class="select">
                <option value="21">Node.js 21</option>
                <option value="20">Node.js 20 LTS</option>
                <option value="18">Node.js 18 LTS (Recommended)</option>
              </select>
            </div>
          </div>

          <!-- Package Managers -->
          <div>
            <label class="block text-dark-200 font-medium mb-3">📦 Package Managers</label>
            <div class="flex flex-wrap gap-3">
              <label class="flex items-center gap-2 cursor-pointer">
                <input type="checkbox" v-model="config.install_bun" class="form-checkbox rounded bg-dark-700 border-dark-500 text-godmode-500 focus:ring-godmode-500">
                <span class="text-dark-200">Bun</span>
                <span class="text-xs text-dark-400">(Fast runtime)</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input type="checkbox" v-model="config.install_pnpm" class="form-checkbox rounded bg-dark-700 border-dark-500 text-godmode-500 focus:ring-godmode-500">
                <span class="text-dark-200">pnpm</span>
                <span class="text-xs text-dark-400">(Disk efficient)</span>
              </label>
              <label class="flex items-center gap-2 cursor-pointer">
                <input type="checkbox" v-model="config.install_yarn" class="form-checkbox rounded bg-dark-700 border-dark-500 text-godmode-500 focus:ring-godmode-500">
                <span class="text-dark-200">Yarn</span>
              </label>
            </div>
          </div>

          <!-- Laravel Options -->
          <div v-if="selectedTemplate === 'laravel'">
            <label class="block text-dark-200 font-medium mb-3">🚀 Laravel Options</label>
            <label class="flex items-center gap-2 p-3 bg-dark-700 rounded-lg cursor-pointer">
              <input type="checkbox" v-model="config.install_laravel" class="form-checkbox rounded bg-dark-600 border-dark-500 text-godmode-500 focus:ring-godmode-500">
              <span class="text-dark-200">Install Fresh Laravel</span>
              <span class="text-xs text-dark-400">(via Composer after containers start)</span>
            </label>
          </div>

          <!-- Services -->
          <div>
            <label class="block text-dark-200 font-medium mb-3">🐳 Services</label>
            <div class="grid grid-cols-2 gap-3">
              <label class="flex items-center gap-2 p-3 bg-dark-700 rounded-lg cursor-pointer">
                <input type="checkbox" v-model="config.services.mysql" class="form-checkbox rounded bg-dark-600 border-dark-500 text-godmode-500 focus:ring-godmode-500">
                <span>🗄️ MySQL</span>
              </label>
              <label class="flex items-center gap-2 p-3 bg-dark-700 rounded-lg cursor-pointer">
                <input type="checkbox" v-model="config.services.redis" class="form-checkbox rounded bg-dark-600 border-dark-500 text-godmode-500 focus:ring-godmode-500">
                <span>🔴 Redis</span>
              </label>
              <label class="flex items-center gap-2 p-3 bg-dark-700 rounded-lg cursor-pointer">
                <input type="checkbox" v-model="config.services.phpmyadmin" class="form-checkbox rounded bg-dark-600 border-dark-500 text-godmode-500 focus:ring-godmode-500">
                <span>🔧 phpMyAdmin</span>
              </label>
              <label class="flex items-center gap-2 p-3 bg-dark-700 rounded-lg cursor-pointer">
                <input type="checkbox" v-model="config.services.mailhog" class="form-checkbox rounded bg-dark-600 border-dark-500 text-godmode-500 focus:ring-godmode-500">
                <span>📧 Mailhog</span>
              </label>
            </div>
          </div>

          <!-- Ports -->
          <div>
            <label class="block text-dark-200 font-medium mb-3">🔌 Ports</label>
            <div class="grid grid-cols-3 gap-3">
              <div>
                <label class="text-xs text-dark-400 mb-1 block">App Port</label>
                <input type="number" v-model.number="config.ports.app" class="input text-center font-mono">
              </div>
              <div>
                <label class="text-xs text-dark-400 mb-1 block">Vite Port</label>
                <input type="number" v-model.number="config.ports.vite" class="input text-center font-mono">
              </div>
              <div>
                <label class="text-xs text-dark-400 mb-1 block">Database Port</label>
                <input type="number" v-model.number="config.ports.db" class="input text-center font-mono">
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="p-6 border-t border-dark-700 flex justify-end gap-3">
        <button @click="$emit('close')" class="btn btn-secondary">Cancel</button>
        <button @click="createProject" class="btn btn-primary" :disabled="creating || !isValid">
          <svg v-if="creating" class="animate-spin w-4 h-4" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
          </svg>
          {{ creating ? 'Creating...' : 'Create Project' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { api } from '@/lib/api'
import type { Template, ProjectConfig } from '@/lib/types'
import { XMarkIcon, CodeBracketIcon, ServerIcon } from '@heroicons/vue/24/outline'

const emit = defineEmits(['close', 'created'])

const templates = ref<Template[]>([])
const projectName = ref('')
const selectedTemplate = ref('laravel')
const nameError = ref('')
const creating = ref(false)

const config = reactive<ProjectConfig>({
  php_version: '8.4',
  node_version: '18',
  install_bun: true,
  install_pnpm: false,
  install_yarn: false,
  install_laravel: true,
  services: {
    mysql: true,
    redis: true,
    phpmyadmin: false,
    mailhog: false,
    nginx: true
  },
  ports: {
    app: 8000,
    vite: 5173,
    db: 3306,
    redis: 6379,
    phpmyadmin: 8080,
    mailhog: 8025
  }
})

const isValid = computed(() => {
  return projectName.value.trim().length > 0 && selectedTemplate.value
})

onMounted(async () => {
  try {
    templates.value = await api.getTemplates()
  } catch (e) {
    // Fallback templates
    templates.value = [
      { name: 'Laravel', template_type: 'laravel', category: 'Backend Framework', description: '', icon: '' },
      { name: 'Node.js', template_type: 'nodejs', category: 'Backend Runtime', description: '', icon: '' }
    ]
  }
})

function validateName() {
  const name = projectName.value.trim()
  if (!name) {
    nameError.value = 'Project name is required'
    return false
  }
  if (!/^[a-z0-9-_]+$/i.test(name)) {
    nameError.value = 'Only letters, numbers, dashes and underscores allowed'
    return false
  }
  nameError.value = ''
  return true
}

async function createProject() {
  if (!validateName()) return

  creating.value = true
  try {
    await api.createProject(projectName.value.trim(), selectedTemplate.value, config)
    emit('created')
  } catch (e) {
    console.error('Failed to create project:', e)
    nameError.value = e as string
  } finally {
    creating.value = false
  }
}

function getTemplateIcon(type: string) {
  return type === 'laravel' ? CodeBracketIcon : ServerIcon
}
</script>
