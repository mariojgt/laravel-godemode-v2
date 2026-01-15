<template>
  <div class="card mt-6">
    <div class="flex items-center justify-between mb-4">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 bg-godmode-500/20 rounded-lg flex items-center justify-center">
          <CodeBracketIcon class="w-5 h-5 text-godmode-400" />
        </div>
        <h3 class="text-lg font-semibold text-dark-100">Laravel Controls</h3>
        <span class="badge bg-dark-700 text-dark-300">{{ project.name }}</span>
      </div>
      <div class="flex items-center gap-2">
        <span
          class="w-2 h-2 rounded-full"
          :class="overallStatus === 'healthy' ? 'bg-emerald-400' : 'bg-amber-400'"
        ></span>
        <span class="text-sm text-dark-400">
          {{ overallStatus === 'healthy' ? 'All Services Running' : 'Some Services Down' }}
        </span>
      </div>
    </div>

    <!-- Tabs -->
    <div class="flex border-b border-dark-700 mb-4 overflow-x-auto">
      <button
        v-for="tab in tabs"
        :key="tab.id"
        @click="activeTab = tab.id"
        class="tab whitespace-nowrap"
        :class="{ 'tab-active': activeTab === tab.id }"
      >
        {{ tab.name }}
      </button>
    </div>

    <!-- Tab Content -->
    <div class="min-h-[200px]">
      <!-- Services Tab -->
      <div v-if="activeTab === 'services'" class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <div
          v-for="service in displayServices"
          :key="service.name"
          class="p-4 bg-dark-700 rounded-xl"
        >
          <div class="flex items-center gap-2 mb-2">
            <span class="text-xl">{{ service.icon }}</span>
            <div>
              <div class="font-medium text-dark-100">{{ service.label }}</div>
              <div class="text-xs text-dark-400">{{ service.description }}</div>
            </div>
          </div>
          <span
            class="badge text-xs"
            :class="service.running ? 'badge-running' : 'badge-stopped'"
          >
            {{ service.running ? 'Running' : 'Stopped' }}
          </span>
        </div>
      </div>

      <!-- Artisan Tab -->
      <div v-else-if="activeTab === 'artisan'" class="space-y-4">
        <div class="flex gap-2">
          <input
            v-model="artisanCommand"
            type="text"
            class="input flex-1 font-mono"
            placeholder="Enter artisan command (e.g., migrate, make:model User)"
            @keyup.enter="runArtisan"
          >
          <button @click="runArtisan" class="btn btn-primary" :disabled="runningCommand">
            <PlayIcon class="w-4 h-4" />
            Run
          </button>
        </div>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="cmd in quickArtisanCommands"
            :key="cmd.command"
            @click="artisanCommand = cmd.command; runArtisan()"
            class="btn btn-sm btn-secondary"
          >
            {{ cmd.label }}
          </button>
        </div>
        <div v-if="commandOutput" class="terminal max-h-64">
          <pre>{{ commandOutput }}</pre>
        </div>
      </div>

      <!-- Queue Tab -->
      <div v-else-if="activeTab === 'queue'" class="space-y-4">
        <div class="flex gap-2">
          <button @click="startQueue" class="btn btn-success" :disabled="runningCommand">
            <PlayIcon class="w-4 h-4" />
            Start Worker
          </button>
          <button @click="stopQueue" class="btn btn-warning" :disabled="runningCommand">
            <StopIcon class="w-4 h-4" />
            Stop Worker
          </button>
          <button @click="retryFailed" class="btn btn-secondary" :disabled="runningCommand">
            <ArrowPathIcon class="w-4 h-4" />
            Retry Failed
          </button>
        </div>
        <div v-if="commandOutput" class="terminal max-h-64">
          <pre>{{ commandOutput }}</pre>
        </div>
      </div>

      <!-- Supervisor Tab -->
      <div v-else-if="activeTab === 'supervisor'" class="space-y-4">
        <h4 class="font-medium text-dark-200">Supervisor Management</h4>
        <div class="flex gap-2 mb-4">
          <button @click="reloadSupervisor" class="btn btn-success btn-sm" :disabled="runningCommand">
            <ArrowPathIcon class="w-4 h-4" />
            Reload Status
          </button>
          <button @click="saveSupervisorConfig" class="btn btn-warning btn-sm" :disabled="runningCommand">
            <DocumentIcon class="w-4 h-4" />
            Save Config
          </button>
          <button @click="restartSupervisor" class="btn btn-secondary btn-sm" :disabled="runningCommand">
            <ArrowPathIcon class="w-4 h-4" />
            Restart Supervisor
          </button>
        </div>

        <div class="grid grid-cols-4 gap-4">
          <div class="text-center p-4 bg-dark-700 rounded-xl">
            <div class="text-2xl font-bold text-godmode-400">{{ supervisorStatus.total_programs }}</div>
            <div class="text-xs text-dark-400">TOTAL PROGRAMS</div>
          </div>
          <div class="text-center p-4 bg-dark-700 rounded-xl">
            <div class="text-2xl font-bold text-emerald-400">{{ supervisorStatus.running }}</div>
            <div class="text-xs text-dark-400">RUNNING</div>
          </div>
          <div class="text-center p-4 bg-dark-700 rounded-xl">
            <div class="text-2xl font-bold text-dark-400">{{ supervisorStatus.stopped }}</div>
            <div class="text-xs text-dark-400">STOPPED</div>
          </div>
          <div class="text-center p-4 bg-dark-700 rounded-xl">
            <div class="text-2xl font-bold text-red-400">{{ supervisorStatus.failed }}</div>
            <div class="text-xs text-dark-400">FAILED</div>
          </div>
        </div>
      </div>

      <!-- Cache Tab -->
      <div v-else-if="activeTab === 'cache'" class="space-y-4">
        <div class="grid grid-cols-2 md:grid-cols-3 gap-3">
          <button
            v-for="cache in cacheCommands"
            :key="cache.type"
            @click="clearCache(cache.type)"
            class="btn btn-secondary justify-start"
            :disabled="runningCommand"
          >
            <TrashIcon class="w-4 h-4" />
            {{ cache.label }}
          </button>
        </div>
        <div class="flex gap-2">
          <button @click="optimizeApp" class="btn btn-success" :disabled="runningCommand">
            <BoltIcon class="w-4 h-4" />
            Optimize Application
          </button>
        </div>
        <div v-if="commandOutput" class="terminal max-h-64">
          <pre>{{ commandOutput }}</pre>
        </div>
      </div>

      <!-- Database Tab -->
      <div v-else-if="activeTab === 'database'" class="space-y-4">
        <div class="flex flex-wrap gap-2">
          <button @click="runMigrations" class="btn btn-primary" :disabled="runningCommand">
            <PlayIcon class="w-4 h-4" />
            Run Migrations
          </button>
          <button @click="runSeeders" class="btn btn-secondary" :disabled="runningCommand">
            <PlayIcon class="w-4 h-4" />
            Run Seeders
          </button>
          <button @click="freshDatabase" class="btn btn-danger" :disabled="runningCommand">
            <TrashIcon class="w-4 h-4" />
            Fresh Database
          </button>
        </div>
        <div v-if="commandOutput" class="terminal max-h-64">
          <pre>{{ commandOutput }}</pre>
        </div>
      </div>

      <!-- Logs Tab -->
      <div v-else-if="activeTab === 'logs'" class="space-y-4">
        <div class="flex gap-2 mb-4">
          <select v-model="selectedLogService" class="select w-48">
            <option value="app">App Container</option>
            <option value="nginx">Nginx</option>
            <option value="db">Database</option>
            <option value="redis">Redis</option>
          </select>
          <button @click="fetchLogs" class="btn btn-secondary" :disabled="loadingLogs">
            <ArrowPathIcon class="w-4 h-4" :class="{ 'animate-spin': loadingLogs }" />
            Refresh
          </button>
        </div>
        <div class="terminal max-h-96 overflow-auto">
          <pre v-if="logs">{{ logs }}</pre>
          <span v-else class="text-dark-400">No logs available. Click Refresh to load.</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { api } from '@/lib/api'
import type { Project, ServiceStatus, SupervisorStatus } from '@/lib/types'
import {
  CodeBracketIcon,
  PlayIcon,
  StopIcon,
  ArrowPathIcon,
  TrashIcon,
  BoltIcon,
  DocumentIcon
} from '@heroicons/vue/24/outline'

const props = defineProps<{
  project: Project
  servicesStatus: ServiceStatus[]
}>()

const activeTab = ref('services')
const artisanCommand = ref('')
const commandOutput = ref('')
const runningCommand = ref(false)
const logs = ref('')
const loadingLogs = ref(false)
const selectedLogService = ref('app')

const supervisorStatus = ref<SupervisorStatus>({
  total_programs: 0,
  running: 0,
  stopped: 0,
  failed: 0,
  programs: []
})

const tabs = [
  { id: 'services', name: 'Services' },
  { id: 'artisan', name: 'Artisan' },
  { id: 'queue', name: 'Queue' },
  { id: 'supervisor', name: 'Supervisor' },
  { id: 'cache', name: 'Cache' },
  { id: 'database', name: 'Database' },
  { id: 'logs', name: 'Logs' }
]

const quickArtisanCommands = [
  { label: 'Migrate', command: 'migrate' },
  { label: 'Seed', command: 'db:seed' },
  { label: 'Route List', command: 'route:list' },
  { label: 'Tinker', command: 'tinker' }
]

const cacheCommands = [
  { type: 'all', label: 'Clear All Caches' },
  { type: 'config', label: 'Clear Config' },
  { type: 'route', label: 'Clear Routes' },
  { type: 'view', label: 'Clear Views' },
  { type: 'cache', label: 'Clear Cache' }
]

const displayServices = computed(() => {
  const services = [
    { name: 'app', label: 'Application', description: 'Laravel Application', icon: '🚀', running: false },
    { name: 'db', label: 'Database', description: 'MySQL Database', icon: '🗄️', running: false },
    { name: 'redis', label: 'Cache', description: 'Redis Cache', icon: '⚡', running: false },
    { name: 'nginx', label: 'Web Server', description: 'Nginx Server', icon: '🌐', running: false }
  ]

  for (const service of services) {
    const status = props.servicesStatus.find(s => s.name.includes(service.name))
    if (status) {
      service.running = status.status === 'running'
    }
  }

  return services
})

const overallStatus = computed(() => {
  const running = displayServices.value.filter(s => s.running).length
  return running >= 3 ? 'healthy' : 'degraded'
})

onMounted(async () => {
  if (props.project.template === 'laravel' && props.project.status === 'running') {
    await loadSupervisorStatus()
  }
})

async function runArtisan() {
  if (!artisanCommand.value.trim()) return
  runningCommand.value = true
  try {
    commandOutput.value = await api.runArtisanCommand(props.project.id, artisanCommand.value)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function startQueue() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.startQueueWorker(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function stopQueue() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.stopQueueWorker(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function retryFailed() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.runArtisanCommand(props.project.id, 'queue:retry all')
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function clearCache(type: string) {
  runningCommand.value = true
  try {
    commandOutput.value = await api.clearCache(props.project.id, type)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function optimizeApp() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.optimizeApp(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function runMigrations() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.runMigrations(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function runSeeders() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.runSeeders(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function freshDatabase() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.freshDatabase(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function loadSupervisorStatus() {
  try {
    supervisorStatus.value = await api.getSupervisorStatus(props.project.id)
  } catch (e) {
    console.error('Failed to load supervisor status:', e)
  }
}

async function reloadSupervisor() {
  runningCommand.value = true
  try {
    await api.reloadSupervisor(props.project.id)
    await loadSupervisorStatus()
  } catch (e) {
    console.error('Failed to reload supervisor:', e)
  } finally {
    runningCommand.value = false
  }
}

async function saveSupervisorConfig() {
  runningCommand.value = true
  try {
    await api.reloadSupervisor(props.project.id)
  } catch (e) {
    console.error('Failed to save supervisor config:', e)
  } finally {
    runningCommand.value = false
  }
}

async function restartSupervisor() {
  runningCommand.value = true
  try {
    await api.restartSupervisor(props.project.id)
    await loadSupervisorStatus()
  } catch (e) {
    console.error('Failed to restart supervisor:', e)
  } finally {
    runningCommand.value = false
  }
}

async function fetchLogs() {
  loadingLogs.value = true
  try {
    logs.value = await api.getContainerLogs(props.project.id, selectedLogService.value, 200)
  } catch (e) {
    logs.value = `Error fetching logs: ${e}`
  } finally {
    loadingLogs.value = false
  }
}
</script>
