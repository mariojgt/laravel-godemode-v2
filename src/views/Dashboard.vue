<template>
  <div class="h-full flex flex-col overflow-hidden">
    <!-- Header -->
    <div class="flex items-center justify-between p-6 border-b border-dark-700 shrink-0">
      <h1 class="text-2xl font-bold text-dark-100">Project Dashboard</h1>
      <div class="flex gap-2">
        <button @click="showImportModal = true" class="btn btn-secondary">
          📥 Import Project
        </button>
        <button @click="showCreateModal = true" class="btn btn-primary">
          <PlusIcon class="w-5 h-5" />
          Create Project
        </button>
      </div>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-auto p-6 pb-96">
      <!-- Loading State -->
      <div v-if="store.loading" class="flex items-center justify-center h-64">
        <div class="flex items-center gap-3 text-dark-400">
          <svg class="animate-spin h-8 w-8" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
          </svg>
          <span>Loading projects...</span>
        </div>
      </div>

      <!-- Empty State -->
      <div v-else-if="store.projects.length === 0" class="flex flex-col items-center justify-center h-64 text-center">
        <div class="w-16 h-16 bg-dark-700 rounded-full flex items-center justify-center mb-4">
          <FolderIcon class="w-8 h-8 text-dark-400" />
        </div>
        <h3 class="text-lg font-medium text-dark-200 mb-2">No projects yet</h3>
        <p class="text-dark-400 mb-4">Create your first Laravel or Node.js project to get started.</p>
        <button @click="showCreateModal = true" class="btn btn-primary">
          <PlusIcon class="w-5 h-5" />
          Create Project
        </button>
      </div>

      <!-- Projects Grid -->
      <div v-else class="space-y-6">
        <!-- Project Cards -->
        <div v-for="project in store.projects" :key="project.id"
             class="card card-hover cursor-pointer"
             :class="{ 'ring-2 ring-godmode-500': store.selectedProjectId === project.id }"
             @click="selectProject(project)">
          <div class="flex items-start justify-between mb-4">
            <div>
              <div class="flex items-center gap-3">
                <h3 class="text-lg font-semibold text-dark-100">{{ project.name }}</h3>
                <span :class="getStatusBadgeClass(project.status)" class="badge">
                  <span class="w-2 h-2 rounded-full" :class="getStatusDotClass(project.status)"></span>
                  {{ project.status.toUpperCase() }}
                </span>
              </div>
              <p class="text-sm text-dark-400 mt-1">Template: {{ project.template }}</p>
              <p class="text-xs text-dark-500 mt-1">Created: {{ formatDate(project.created_at) }}</p>
              <p class="text-xs text-dark-500 font-mono mt-1">📁 {{ project.path }}</p>
            </div>
          </div>

          <!-- Ports -->
          <div class="flex flex-wrap gap-2 mb-4">
            <div class="port-badge">
              <span class="port-badge-label">app:</span>
              <span class="port-badge-value">{{ project.config.ports.app }}</span>
            </div>
            <div class="port-badge">
              <span class="port-badge-label">vite:</span>
              <span class="port-badge-value">{{ project.config.ports.vite }}</span>
            </div>
            <div v-if="project.config.services.phpmyadmin" class="port-badge">
              <span class="port-badge-label">phpmyadmin:</span>
              <span class="port-badge-value">{{ project.config.ports.phpmyadmin }}</span>
            </div>
            <div v-if="project.config.services.mailhog" class="port-badge">
              <span class="port-badge-label">mailhog:</span>
              <span class="port-badge-value">{{ project.config.ports.mailhog }}</span>
            </div>
            <div v-if="project.config.services.redis" class="port-badge">
              <span class="port-badge-label">redis:</span>
              <span class="port-badge-value">{{ project.config.ports.redis }}</span>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex items-center gap-2">
            <button
              v-if="project.status === 'stopped' || project.status === 'error'"
              @click.stop="startProject(project.id)"
              :disabled="actionLoading[project.id]"
              class="btn btn-success btn-sm"
            >
              <PlayIcon class="w-4 h-4" />
              Start
            </button>
            <button
              v-else-if="project.status === 'running'"
              @click.stop="stopProject(project.id)"
              :disabled="actionLoading[project.id]"
              class="btn btn-warning btn-sm"
            >
              <StopIcon class="w-4 h-4" />
              Stop
            </button>
            <button
              v-else
              disabled
              class="btn btn-secondary btn-sm"
            >
              <svg class="animate-spin w-4 h-4" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
              </svg>
              {{ project.status }}...
            </button>

            <button
              @click.stop="rebuildProject(project.id)"
              :disabled="actionLoading[project.id]"
              class="btn btn-warning btn-sm"
            >
              <ArrowPathIcon class="w-4 h-4" />
              Rebuild
            </button>

            <button
              @click.stop="openInEditor(project.id)"
              class="btn btn-primary btn-sm"
            >
              <CodeBracketIcon class="w-4 h-4" />
              Open in {{ getEditorName(settingsStore.settings.preferred_editor) }}
            </button>

            <button
              @click.stop="openEnvModal(project)"
              class="btn btn-secondary btn-sm"
            >
              Edit .env
            </button>

            <button
              @click.stop="openCloneModal(project)"
              class="btn btn-secondary btn-sm"
            >
              📋 Clone
            </button>

            <button
              @click.stop="confirmDelete(project)"
              class="btn btn-danger btn-sm"
            >
              <TrashIcon class="w-4 h-4" />
              Delete
            </button>
          </div>
        </div>

        <!-- Project Controls Panel -->
        <ProjectControls
          v-if="store.selectedProject"
          :project="store.selectedProject"
          :services-status="store.servicesStatus[store.selectedProject.id] || []"
        />
      </div>
    </div>

    <!-- Create Project Modal -->
    <CreateProjectModal
      v-if="showCreateModal"
      @close="showCreateModal = false"
      @created="onProjectCreated"
    />

    <!-- Delete Confirmation Modal -->
    <ConfirmModal
      v-if="projectToDelete"
      title="Delete Project"
      :message="`Are you sure you want to delete '${projectToDelete.name}'? This will stop all containers and optionally delete all files.`"
      confirm-text="Delete"
      @confirm="deleteProject"
      @cancel="projectToDelete = null"
    />

    <!-- Env Editor Modal -->
    <EnvEditorModal
      v-if="envProject"
      :project-id="envProject.id"
      :project-name="envProject.name"
      @close="envProject = null"
      @saved="envProject = null"
    />

    <!-- Clone Project Modal -->
    <CloneProjectModal
      v-if="cloneProject"
      :project="cloneProject"
      @close="cloneProject = null"
      @cloned="onProjectCloned"
    />

    <!-- Import Project Modal -->
    <ImportProjectModal
      v-if="showImportModal"
      @close="showImportModal = false"
      @imported="onProjectImported"
    />

    <!-- Custom Template Project Modal -->
    <div v-if="showCustomTemplateModal && pendingCustomTemplate" class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div class="bg-dark-800 rounded-2xl max-w-lg w-full overflow-hidden">
        <div class="p-6 border-b border-dark-700">
          <h2 class="text-xl font-bold text-dark-100">Create Project from Template</h2>
          <p class="text-sm text-dark-400 mt-1">Using: {{ pendingCustomTemplate.name }}</p>
        </div>

        <div class="p-6">
          <div class="mb-4">
            <label class="block text-dark-300 mb-2">Project Name *</label>
            <input
              v-model="customProjectName"
              type="text"
              placeholder="my-awesome-project"
              class="w-full px-4 py-2 bg-dark-700 border border-dark-600 rounded-lg text-dark-100 placeholder-dark-500"
              :disabled="customTemplateCreating"
            />
          </div>

          <div class="mb-4 p-3 bg-dark-700 rounded-lg">
            <h4 class="text-sm font-medium text-dark-300 mb-2">Included Services</h4>
            <div class="flex flex-wrap gap-1">
              <span
                v-for="block in pendingCustomTemplate.blocks"
                :key="block.blockId"
                class="text-xs px-2 py-1 bg-dark-600 rounded text-dark-200"
              >
                {{ block.blockId }}
              </span>
            </div>
          </div>
        </div>

        <div class="p-6 border-t border-dark-700 flex justify-end gap-3">
          <button
            @click="showCustomTemplateModal = false; pendingCustomTemplate = null"
            class="btn btn-secondary"
            :disabled="customTemplateCreating"
          >
            Cancel
          </button>
          <button
            @click="createFromCustomTemplate"
            class="btn btn-primary"
            :disabled="!customProjectName.trim() || customTemplateCreating"
          >
            <span v-if="customTemplateCreating" class="flex items-center gap-2">
              <svg class="animate-spin h-4 w-4" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
              </svg>
              Creating...
            </span>
            <span v-else>Create Project</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Terminal Panel -->
    <TerminalPanel
      :logs="terminalLogs"
      @clear="terminalLogs = []"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, reactive } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useProjectsStore } from '@/stores/projects'
import { useSettingsStore } from '@/stores/settings'
import { api } from '@/lib/api'
import type { Project } from '@/lib/types'
import CreateProjectModal from '@/components/CreateProjectModal.vue'
import ProjectControls from '@/components/ProjectControls.vue'
import ConfirmModal from '@/components/ConfirmModal.vue'
import TerminalPanel from '@/components/TerminalPanel.vue'
import EnvEditorModal from '@/components/EnvEditorModal.vue'
import CloneProjectModal from '@/components/CloneProjectModal.vue'
import ImportProjectModal from '@/components/ImportProjectModal.vue'
import type { TerminalLog } from '@/components/TerminalPanel.vue'
import {
  PlusIcon,
  FolderIcon,
  PlayIcon,
  StopIcon,
  ArrowPathIcon,
  TrashIcon,
  CodeBracketIcon
} from '@heroicons/vue/24/outline'

interface DockerOutputEvent {
  project_id: string
  line: string
  stream_type: string
}

const store = useProjectsStore()
const settingsStore = useSettingsStore()
const showCreateModal = ref(false)
const showImportModal = ref(false)
const showCustomTemplateModal = ref(false)
const pendingCustomTemplate = ref<any>(null)
const customProjectName = ref('')
const customTemplateCreating = ref(false)
const projectToDelete = ref<Project | null>(null)
const envProject = ref<Project | null>(null)
const cloneProject = ref<Project | null>(null)
const actionLoading = reactive<Record<string, boolean>>({})
const terminalLogs = ref<TerminalLog[]>([])
let unlistenDockerOutput: UnlistenFn | null = null

function addLog(action: string, project: string, output: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
  terminalLogs.value.push({
    timestamp: new Date(),
    action,
    project,
    output,
    type
  })
}

// Get project name by ID
function getProjectName(projectId: string): string {
  const project = store.projects.find(p => p.id === projectId)
  return project?.name || projectId
}

onMounted(async () => {
  store.fetchProjects()

  // Load settings if not already loaded
  if (!settingsStore.loaded) {
    await settingsStore.loadSettings()
  }

  // Check if we have a custom template to use from Templates page
  const customTemplateJson = sessionStorage.getItem('use-custom-template')
  if (customTemplateJson) {
    sessionStorage.removeItem('use-custom-template')
    try {
      pendingCustomTemplate.value = JSON.parse(customTemplateJson)
      showCustomTemplateModal.value = true
    } catch (e) {
      console.error('Failed to parse custom template:', e)
    }
  }

  // Listen for docker output events (real-time streaming)
  unlistenDockerOutput = await listen<DockerOutputEvent>('docker-output', (event) => {
    const { project_id, line, stream_type } = event.payload
    const projectName = getProjectName(project_id)

    let logType: 'info' | 'success' | 'error' | 'warning' = 'info'
    if (stream_type === 'stderr') {
      logType = 'warning'
    } else if (stream_type === 'status') {
      if (line.includes('✓')) {
        logType = 'success'
      } else if (line.includes('✗')) {
        logType = 'error'
      }
    }

    addLog('output', projectName, line, logType)
  })
})

onUnmounted(() => {
  if (unlistenDockerOutput) {
    unlistenDockerOutput()
  }
})

function selectProject(project: Project) {
  store.selectProject(project.id)
}

async function startProject(projectId: string) {
  actionLoading[projectId] = true
  const project = store.projects.find(p => p.id === projectId)
  const projectName = project?.name || projectId

  addLog('starting', projectName, 'Starting Docker containers...', 'info')

  try {
    // Use streaming version for real-time output
    const output = await api.startProjectStreaming(projectId)
    addLog('completed', projectName, output || 'Project started successfully', 'success')
    await store.fetchProjects()
    await store.refreshProjectStatus(projectId)
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e)
    addLog('error', projectName, `Failed to start: ${errorMsg}`, 'error')
    console.error('Failed to start project:', e)
  } finally {
    actionLoading[projectId] = false
  }
}

async function stopProject(projectId: string) {
  actionLoading[projectId] = true
  const project = store.projects.find(p => p.id === projectId)
  const projectName = project?.name || projectId

  addLog('stopping', projectName, 'Stopping Docker containers...', 'info')

  try {
    // Use streaming version for real-time output
    const output = await api.stopProjectStreaming(projectId)
    addLog('completed', projectName, output || 'Project stopped successfully', 'success')
    await store.fetchProjects()
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e)
    addLog('error', projectName, `Failed to stop: ${errorMsg}`, 'error')
    console.error('Failed to stop project:', e)
  } finally {
    actionLoading[projectId] = false
  }
}

async function rebuildProject(projectId: string) {
  actionLoading[projectId] = true
  const project = store.projects.find(p => p.id === projectId)
  const projectName = project?.name || projectId

  addLog('rebuilding', projectName, 'Rebuilding Docker containers (this may take a while)...', 'warning')

  try {
    // Use streaming version for real-time output
    const output = await api.rebuildProjectStreaming(projectId)
    addLog('completed', projectName, output || 'Project rebuilt successfully', 'success')
    await store.fetchProjects()
    await store.refreshProjectStatus(projectId)
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e)
    addLog('error', projectName, `Failed to rebuild: ${errorMsg}`, 'error')
    console.error('Failed to rebuild project:', e)
  } finally {
    actionLoading[projectId] = false
  }
}

function confirmDelete(project: Project) {
  projectToDelete.value = project
}

async function deleteProject() {
  if (!projectToDelete.value) return
  try {
    await store.deleteProject(projectToDelete.value.id, true)
  } catch (e) {
    console.error('Failed to delete project:', e)
  } finally {
    projectToDelete.value = null
  }
}

async function openInEditor(projectId: string) {
  try {
    const editor = settingsStore.settings.preferred_editor || 'code'
    await api.openProjectInEditor(projectId, editor)
  } catch (e) {
    console.error('Failed to open project in editor:', e)
  }
}

function openEnvModal(project: Project) {
  envProject.value = project
}

function openCloneModal(project: Project) {
  cloneProject.value = project
}

function onProjectCreated() {
  showCreateModal.value = false
  store.fetchProjects()
}

function onProjectCloned(_project: Project) {
  cloneProject.value = null
  store.fetchProjects()
}

function onProjectImported(_project: Project) {
  showImportModal.value = false
  store.fetchProjects()
}

async function createFromCustomTemplate() {
  if (!customProjectName.value.trim() || !pendingCustomTemplate.value) return

  customTemplateCreating.value = true
  addLog('creating', customProjectName.value, 'Creating project from custom template...', 'info')

  try {
    await api.createProjectFromCustomTemplate(customProjectName.value.trim(), pendingCustomTemplate.value)
    addLog('completed', customProjectName.value, 'Project created successfully', 'success')
    showCustomTemplateModal.value = false
    pendingCustomTemplate.value = null
    customProjectName.value = ''
    store.fetchProjects()
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e)
    addLog('error', customProjectName.value, `Failed to create: ${errorMsg}`, 'error')
    console.error('Failed to create project from custom template:', e)
  } finally {
    customTemplateCreating.value = false
  }
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  })
}

function getEditorName(editor: string): string {
  const editors: Record<string, string> = {
    'code': 'VS Code',
    'cursor': 'Cursor',
    'phpstorm': 'PHPStorm',
    'sublime': 'Sublime',
    'atom': 'Atom'
  }
  return editors[editor] || 'Editor'
}

function getStatusBadgeClass(status: string): string {
  switch (status) {
    case 'running': return 'badge-running'
    case 'stopped': return 'badge-stopped'
    case 'building':
    case 'starting':
    case 'stopping': return 'badge-building'
    case 'error': return 'badge-error'
    default: return 'badge-stopped'
  }
}

function getStatusDotClass(status: string): string {
  switch (status) {
    case 'running': return 'bg-emerald-400'
    case 'stopped': return 'bg-red-400'
    case 'building':
    case 'starting':
    case 'stopping': return 'bg-amber-400 animate-pulse'
    case 'error': return 'bg-red-400'
    default: return 'bg-dark-400'
  }
}
</script>
