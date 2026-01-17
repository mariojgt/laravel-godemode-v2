<template>
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="$emit('close')">
    <div class="bg-dark-800 rounded-2xl max-w-lg w-full overflow-hidden">
      <!-- Header -->
      <div class="p-6 border-b border-dark-700">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold text-dark-100">Import Existing Project</h2>
          <button @click="$emit('close')" class="btn btn-ghost">
            <XMarkIcon class="w-5 h-5" />
          </button>
        </div>
        <p class="text-dark-400 text-sm mt-2">
          Import an existing Laravel project into Laravel God Mode
        </p>
      </div>

      <!-- Content -->
      <div class="p-6 space-y-4">
        <div>
          <label class="block text-dark-200 font-medium mb-2">Project Name</label>
          <input
            v-model="projectName"
            type="text"
            class="input"
            placeholder="my-imported-project"
            :class="{ 'border-red-500': nameError }"
          >
          <p v-if="nameError" class="text-red-400 text-sm mt-1">{{ nameError }}</p>
        </div>

        <div>
          <label class="block text-dark-200 font-medium mb-2">Source Path</label>
          <div class="flex gap-2">
            <input
              v-model="sourcePath"
              type="text"
              class="input flex-1 font-mono text-sm"
              placeholder="/path/to/your/laravel-project"
              :class="{ 'border-red-500': pathError }"
            >
            <button @click="browseFolder" class="btn btn-secondary">
              <FolderIcon class="w-4 h-4" />
              Browse
            </button>
          </div>
          <p v-if="pathError" class="text-red-400 text-sm mt-1">{{ pathError }}</p>
          <p class="text-dark-400 text-xs mt-1">
            Select the root folder of your Laravel project (containing artisan file)
          </p>
        </div>

        <div v-if="importing" class="flex items-center gap-2 text-dark-300">
          <svg class="animate-spin h-5 w-5" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
          </svg>
          <span>Importing project... This may take a moment.</span>
        </div>

        <div v-if="error" class="p-3 bg-red-500/20 border border-red-500/50 rounded-lg text-red-400 text-sm">
          {{ error }}
        </div>

        <div class="p-4 bg-dark-700 rounded-xl">
          <h4 class="font-medium text-dark-200 mb-2">What happens when importing:</h4>
          <ul class="text-sm text-dark-400 space-y-1">
            <li>• Your project files will be copied to the God Mode projects folder</li>
            <li>• Docker configuration will be generated automatically</li>
            <li>• Original project remains untouched</li>
            <li>• You may need to update .env settings after import</li>
          </ul>
        </div>
      </div>

      <!-- Footer -->
      <div class="p-6 border-t border-dark-700 flex items-center justify-end gap-3">
        <button @click="$emit('close')" class="btn btn-ghost" :disabled="importing">
          Cancel
        </button>
        <button
          @click="importProject"
          class="btn btn-primary"
          :disabled="importing || !projectName.trim() || !sourcePath.trim()"
        >
          📥 Import Project
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { XMarkIcon, FolderIcon } from '@heroicons/vue/24/outline'
import { open } from '@tauri-apps/plugin-dialog'
import { api } from '@/lib/api'
import type { Project } from '@/lib/types'

const emit = defineEmits<{
  close: []
  imported: [project: Project]
}>()

const projectName = ref('')
const sourcePath = ref('')
const nameError = ref('')
const pathError = ref('')
const importing = ref(false)
const error = ref('')

async function browseFolder() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Laravel Project Folder'
    })

    if (selected && typeof selected === 'string') {
      sourcePath.value = selected

      // Auto-fill project name from folder name if empty
      if (!projectName.value) {
        const parts = selected.split('/')
        projectName.value = parts[parts.length - 1] || ''
      }
    }
  } catch (e) {
    console.error('Failed to open folder dialog:', e)
  }
}

async function importProject() {
  // Validate inputs
  nameError.value = ''
  pathError.value = ''
  error.value = ''

  if (!projectName.value.trim()) {
    nameError.value = 'Please enter a project name'
    return
  }

  if (!/^[a-zA-Z0-9_-]+$/.test(projectName.value)) {
    nameError.value = 'Name can only contain letters, numbers, hyphens, and underscores'
    return
  }

  if (!sourcePath.value.trim()) {
    pathError.value = 'Please select a source folder'
    return
  }

  importing.value = true

  try {
    const importedProject = await api.importProject(sourcePath.value, projectName.value)
    emit('imported', importedProject)
  } catch (e: any) {
    error.value = e.message || String(e)
  } finally {
    importing.value = false
  }
}
</script>
