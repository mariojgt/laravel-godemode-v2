<template>
  <div class="h-full overflow-auto p-6">
    <h1 class="text-2xl font-bold text-dark-100 mb-6">Settings</h1>

    <div class="max-w-2xl space-y-6">
      <!-- General Settings -->
      <div class="card">
        <h2 class="text-lg font-semibold text-dark-100 mb-4">General</h2>

        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <div>
              <label class="text-dark-200 font-medium">Auto-start projects</label>
              <p class="text-sm text-dark-400">Automatically start projects when the app launches</p>
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" v-model="settings.auto_start_projects" class="sr-only peer">
              <div class="w-11 h-6 bg-dark-600 peer-focus:ring-2 peer-focus:ring-godmode-500 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-godmode-500"></div>
            </label>
          </div>
        </div>
      </div>

      <!-- Development Settings -->
      <div class="card">
        <h2 class="text-lg font-semibold text-dark-100 mb-4">Development</h2>

        <div class="space-y-4">
          <div>
            <label class="block text-dark-200 font-medium mb-2">Preferred Code Editor</label>
            <select v-model="settings.preferred_editor" class="select">
              <option value="code">Visual Studio Code</option>
              <option value="cursor">Cursor</option>
              <option value="phpstorm">PHPStorm</option>
              <option value="sublime">Sublime Text</option>
              <option value="atom">Atom</option>
            </select>
          </div>

          <div>
            <label class="block text-dark-200 font-medium mb-2">Projects Directory</label>
            <div class="flex gap-2">
              <input
                type="text"
                v-model="settings.projects_path"
                class="input flex-1 font-mono text-sm"
                readonly
              >
              <button @click="selectProjectsDir" class="btn btn-secondary">
                <FolderIcon class="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Default Versions -->
      <div class="card">
        <h2 class="text-lg font-semibold text-dark-100 mb-4">🔧 Runtime Versions</h2>

        <div class="space-y-4">
          <div>
            <label class="block text-dark-200 font-medium mb-2">PHP Version</label>
            <p class="text-sm text-dark-400 mb-2">Choose your preferred PHP version</p>
            <select v-model="settings.default_php_version" class="select">
              <option value="8.5">PHP 8.5 (Latest)</option>
              <option value="8.4">PHP 8.4 (Recommended)</option>
              <option value="8.3">PHP 8.3</option>
              <option value="8.2">PHP 8.2</option>
            </select>
          </div>

          <div>
            <label class="block text-dark-200 font-medium mb-2">NODE Version</label>
            <p class="text-sm text-dark-400 mb-2">Choose your preferred Node version</p>
            <select v-model="settings.default_node_version" class="select">
              <option value="21">Node.js 21 (Latest)</option>
              <option value="20">Node.js 20 LTS</option>
              <option value="18">Node.js 18 LTS (Recommended)</option>
              <option value="16">Node.js 16 LTS</option>
            </select>
          </div>
        </div>
      </div>

      <!-- Docker Info -->
      <div class="card">
        <h2 class="text-lg font-semibold text-dark-100 mb-4">🐳 Docker</h2>

        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <span class="text-dark-300">Docker Status</span>
            <span
              class="badge"
              :class="dockerInstalled ? 'badge-running' : 'badge-error'"
            >
              {{ dockerInstalled ? 'Installed' : 'Not Found' }}
            </span>
          </div>
          <div v-if="dockerVersion" class="flex items-center justify-between">
            <span class="text-dark-300">Version</span>
            <span class="text-dark-100 font-mono text-sm">{{ dockerVersion }}</span>
          </div>
        </div>
      </div>

      <!-- Save Button -->
      <div class="flex justify-end gap-3">
        <button @click="resetSettings" class="btn btn-secondary">
          Reset to Defaults
        </button>
        <button @click="saveSettings" class="btn btn-primary" :disabled="saving">
          <svg v-if="saving" class="animate-spin w-4 h-4" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
          </svg>
          {{ saving ? 'Saving...' : 'Save Settings' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { api } from '@/lib/api'
import { useSettingsStore } from '@/stores/settings'
import { storeToRefs } from 'pinia'
import { FolderIcon } from '@heroicons/vue/24/outline'

const settingsStore = useSettingsStore()
const { settings } = storeToRefs(settingsStore)

const dockerInstalled = ref(false)
const dockerVersion = ref('')
const saving = ref(false)

onMounted(async () => {
  if (!settingsStore.loaded) {
    await settingsStore.loadSettings()
  }

  try {
    dockerInstalled.value = await api.checkDockerInstalled()
    if (dockerInstalled.value) {
      dockerVersion.value = await api.getDockerVersion()
    }
  } catch (e) {
    console.error('Failed to check Docker:', e)
  }
})

// Auto-save when settings change
watch(settings, async () => {
  saving.value = true
  await settingsStore.saveSettings()
  saving.value = false
}, { deep: true })

async function saveSettings() {
  saving.value = true
  await settingsStore.saveSettings()
  saving.value = false
}

function resetSettings() {
  settings.value.projects_path = ''
  settings.value.auto_start_projects = false
  settings.value.preferred_editor = 'code'
  settings.value.default_php_version = '8.4'
  settings.value.default_node_version = '18'
  settings.value.theme = 'dark'
}

async function selectProjectsDir() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Projects Directory'
    })
    if (selected && typeof selected === 'string') {
      settings.value.projects_path = selected
    }
  } catch (e) {
    console.error('Failed to select directory:', e)
  }
}
</script>
