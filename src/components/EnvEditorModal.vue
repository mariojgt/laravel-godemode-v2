<template>
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="$emit('close')">
    <div class="bg-dark-800 rounded-2xl w-[90vw] max-w-6xl h-[85vh] overflow-hidden flex flex-col">
      <!-- Header -->
      <div class="p-6 border-b border-dark-700">
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-bold text-dark-100">
            Edit .env - {{ projectName }}
          </h2>
          <button @click="$emit('close')" class="btn btn-ghost">
            <XMarkIcon class="w-5 h-5" />
          </button>
        </div>
      </div>

      <!-- Content -->
      <div class="flex-1 p-6 overflow-hidden flex flex-col">
        <div v-if="loading" class="flex items-center justify-center py-12">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-godmode-500"></div>
          <span class="ml-3 text-dark-300">Loading .env file...</span>
        </div>

        <div v-else-if="error" class="text-center py-12">
          <div class="text-red-400 mb-2">Failed to load .env file</div>
          <div class="text-dark-400 text-sm">{{ error }}</div>
        </div>

        <div v-else class="flex-1 flex flex-col min-h-0">
          <textarea
            v-model="envContent"
            class="flex-1 w-full bg-dark-900 border border-dark-600 rounded-xl p-4 text-dark-100 font-mono text-sm resize-none focus:outline-none focus:border-godmode-500 transition-colors"
            placeholder="# Environment variables..."
            spellcheck="false"
          ></textarea>
          <p class="text-dark-400 text-xs mt-2">
            Changes will be applied after saving. You may need to restart the containers for some changes to take effect.
          </p>
        </div>
      </div>

      <!-- Footer -->
      <div class="p-6 border-t border-dark-700 flex items-center justify-end gap-3">
        <button @click="$emit('close')" class="btn btn-ghost">
          Cancel
        </button>
        <button
          @click="saveEnv"
          :disabled="saving || loading"
          class="btn btn-primary"
        >
          <span v-if="saving" class="flex items-center">
            <span class="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></span>
            Saving...
          </span>
          <span v-else>Save Changes</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { XMarkIcon } from '@heroicons/vue/24/outline'
import { api } from '@/lib/api'

const props = defineProps<{
  projectId: string
  projectName: string
}>()

const emit = defineEmits<{
  close: []
  saved: []
}>()

const envContent = ref('')
const loading = ref(true)
const saving = ref(false)
const error = ref('')

onMounted(async () => {
  await loadEnv()
})

async function loadEnv() {
  loading.value = true
  error.value = ''
  try {
    envContent.value = await api.getProjectEnv(props.projectId)
  } catch (err: any) {
    console.error('Failed to load .env:', err)
    error.value = err.message || String(err)
  } finally {
    loading.value = false
  }
}

async function saveEnv() {
  saving.value = true
  try {
    await api.updateProjectEnv(props.projectId, envContent.value)
    emit('saved')
    emit('close')
  } catch (err: any) {
    console.error('Failed to save .env:', err)
    error.value = err.message || String(err)
  } finally {
    saving.value = false
  }
}
</script>
