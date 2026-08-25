import type { AppSettings } from '../types'

export interface ThemePreset {
  id: string
  name: string
  vars: Record<string, string>
}

export const THEME_PRESETS: ThemePreset[] = [
  {
    id: 'minimal',
    name: '极简沉浸',
    vars: {
      '--bg': '#faf9f7', '--text': '#2b2b2b', '--text-dim': '#8a8a8a',
      '--panel': '#ffffff', '--accent': '#4a6fa5', '--border': '#e6e4e0',
      '--reader-bg': '#f5f4f1', '--reader-text': '#333333',
    },
  },
  {
    id: 'paper',
    name: '书卷质感',
    vars: {
      '--bg': '#f4ecd8', '--text': '#3a3128', '--text-dim': '#8d7f6b',
      '--panel': '#faf3e3', '--accent': '#8c6a3f', '--border': '#e2d5bc',
      '--reader-bg': '#f6eeda', '--reader-text': '#3a3128',
    },
  },
]

export function applyTheme(settings: AppSettings) {
  const preset = THEME_PRESETS.find((p) => p.id === settings.theme) ?? THEME_PRESETS[0]
  const root = document.documentElement
  for (const [k, v] of Object.entries(preset.vars)) root.style.setProperty(k, v)
  root.style.setProperty('--reader-font-size', `${settings.fontSize}px`)
  root.style.setProperty('--reader-line-height', String(settings.lineHeight))
  root.style.setProperty('--reader-para-spacing', `${settings.paragraphSpacing}px`)
  root.style.setProperty('--reader-char-spacing', `${settings.charSpacing}px`)
  root.style.setProperty('--reader-padding', `${settings.innerPadding}px`)
  root.style.setProperty('--reader-font-family', settings.fontFamily)
  root.style.setProperty('--reader-word-wrap', settings.wordWrap ? 'break-word' : 'normal')
  root.style.opacity = String(settings.windowOpacity)
}
