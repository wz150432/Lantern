import { describe, it, expect, vi } from 'vitest'
import { THEME_PRESETS, applyTheme } from './theme'
import type { AppSettings } from '../types'

const settings: AppSettings = {
  theme: 'minimal', fontFamily: 'sans-serif', fontSize: 18, lineHeight: 1.8,
  paragraphSpacing: 8, firstLineIndent: true, charSpacing: 0.5, compressBlankLines: true,
  wordWrap: true, innerPadding: 48, scrollSpeed: 1, pageMode: 'page',
  autoPageIntervalMs: 3000, clickMode: 'thirds', pageDouble: false, windowTopmost: false,
  autoHideOnLeave: false, hotkeys: {},
  windowOpacity: 1, immersiveMode: false,
}

describe('theme', () => {
  it('minimal preset exists by default', () => {
    expect(THEME_PRESETS[0].id).toBe('minimal')
  })
  it('applies css vars', () => {
    const el = document.createElement('div')
    const spy = vi.spyOn(document.documentElement.style, 'setProperty')
    applyTheme(settings)
    expect(spy).toHaveBeenCalledWith('--reader-font-size', '18px')
    spy.mockRestore()
    void el
  })
})
