<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useSettingsStore } from '../stores/settings'
import { THEME_PRESETS } from '../theme/theme'
import { zh } from '../i18n/zh'
import * as ipc from '../ipc'
import { ask } from '@tauri-apps/plugin-dialog'
import type { AppSettings } from '../types'
import { HOTKEY_ACTIONS, buildBindings, comboToText } from '../hotkeys/actions'
import type { HotkeyAction } from '../hotkeys/actions'

const router = useRouter()
const settings = useSettingsStore()

const previewIndentStyle = computed(() => ({
  textIndent: settings.settings?.firstLineIndent ? '2em' : '0',
}))

const previewParagraphs = computed(() => {
  if (!settings.settings?.compressBlankLines) return [zh.settings.previewChapter, '', zh.settings.previewBody]
  return [zh.settings.previewChapter, zh.settings.previewBody]
})

function patch(p: Partial<AppSettings>) {
  void settings.update(p)
}

const recordingId = ref<HotkeyAction | null>(null)

function comboText(id: HotkeyAction): string {
  return comboToText(buildBindings(settings.settings)[id])
}

function startRecord(id: HotkeyAction) {
  recordingId.value = id
}

async function onRecordKey(e: KeyboardEvent) {
  if (!recordingId.value) return
  e.preventDefault()
  e.stopPropagation()
  if (e.key === 'Escape') { recordingId.value = null; return }
  if (['Control', 'Shift', 'Alt', 'Meta'].includes(e.key)) return
  const mods: string[] = []
  if (e.ctrlKey) mods.push('Ctrl')
  if (e.shiftKey) mods.push('Shift')
  if (e.altKey) mods.push('Alt')
  const keyMap: Record<string, string> = {
    ' ': 'Space', ArrowRight: 'ArrowRight', ArrowLeft: 'ArrowLeft',
    ArrowUp: 'ArrowUp', ArrowDown: 'ArrowDown',
  }
  const k = keyMap[e.key] ?? e.key
  const combo = [...mods, k].join('+')
  const hotkeys = { ...(settings.settings?.hotkeys ?? {}), [recordingId.value]: combo }
  await settings.update({ hotkeys })
  recordingId.value = null
}

async function onAutoHideChange() {
  await ipc.toggleAutoHide()
  await settings.load()
}

async function onOpacity(v: number) {
  patch({ windowOpacity: v })
  // 平台限制：macOS/Linux 整窗透明支持有限，此处仅尽力持久化设置，失败忽略（R6）
  try {
    await ipc.setOpacity(v)
  } catch { /* ignore */ }
}

async function restore() {
  const ok = await ask(zh.settings.resetConfirm, {
    title: zh.settings.restoreDefault,
    kind: 'warning',
    okLabel: zh.common.confirm,
    cancelLabel: zh.common.cancel,
  })
  if (ok) void settings.restoreDefault()
}

onMounted(() => {
  void settings.load()
  window.addEventListener('keydown', onRecordKey)
})

onUnmounted(() => {
  window.removeEventListener('keydown', onRecordKey)
})
</script>

<template>
  <div class="settings-page">
    <header class="settings-head">
      <button class="link" @click="router.push('/')">← {{ zh.reader.back }}</button>
      <h1>{{ zh.settings.title }}</h1>
    </header>

    <section v-if="settings.settings" class="group">
      <h2>{{ zh.settings.display }}</h2>
      <label>{{ zh.settings.theme }}
        <select :value="settings.settings.theme" @change="patch({ theme: ($event.target as HTMLSelectElement).value })">
          <option v-for="t in THEME_PRESETS" :key="t.id" :value="t.id">{{ t.name }}</option>
        </select>
      </label>
      <label>{{ zh.settings.fontFamily }}
        <select :value="settings.settings.fontFamily" @change="patch({ fontFamily: ($event.target as HTMLSelectElement).value })">
          <option value="system-ui, &quot;PingFang SC&quot;, &quot;Microsoft YaHei&quot;, sans-serif">{{ zh.settings.fontSystem }}</option>
          <option value="&quot;Source Han Serif SC&quot;, serif">{{ zh.settings.fontSerif }}</option>
          <option value="&quot;Noto Serif SC&quot;, serif">{{ zh.settings.fontNotoSerif }}</option>
        </select>
      </label>
      <label>{{ zh.settings.fontSize }} <input type="number" min="10" max="48" :value="settings.settings.fontSize" @change="patch({ fontSize: Number(($event.target as HTMLInputElement).value) })" /> px</label>
    </section>

    <section v-if="settings.settings" class="group">
      <h2>{{ zh.settings.typography }}</h2>
      <label>{{ zh.settings.lineHeight }} <input type="range" min="1" max="3" step="0.1" :value="settings.settings.lineHeight" @input="patch({ lineHeight: Number(($event.target as HTMLInputElement).value) })" /></label>
      <label>{{ zh.settings.paragraphSpacing }} <input type="range" min="0" max="40" step="1" :value="settings.settings.paragraphSpacing" @input="patch({ paragraphSpacing: Number(($event.target as HTMLInputElement).value) })" /></label>
      <label>{{ zh.settings.charSpacing }} <input type="range" min="0" max="5" step="0.1" :value="settings.settings.charSpacing" @input="patch({ charSpacing: Number(($event.target as HTMLInputElement).value) })" /></label>
      <label>{{ zh.settings.innerPadding }} <input type="range" min="0" max="200" step="4" :value="settings.settings.innerPadding" @input="patch({ innerPadding: Number(($event.target as HTMLInputElement).value) })" /></label>
      <label><input type="checkbox" :checked="settings.settings.firstLineIndent" @change="patch({ firstLineIndent: ($event.target as HTMLInputElement).checked })" /> {{ zh.settings.firstLineIndent }}</label>
      <label><input type="checkbox" :checked="settings.settings.compressBlankLines" @change="patch({ compressBlankLines: ($event.target as HTMLInputElement).checked })" /> {{ zh.settings.compressBlankLines }}</label>
      <label><input type="checkbox" :checked="settings.settings.wordWrap" @change="patch({ wordWrap: ($event.target as HTMLInputElement).checked })" /> {{ zh.settings.wordWrap }}</label>

      <div class="preview">
        <h3>{{ zh.settings.preview }}</h3>
        <div class="preview-body" :style="previewIndentStyle">
          <p v-for="(text, i) in previewParagraphs" :key="i">{{ text }}</p>
        </div>
      </div>
    </section>

    <section v-if="settings.settings" class="group">
      <h2>{{ zh.settings.reading }}</h2>
      <label>{{ zh.settings.pageMode }}
        <select :value="settings.settings.pageMode" @change="patch({ pageMode: ($event.target as HTMLSelectElement).value as 'page' | 'scroll' })">
          <option value="page">{{ zh.settings.pageModePage }}</option>
          <option value="scroll">{{ zh.settings.pageModeScroll }}</option>
        </select>
      </label>
      <label>{{ zh.settings.autoPageInterval }} <input type="number" min="500" step="500" :value="settings.settings.autoPageIntervalMs" @change="patch({ autoPageIntervalMs: Number(($event.target as HTMLInputElement).value) })" /></label>
      <label>{{ zh.settings.clickMode }}
        <select :value="settings.settings.clickMode" @change="patch({ clickMode: ($event.target as HTMLSelectElement).value as 'thirds' | 'left-right' })">
          <option value="thirds">{{ zh.settings.clickThirds }}</option>
          <option value="left-right">{{ zh.settings.clickLeftRight }}</option>
        </select>
      </label>
      <label><input type="checkbox" :checked="settings.settings.pageDouble" @change="patch({ pageDouble: ($event.target as HTMLInputElement).checked })" /> {{ zh.settings.pageDouble }}</label>
      <label><input type="checkbox" :checked="settings.settings.autoHideOnLeave" @change="onAutoHideChange" /> {{ zh.settings.autoHideOnLeave }}</label>
    </section>

    <section v-if="settings.settings" class="group">
      <h2>{{ zh.settings.hotkeys }}</h2>
      <p class="hint">{{ zh.settings.hotkeysHint }}</p>
      <div class="hk-row" v-for="a in HOTKEY_ACTIONS" :key="a">
        <span class="hk-name">{{ zh.hotkeyActions[a] }}</span>
        <button class="hk-btn" :class="{ recording: recordingId === a }" @click="startRecord(a)">
          {{ recordingId === a ? zh.settings.recording : comboText(a) }}
        </button>
      </div>
    </section>

    <section v-if="settings.settings" class="group">
      <h2>{{ zh.settings.window }}</h2>
      <label><input type="checkbox" :checked="settings.settings.windowTopmost" @change="patch({ windowTopmost: ($event.target as HTMLInputElement).checked })" /> {{ zh.settings.windowTopmost }}</label>
      <label>{{ zh.settings.windowOpacity }} <input type="range" min="0.1" max="1" step="0.05" :value="settings.settings.windowOpacity" @input="onOpacity(Number(($event.target as HTMLInputElement).value))" /></label>
      <label><input type="checkbox" :checked="settings.settings.immersiveMode" @change="patch({ immersiveMode: ($event.target as HTMLInputElement).checked })" /> {{ zh.settings.immersiveMode }}</label>
    </section>

    <section class="group">
      <h2>{{ zh.settings.advanced }}</h2>
      <button @click="restore">{{ zh.settings.restoreDefault }}</button>
      <p class="about">{{ zh.settings.about }}：{{ zh.appName }} {{ zh.settings.version }}</p>
    </section>
  </div>
</template>

<style scoped>
.settings-page { height: 100%; overflow-y: auto; padding: 24px 40px; background: var(--bg); }
.settings-head { display: flex; align-items: center; gap: 16px; margin-bottom: 20px; }
.group { background: var(--panel); border: 1px solid var(--border); border-radius: 10px; padding: 16px 20px; margin-bottom: 16px; }
.group h2 { font-size: 15px; margin-bottom: 12px; color: var(--text-dim); }
.group label { display: flex; align-items: center; gap: 10px; margin-bottom: 10px; font-size: 14px; }
.group input[type="number"], .group select { padding: 4px 8px; border: 1px solid var(--border); border-radius: 6px; }
.preview { margin-top: 14px; padding: 14px; background: var(--reader-bg); border-radius: 8px; }
.preview-body { font-family: var(--reader-font-family); font-size: var(--reader-font-size); line-height: var(--reader-line-height); letter-spacing: var(--reader-char-spacing); }
.preview-body p { margin: 0 0 var(--reader-para-spacing) 0; }
.about { margin-top: 12px; font-size: 13px; color: var(--text-dim); }
.hint { font-size: 12px; color: var(--text-dim); margin-bottom: 10px; }
.hk-row { display: flex; align-items: center; justify-content: space-between; margin-bottom: 8px; }
.hk-name { font-size: 14px; }
.hk-btn { min-width: 120px; padding: 4px 10px; border: 1px solid var(--border); border-radius: 6px; background: var(--panel); color: var(--text); cursor: pointer; font-size: 13px; }
.hk-btn.recording { border-color: var(--accent); color: var(--accent); }
</style>
