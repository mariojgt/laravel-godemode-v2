import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { api } from '@/lib/api'
import type { Project, ServiceStatus } from '@/lib/types'

export const useProjectsStore = defineStore('projects', () => {
  const projects = ref<Project[]>([])
  const selectedProjectId = ref<string | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const servicesStatus = ref<Record<string, ServiceStatus[]>>({})

  const selectedProject = computed(() => {
    if (!selectedProjectId.value) return null
    return projects.value.find(p => p.id === selectedProjectId.value) || null
  })

  const runningProjects = computed(() =>
    projects.value.filter(p => p.status === 'running')
  )

  const stoppedProjects = computed(() =>
    projects.value.filter(p => p.status === 'stopped')
  )

  async function fetchProjects() {
    loading.value = true
    error.value = null
    try {
      projects.value = await api.getProjects()
      // Update status for each project
      for (const project of projects.value) {
        try {
          const status = await api.getProjectStatus(project.id)
          servicesStatus.value[project.id] = status
          // Update project status based on containers
          if (status.length > 0 && status.some(s => s.status === 'running')) {
            project.status = 'running'
          } else {
            project.status = 'stopped'
          }
        } catch {
          // Project containers not running
          project.status = 'stopped'
        }
      }
    } catch (e) {
      error.value = e as string
      console.error('Failed to fetch projects:', e)
    } finally {
      loading.value = false
    }
  }

  async function createProject(name: string, template: string, config: any) {
    loading.value = true
    error.value = null
    try {
      const project = await api.createProject(name, template, config)
      projects.value.push(project)
      return project
    } catch (e) {
      error.value = e as string
      throw e
    } finally {
      loading.value = false
    }
  }

  async function deleteProject(projectId: string, deleteFiles: boolean = true) {
    loading.value = true
    error.value = null
    try {
      await api.deleteProject(projectId, deleteFiles)
      projects.value = projects.value.filter(p => p.id !== projectId)
      if (selectedProjectId.value === projectId) {
        selectedProjectId.value = null
      }
    } catch (e) {
      error.value = e as string
      throw e
    } finally {
      loading.value = false
    }
  }

  async function startProject(projectId: string) {
    const project = projects.value.find(p => p.id === projectId)
    if (project) {
      project.status = 'starting'
    }
    try {
      await api.startProject(projectId)
      if (project) {
        project.status = 'running'
      }
      await refreshProjectStatus(projectId)
    } catch (e) {
      if (project) {
        project.status = 'error'
      }
      throw e
    }
  }

  async function stopProject(projectId: string) {
    const project = projects.value.find(p => p.id === projectId)
    if (project) {
      project.status = 'stopping'
    }
    try {
      await api.stopProject(projectId)
      if (project) {
        project.status = 'stopped'
      }
      servicesStatus.value[projectId] = []
    } catch (e) {
      if (project) {
        project.status = 'error'
      }
      throw e
    }
  }

  async function restartProject(projectId: string) {
    await api.restartProject(projectId)
    await refreshProjectStatus(projectId)
  }

  async function rebuildProject(projectId: string) {
    const project = projects.value.find(p => p.id === projectId)
    if (project) {
      project.status = 'building'
    }
    try {
      await api.rebuildProject(projectId)
      if (project) {
        project.status = 'running'
      }
      await refreshProjectStatus(projectId)
    } catch (e) {
      if (project) {
        project.status = 'error'
      }
      throw e
    }
  }

  async function refreshProjectStatus(projectId: string) {
    try {
      const status = await api.getProjectStatus(projectId)
      servicesStatus.value[projectId] = status
    } catch {
      servicesStatus.value[projectId] = []
    }
  }

  function selectProject(projectId: string | null) {
    selectedProjectId.value = projectId
  }

  return {
    projects,
    selectedProjectId,
    selectedProject,
    loading,
    error,
    servicesStatus,
    runningProjects,
    stoppedProjects,
    fetchProjects,
    createProject,
    deleteProject,
    startProject,
    stopProject,
    restartProject,
    rebuildProject,
    refreshProjectStatus,
    selectProject
  }
})
