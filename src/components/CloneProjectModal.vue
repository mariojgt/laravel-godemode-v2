<template>
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="$emit('close')">
    <div class="bg-dark-800 rounded-2xl max-w-md w-full overflow-hidden">
      <!-- Header -->
      <div class="p-6 border-b border-dark-700">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold text-dark-100">Clone Project</h2>
          <button @click="$emit('close')" class="btn btn-ghost">
            <XMarkIcon class="w-5 h-5" />
          </button>
        </div>
        <p class="text-dark-400 text-sm mt-2">
          Create a copy of "{{ project.name }}" with a new name
        </p>
      </div>

      <!-- Content -->
      <div class="p-6 space-y-4">
        <div>
          <label class="block text-dark-200 font-medium mb-2">New Project Name</label>
          <input
            v-model="newName"
            type="text"
            class="input"
            placeholder="my-cloned-project"
            :class="{ 'border-red-500': nameError }"
            @keyup.enter="cloneProject"
          >
          <p v-if="nameError" class="text-red-400 text-sm mt-1">{{ nameError }}</p>
        </div>

        <div v-if="cloning" class="flex items-center gap-2 text-dark-300">
          <svg class="animate-spin h-5 w-5" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
          </svg>
          <span>Cloning project... This may take a moment.</span>
        </div>

        <div v-if="error" class="p-3 bg-red-500/20 border border-red-500/50 rounded-lg text-red-400 text-sm">
          {{ error }}
        </div>
      </div>

      <!-- Footer -->
      <div class="p-6 border-t border-dark-700 flex items-center justify-end gap-3">
        <button @click="$emit('close')" class="btn btn-ghost" :disabled="cloning">
          Cancel
        </button>
        <button @click="cloneProject" class="btn btn-primary" :disabled="cloning || !newName.trim()">
          📋 Clone Project
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { XMarkIcon } from '@heroicons/vue/24/outline'
import { api } from '@/lib/api'
import type { Project } from '@/lib/types'

const props = defineProps<{
  project: Project
}>()

const emit = defineEmits<{
  close: []
  cloned: [project: Project]
}>()

const newName = ref('')
const nameError = ref('')
const cloning = ref(false)
const error = ref('')

async function cloneProject() {
  if (!newName.value.trim()) {
    nameError.value = 'Please enter a project name'
    return
  }

  // Validate name (alphanumeric, hyphens, underscores)
  if (!/^[a-zA-Z0-9_-]+$/.test(newName.value)) {
    nameError.value = 'Name can only contain letters, numbers, hyphens, and underscores'
    return
  }

  nameError.value = ''
  error.value = ''
  cloning.value = true

  try {
    const clonedProject = await api.cloneProject(props.project.id, newName.value)
    emit('cloned', clonedProject)
  } catch (e: any) {
    error.value = e.message || String(e)
  } finally {
    cloning.value = false
  }
}
</script>
