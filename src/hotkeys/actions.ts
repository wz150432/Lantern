import type { AppSettings } from '../types'

export const HOTKEY_ACTIONS = [
  'nextPage', 'prevPage', 'nextChapter', 'prevChapter',
  'scrollUp', 'scrollDown', 'toggleSearch', 'jumpPercent',
  'addBookmark', 'openFile', 'toggleTopmost', 'zoomIn',
  'zoomOut', 'toggleAutoPage', 'toggleFullscreen', 'toggleImmersive',
  'toggleWindowVisible',
] as const

export type HotkeyAction = (typeof HOTKEY_ACTIONS)[number]

export const DEFAULT_HOTKEYS: Record<HotkeyAction, string> = {
  nextPage: 'ArrowRight',
  prevPage: 'ArrowLeft',
  nextChapter: 'Ctrl+ArrowRight',
  prevChapter: 'Ctrl+ArrowLeft',
  scrollUp: 'ArrowUp',
  scrollDown: 'ArrowDown',
  toggleSearch: 'Ctrl+F',
  jumpPercent: 'Ctrl+G',
  addBookmark: 'Ctrl+M',
  openFile: 'Ctrl+O',
  toggleTopmost: 'Ctrl+T',
  zoomIn: 'Ctrl+=',
  zoomOut: 'Ctrl+-',
  toggleAutoPage: 'Space',
  toggleFullscreen: 'F11',
  toggleImmersive: 'F12',
  toggleWindowVisible: 'Alt+H',
}

export function buildBindings(settings: AppSettings | null): Record<HotkeyAction, string> {
  const merged = { ...DEFAULT_HOTKEYS }
  if (settings?.hotkeys) {
    for (const [k, v] of Object.entries(settings.hotkeys)) {
      if (k in merged && v) merged[k as HotkeyAction] = v
    }
  }
  return merged
}

const KEY_ALIAS: Record<string, string> = { Space: ' ', Plus: '+', Minus: '-' }

export function comboMatches(combo: string, e: KeyboardEvent): boolean {
  const parts = combo.split('+')
  const key = KEY_ALIAS[parts[parts.length - 1]] ?? parts[parts.length - 1]
  const wantCtrl = parts.includes('Ctrl')
  const wantShift = parts.includes('Shift')
  const wantAlt = parts.includes('Alt')
  if (e.ctrlKey !== wantCtrl) return false
  if (e.shiftKey !== wantShift) return false
  if (e.altKey !== wantAlt) return false
  return e.key.toLowerCase() === key.toLowerCase()
}

export function comboToText(combo: string): string {
  return combo
    .replace('ArrowRight', '→')
    .replace('ArrowLeft', '←')
    .replace('ArrowUp', '↑')
    .replace('ArrowDown', '↓')
    .replace('Space', '空格')
}
