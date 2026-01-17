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
        <div class="flex gap-2 flex-wrap">
          <button @click="startQueue" class="btn btn-success" :disabled="runningCommand">
            <PlayIcon class="w-4 h-4" />
            Start Worker
          </button>
          <button @click="stopQueue" class="btn btn-warning" :disabled="runningCommand">
            <StopIcon class="w-4 h-4" />
            Stop Worker
          </button>
          <button @click="restartQueue" class="btn btn-secondary" :disabled="runningCommand">
            <ArrowPathIcon class="w-4 h-4" />
            Restart Worker
          </button>
        </div>
        
        <!-- Failed Jobs Section -->
        <div class="border-t border-dark-700 pt-4">
          <div class="flex items-center justify-between mb-3">
            <h4 class="font-medium text-dark-200">Failed Jobs</h4>
            <div class="flex gap-2">
              <button @click="loadFailedJobs" class="btn btn-sm btn-secondary" :disabled="runningCommand">
                <ArrowPathIcon class="w-4 h-4" />
                Refresh
              </button>
              <button @click="retryAllFailed" class="btn btn-sm btn-primary" :disabled="runningCommand">
                Retry All
              </button>
              <button @click="clearAllFailed" class="btn btn-sm btn-danger" :disabled="runningCommand">
                Clear All
              </button>
            </div>
          </div>
          
          <div v-if="failedJobs.length > 0" class="space-y-2 max-h-64 overflow-auto">
            <div 
              v-for="job in failedJobs" 
              :key="job.id"
              class="flex items-center justify-between p-3 bg-dark-700 rounded-lg"
            >
              <div class="flex-1">
                <div class="font-mono text-sm text-dark-200">{{ job.payload?.displayName || job.id }}</div>
                <div class="text-xs text-dark-400">Failed at: {{ job.failed_at }}</div>
              </div>
              <button @click="retryJob(job.id)" class="btn btn-xs btn-primary">
                Retry
              </button>
            </div>
          </div>
          <div v-else class="text-center py-4 text-dark-400">
            No failed jobs
          </div>
        </div>
        
        <div v-if="commandOutput" class="terminal max-h-64">
          <pre>{{ commandOutput }}</pre>
        </div>
      </div>

      <!-- Scheduler Tab -->
      <div v-else-if="activeTab === 'scheduler'" class="space-y-4">
        <div class="flex gap-2 flex-wrap">
          <button @click="loadScheduledTasks" class="btn btn-secondary" :disabled="runningCommand">
            <ArrowPathIcon class="w-4 h-4" />
            Refresh Tasks
          </button>
          <button @click="runSchedulerNow" class="btn btn-primary" :disabled="runningCommand">
            <PlayIcon class="w-4 h-4" />
            Run Scheduler Now
          </button>
          <button @click="startSchedulerProcess" class="btn btn-success" :disabled="runningCommand">
            <PlayIcon class="w-4 h-4" />
            Start Scheduler
          </button>
          <button @click="stopSchedulerProcess" class="btn btn-warning" :disabled="runningCommand">
            <StopIcon class="w-4 h-4" />
            Stop Scheduler
          </button>
        </div>
        
        <div class="border-t border-dark-700 pt-4">
          <h4 class="font-medium text-dark-200 mb-3">Scheduled Tasks</h4>
          <div v-if="scheduledTasks" class="terminal max-h-96 overflow-auto">
            <pre>{{ scheduledTasks }}</pre>
          </div>
          <div v-else class="text-center py-4 text-dark-400">
            Click "Refresh Tasks" to load scheduled tasks
          </div>
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

      <!-- Backups Tab -->
      <div v-else-if="activeTab === 'backups'" class="space-y-4">
        <div class="flex gap-2 flex-wrap">
          <button @click="createBackup" class="btn btn-primary" :disabled="backupInProgress">
            <span v-if="backupInProgress" class="flex items-center">
              <svg class="animate-spin h-4 w-4 mr-2" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
              </svg>
              Creating...
            </span>
            <span v-else>💾 Create Backup</span>
          </button>
          <button @click="loadBackups" class="btn btn-secondary" :disabled="loadingBackups">
            <ArrowPathIcon class="w-4 h-4" :class="{ 'animate-spin': loadingBackups }" />
            Refresh
          </button>
        </div>
        
        <div class="border-t border-dark-700 pt-4">
          <h4 class="font-medium text-dark-200 mb-3">Available Backups</h4>
          
          <div v-if="loadingBackups" class="flex items-center justify-center py-8">
            <svg class="animate-spin h-6 w-6 text-godmode-500" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
            </svg>
          </div>
          
          <div v-else-if="backups.length === 0" class="text-center py-8 text-dark-400">
            No backups yet. Click "Create Backup" to create your first backup.
          </div>
          
          <div v-else class="space-y-2 max-h-64 overflow-auto">
            <div 
              v-for="backup in backups" 
              :key="backup.name"
              class="flex items-center justify-between p-3 bg-dark-700 rounded-lg"
            >
              <div class="flex-1">
                <div class="font-mono text-sm text-dark-200">{{ backup.name }}</div>
                <div class="flex gap-3 text-xs text-dark-400">
                  <span>📅 {{ backup.created_at }}</span>
                  <span>📦 {{ formatFileSize(backup.size) }}</span>
                </div>
              </div>
              <div class="flex gap-2">
                <button @click="restoreBackup(backup.name)" class="btn btn-xs btn-primary" :disabled="runningCommand">
                  ↩️ Restore
                </button>
                <button @click="deleteBackupFile(backup.name)" class="btn btn-xs btn-danger" :disabled="runningCommand">
                  🗑️
                </button>
              </div>
            </div>
          </div>
        </div>
        
        <div v-if="commandOutput" class="terminal max-h-64">
          <pre>{{ commandOutput }}</pre>
        </div>
      </div>

      <!-- Terminal Tab -->
      <div v-else-if="activeTab === 'terminal'" class="space-y-4">
        <div class="flex gap-2 items-center">
          <select v-model="selectedTerminalType" class="select w-40">
            <option value="bash">Bash Shell</option>
            <option value="tinker">PHP Tinker</option>
            <option value="composer">Composer</option>
            <option value="npm">NPM</option>
            <option value="artisan">Artisan</option>
          </select>
          <span class="text-dark-400 text-sm">
            {{ getTerminalPlaceholder() }}
          </span>
        </div>
        
        <div class="flex gap-2">
          <input
            v-model="terminalCommand"
            type="text"
            class="input flex-1 font-mono"
            :placeholder="getTerminalPlaceholder()"
            @keyup.enter="runTerminalCommand"
            @keyup.up="navigateHistory('up')"
            @keyup.down="navigateHistory('down')"
          >
          <button @click="runTerminalCommand" class="btn btn-primary" :disabled="runningCommand">
            <PlayIcon class="w-4 h-4" />
            Run
          </button>
        </div>
        
        <div class="flex flex-wrap gap-2">
          <template v-if="selectedTerminalType === 'bash'">
            <button @click="quickTerminalCommand('ls -la')" class="btn btn-xs btn-secondary">ls -la</button>
            <button @click="quickTerminalCommand('pwd')" class="btn btn-xs btn-secondary">pwd</button>
            <button @click="quickTerminalCommand('cat .env')" class="btn btn-xs btn-secondary">cat .env</button>
            <button @click="quickTerminalCommand('php -v')" class="btn btn-xs btn-secondary">php -v</button>
            <button @click="quickTerminalCommand('node -v')" class="btn btn-xs btn-secondary">node -v</button>
          </template>
          <template v-else-if="selectedTerminalType === 'tinker'">
            <button @click="quickTerminalCommand('User::count()')" class="btn btn-xs btn-secondary">User::count()</button>
            <button @click="quickTerminalCommand('DB::connection()->getDatabaseName()')" class="btn btn-xs btn-secondary">DB Name</button>
            <button @click="quickTerminalCommand(`config('app.name')`)" class="btn btn-xs btn-secondary">App Name</button>
          </template>
          <template v-else-if="selectedTerminalType === 'composer'">
            <button @click="quickTerminalCommand('install')" class="btn btn-xs btn-secondary">install</button>
            <button @click="quickTerminalCommand('update')" class="btn btn-xs btn-secondary">update</button>
            <button @click="quickTerminalCommand('dump-autoload')" class="btn btn-xs btn-secondary">dump-autoload</button>
            <button @click="quickTerminalCommand('show --outdated')" class="btn btn-xs btn-secondary">outdated</button>
          </template>
          <template v-else-if="selectedTerminalType === 'npm'">
            <button @click="quickTerminalCommand('install')" class="btn btn-xs btn-secondary">install</button>
            <button @click="quickTerminalCommand('run dev')" class="btn btn-xs btn-secondary">run dev</button>
            <button @click="quickTerminalCommand('run build')" class="btn btn-xs btn-secondary">run build</button>
            <button @click="quickTerminalCommand('outdated')" class="btn btn-xs btn-secondary">outdated</button>
          </template>
          <template v-else-if="selectedTerminalType === 'artisan'">
            <button @click="quickTerminalCommand('route:list')" class="btn btn-xs btn-secondary">route:list</button>
            <button @click="quickTerminalCommand('migrate:status')" class="btn btn-xs btn-secondary">migrate:status</button>
            <button @click="quickTerminalCommand('config:show app')" class="btn btn-xs btn-secondary">config:show</button>
            <button @click="quickTerminalCommand('about')" class="btn btn-xs btn-secondary">about</button>
          </template>
        </div>
        
        <div class="terminal min-h-48 max-h-96 overflow-auto">
          <pre v-if="terminalOutput">{{ terminalOutput }}</pre>
          <span v-else class="text-dark-400">Output will appear here...</span>
        </div>
        
        <div v-if="terminalHistory.length > 0" class="border-t border-dark-700 pt-2">
          <div class="text-xs text-dark-400 mb-1">Recent commands:</div>
          <div class="flex flex-wrap gap-1">
            <button 
              v-for="(cmd, i) in terminalHistory.slice(-5).reverse()" 
              :key="i"
              @click="terminalCommand = cmd"
              class="text-xs px-2 py-1 bg-dark-700 rounded text-dark-300 hover:bg-dark-600"
            >
              {{ cmd }}
            </button>
          </div>
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
import type { Project, ServiceStatus, SupervisorStatus, BackupInfo } from '@/lib/types'
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

const failedJobs = ref<any[]>([])
const scheduledTasks = ref('')

// Backups state
const backups = ref<BackupInfo[]>([])
const loadingBackups = ref(false)
const backupInProgress = ref(false)

// Terminal state
const terminalCommand = ref('')
const terminalOutput = ref('')
const terminalHistory = ref<string[]>([])
const selectedTerminalType = ref('bash')

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
  { id: 'terminal', name: 'Terminal' },
  { id: 'queue', name: 'Queue' },
  { id: 'scheduler', name: 'Scheduler' },
  { id: 'supervisor', name: 'Supervisor' },
  { id: 'cache', name: 'Cache' },
  { id: 'database', name: 'Database' },
  { id: 'backups', name: 'Backups' },
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

async function restartQueue() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.restartQueueWorker(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function loadFailedJobs() {
  runningCommand.value = true
  try {
    const result = await api.getFailedJobs(props.project.id)
    try {
      failedJobs.value = JSON.parse(result) || []
    } catch {
      failedJobs.value = []
      commandOutput.value = result
    }
  } catch (e) {
    commandOutput.value = `Error: ${e}`
    failedJobs.value = []
  } finally {
    runningCommand.value = false
  }
}

async function retryJob(jobId: string) {
  runningCommand.value = true
  try {
    commandOutput.value = await api.retryFailedJob(props.project.id, jobId)
    await loadFailedJobs()
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function retryAllFailed() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.retryAllFailedJobs(props.project.id)
    await loadFailedJobs()
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function clearAllFailed() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.clearFailedJobs(props.project.id)
    failedJobs.value = []
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

// Scheduler functions
async function loadScheduledTasks() {
  runningCommand.value = true
  try {
    scheduledTasks.value = await api.getScheduledTasks(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function runSchedulerNow() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.runScheduler(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function startSchedulerProcess() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.startScheduler(props.project.id)
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function stopSchedulerProcess() {
  runningCommand.value = true
  try {
    commandOutput.value = await api.stopScheduler(props.project.id)
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

// ============ Backup Functions ============

async function loadBackups() {
  loadingBackups.value = true
  try {
    backups.value = await api.getBackupsWithInfo(props.project.id)
  } catch (e) {
    console.error('Failed to load backups:', e)
    backups.value = []
  } finally {
    loadingBackups.value = false
  }
}

async function createBackup() {
  backupInProgress.value = true
  commandOutput.value = ''
  try {
    const result = await api.backupDatabase(props.project.id)
    commandOutput.value = result
    await loadBackups()
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    backupInProgress.value = false
  }
}

async function restoreBackup(backupName: string) {
  if (!confirm(`Are you sure you want to restore from "${backupName}"? This will overwrite the current database.`)) {
    return
  }
  
  runningCommand.value = true
  commandOutput.value = ''
  try {
    const result = await api.restoreDatabase(props.project.id, backupName)
    commandOutput.value = result
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

async function deleteBackupFile(backupName: string) {
  if (!confirm(`Are you sure you want to delete "${backupName}"?`)) {
    return
  }
  
  runningCommand.value = true
  try {
    await api.deleteBackup(props.project.id, backupName)
    await loadBackups()
  } catch (e) {
    commandOutput.value = `Error: ${e}`
  } finally {
    runningCommand.value = false
  }
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

// ============ Terminal Functions ============

let historyIndex = -1

async function runTerminalCommand() {
  if (!terminalCommand.value.trim()) return
  
  // Add to history
  terminalHistory.value.push(terminalCommand.value)
  historyIndex = -1
  
  runningCommand.value = true
  terminalOutput.value = `$ ${terminalCommand.value}\n\nRunning...\n`
  
  try {
    let result: string
    
    switch (selectedTerminalType.value) {
      case 'tinker':
        result = await api.runTinkerCommand(props.project.id, terminalCommand.value)
        break
      case 'composer':
        result = await api.runComposerCommand(props.project.id, terminalCommand.value)
        break
      case 'npm':
        result = await api.runNpmCommand(props.project.id, terminalCommand.value)
        break
      case 'artisan':
        result = await api.runArtisanCommand(props.project.id, terminalCommand.value)
        break
      default: // bash
        result = await api.execContainerCommand(props.project.id, 'app', terminalCommand.value)
    }
    
    terminalOutput.value = `$ ${terminalCommand.value}\n\n${result}`
  } catch (e) {
    terminalOutput.value = `$ ${terminalCommand.value}\n\nError: ${e}`
  } finally {
    runningCommand.value = false
    terminalCommand.value = ''
  }
}

function quickTerminalCommand(cmd: string) {
  terminalCommand.value = cmd
  runTerminalCommand()
}

function navigateHistory(direction: 'up' | 'down') {
  if (terminalHistory.value.length === 0) return
  
  if (direction === 'up') {
    if (historyIndex < terminalHistory.value.length - 1) {
      historyIndex++
      terminalCommand.value = terminalHistory.value[terminalHistory.value.length - 1 - historyIndex]
    }
  } else {
    if (historyIndex > 0) {
      historyIndex--
      terminalCommand.value = terminalHistory.value[terminalHistory.value.length - 1 - historyIndex]
    } else {
      historyIndex = -1
      terminalCommand.value = ''
    }
  }
}

function getTerminalPlaceholder(): string {
  switch (selectedTerminalType.value) {
    case 'tinker': return 'PHP code (e.g., User::count())'
    case 'composer': return 'composer command (e.g., install, update)'
    case 'npm': return 'npm command (e.g., install, run dev)'
    case 'artisan': return 'artisan command (e.g., migrate, route:list)'
    default: return 'bash command (e.g., ls -la)'
  }
}
</script>
