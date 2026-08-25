import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as ipc from '../ipc'
import { applyTheme } from '../theme/theme'
import type { AppSettings } from '../types'

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings | null>(null)
  async function load() {
    settings.value = await ipc.getSettings()
    applyTheme(settings.value)
  }
  async function update(patch: Partial<AppSettings>) {
    if (!settings.value) return
    settings.value = { ...settings.value, ...patch }
    await ipc.saveSettings(settings.value)
    applyTheme(settings.value)
  }
  async function restoreDefault() {
    settings.value = { ...settings.value!, theme: 'minimal' }
    // 简化：还原主题相关；完整还原由后端支持后实现
    await ipc.saveSettings(settings.value)
    applyTheme(settings.value)
  }
  return { settings, load, update, restoreDefault }
})
