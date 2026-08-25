import { onMounted, onUnmounted } from 'vue'

export interface HotkeyHandlers {
  nextPage?: () => void
  prevPage?: () => void
  nextChapter?: () => void
  prevChapter?: () => void
  toggleFullscreen?: () => void
  toggleImmersive?: () => void
  toggleAutoPage?: () => void
  toggleSearch?: () => void
  jumpPercent?: () => void
  addBookmark?: () => void
  openFile?: () => void
  zoomIn?: () => void
  zoomOut?: () => void
  toggleTopmost?: () => void
}

export function useHotkeys(handlers: HotkeyHandlers) {
  function onKey(e: KeyboardEvent) {
    const k = e.key
    if (e.ctrlKey && !e.shiftKey && !e.altKey) {
      if (k === 'ArrowRight') { handlers.nextChapter?.(); e.preventDefault(); return }
      if (k === 'ArrowLeft') { handlers.prevChapter?.(); e.preventDefault(); return }
      if (k === 'f') { handlers.toggleSearch?.(); e.preventDefault(); return }
      if (k === 'g') { handlers.jumpPercent?.(); e.preventDefault(); return }
      if (k === 'm') { handlers.addBookmark?.(); e.preventDefault(); return }
      if (k === 'o') { handlers.openFile?.(); e.preventDefault(); return }
      if (k === 't') { handlers.toggleTopmost?.(); e.preventDefault(); return }
      if (k === '=' || k === '+') { handlers.zoomIn?.(); e.preventDefault(); return }
      if (k === '-') { handlers.zoomOut?.(); e.preventDefault(); return }
      return
    }
    if (!e.ctrlKey && !e.shiftKey && !e.altKey) {
      if (k === 'ArrowRight') { handlers.nextPage?.(); e.preventDefault(); return }
      if (k === 'ArrowLeft') { handlers.prevPage?.(); e.preventDefault(); return }
      if (k === ' ') { handlers.toggleAutoPage?.(); e.preventDefault(); return }
      if (k === 'F11') { handlers.toggleFullscreen?.(); e.preventDefault(); return }
      if (k === 'F12') { handlers.toggleImmersive?.(); e.preventDefault(); return }
    }
  }
  onMounted(() => window.addEventListener('keydown', onKey))
  onUnmounted(() => window.removeEventListener('keydown', onKey))
}
