import { defineStore } from 'pinia'
import { ref } from 'vue'
import { api } from '@/lib/api'
import type { Settings } from '@/lib/types'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<Settings>({
    projects_path: '',
    auto_start_projects: false,
    preferred_editor: 'code',
    default_php_version: '8.4',
    default_node_version: '18',
    theme: 'dark'
  })
  
  const loaded = ref(false)

  async function loadSettings() {
    try {
      const loadedSettings = await api.getSettings()
      settings.value = loadedSettings
      loaded.value = true
    } catch (e) {
      console.error('Failed to load settings:', e)
    }
  }

  async function saveSettings() {
    try {
      await api.saveSettings(settings.value)
    } catch (e) {
      console.error('Failed to save settings:', e)
    }
  }

  async function updateSetting<K extends keyof Settings>(key: K, value: Settings[K]) {
    settings.value[key] = value
    await saveSettings()
  }

  return {
    settings,
    loaded,
    loadSettings,
    saveSettings,
    updateSetting
  }
})
