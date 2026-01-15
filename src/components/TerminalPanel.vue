<template>
  <div class="terminal-panel" :class="{ 'terminal-expanded': expanded }">
    <!-- Terminal Header -->
    <div 
      class="terminal-header"
      @click="expanded = !expanded"
    >
      <div class="flex items-center gap-2">
        <span class="text-godmode-400">⌘</span>
        <span class="font-medium text-dark-100">Terminal Output</span>
        <span v-if="logs.length > 0" class="text-xs text-dark-400">({{ logs.length }} entries)</span>
      </div>
      <div class="flex items-center gap-2">
        <button 
          @click.stop="clearLogs" 
          class="p-1 hover:bg-dark-700 rounded text-dark-400 hover:text-dark-200"
          title="Clear"
        >
          <TrashIcon class="w-4 h-4" />
        </button>
        <button 
          class="p-1 hover:bg-dark-700 rounded text-dark-400 hover:text-dark-200"
          :title="expanded ? 'Collapse' : 'Expand'"
        >
          <span v-if="expanded">▲</span>
          <span v-else>▼</span>
        </button>
      </div>
    </div>

    <!-- Terminal Content -->
    <div v-show="expanded" class="terminal-content" ref="terminalContent">
      <div v-if="logs.length === 0" class="text-dark-500 text-sm">
        No output yet. Start a project to see logs here.
      </div>
      <div v-else class="space-y-2">
        <div 
          v-for="(log, index) in logs" 
          :key="index"
          class="terminal-entry"
          :class="getLogClass(log.type)"
        >
          <div class="terminal-entry-header">
            <span class="terminal-timestamp">{{ formatTime(log.timestamp) }}</span>
            <span class="terminal-action">{{ log.action }}</span>
            <span v-if="log.project" class="terminal-project">{{ log.project }}</span>
          </div>
          <pre class="terminal-output">{{ log.output }}</pre>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { TrashIcon } from '@heroicons/vue/24/outline'

export interface TerminalLog {
  timestamp: Date
  action: string
  project?: string
  output: string
  type: 'info' | 'success' | 'error' | 'warning'
}

const props = defineProps<{
  logs: TerminalLog[]
}>()

const emit = defineEmits(['clear'])

const expanded = ref(true)
const terminalContent = ref<HTMLElement | null>(null)

// Auto-scroll to bottom when new logs arrive and auto-expand
watch(() => props.logs.length, async (newLen, oldLen) => {
  // Auto-expand when new logs arrive
  if (newLen > oldLen) {
    expanded.value = true
  }
  
  await nextTick()
  if (terminalContent.value) {
    terminalContent.value.scrollTop = terminalContent.value.scrollHeight
  }
})

function clearLogs() {
  emit('clear')
}

function formatTime(date: Date): string {
  return date.toLocaleTimeString('en-US', { 
    hour: '2-digit', 
    minute: '2-digit', 
    second: '2-digit' 
  })
}

function getLogClass(type: string): string {
  switch (type) {
    case 'success': return 'terminal-success'
    case 'error': return 'terminal-error'
    case 'warning': return 'terminal-warning'
    default: return 'terminal-info'
  }
}
</script>

<style scoped>
.terminal-panel {
  @apply bg-dark-900 border-t-2 border-godmode-600 flex flex-col;
  height: 48px;
  transition: height 0.2s ease-in-out;
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 50;
}

.terminal-expanded {
  height: 350px;
}

.terminal-header {
  @apply flex items-center justify-between px-4 py-3 cursor-pointer hover:bg-dark-800 border-b border-dark-700;
}

.terminal-content {
  @apply flex-1 overflow-auto p-4 font-mono text-sm bg-dark-950;
}

.terminal-entry {
  @apply p-3 rounded-lg border;
}

.terminal-entry-header {
  @apply flex items-center gap-2 mb-2 text-xs;
}

.terminal-timestamp {
  @apply text-dark-500;
}

.terminal-action {
  @apply font-semibold uppercase text-godmode-400;
}

.terminal-project {
  @apply text-dark-300 bg-dark-700 px-2 py-0.5 rounded;
}

.terminal-output {
  @apply text-dark-200 whitespace-pre-wrap text-xs overflow-x-auto;
}

.terminal-info {
  @apply border-dark-700 bg-dark-900;
}

.terminal-success {
  @apply border-emerald-700/50 bg-emerald-900/20;
}

.terminal-success .terminal-action {
  @apply text-emerald-400;
}

.terminal-error {
  @apply border-red-700/50 bg-red-900/20;
}

.terminal-error .terminal-action {
  @apply text-red-400;
}

.terminal-warning {
  @apply border-amber-700/50 bg-amber-900/20;
}

.terminal-warning .terminal-action {
  @apply text-amber-400;
}
</style>
