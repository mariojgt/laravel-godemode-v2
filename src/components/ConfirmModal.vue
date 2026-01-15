<template>
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4" @click.self="$emit('cancel')">
    <div class="bg-dark-800 rounded-2xl max-w-md w-full overflow-hidden">
      <div class="p-6">
        <div class="flex items-center gap-3 mb-4">
          <div class="w-10 h-10 bg-red-500/20 rounded-full flex items-center justify-center">
            <ExclamationTriangleIcon class="w-6 h-6 text-red-400" />
          </div>
          <h2 class="text-xl font-bold text-dark-100">{{ title }}</h2>
        </div>
        <p class="text-dark-300">{{ message }}</p>
      </div>
      <div class="p-6 border-t border-dark-700 flex justify-end gap-3">
        <button @click="$emit('cancel')" class="btn btn-secondary">Cancel</button>
        <button @click="$emit('confirm')" class="btn btn-danger">{{ confirmText }}</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ExclamationTriangleIcon } from '@heroicons/vue/24/outline'

defineProps<{
  title: string
  message: string
  confirmText: string
}>()

defineEmits(['confirm', 'cancel'])
</script>
