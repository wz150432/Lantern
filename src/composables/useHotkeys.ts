import { onMounted, onUnmounted } from 'vue'
import type { HotkeyAction } from '../hotkeys/actions'
import { comboMatches } from '../hotkeys/actions'

export type HotkeyHandlers = Partial<Record<HotkeyAction, () => void>>

/**
 * 数据驱动的快捷键：bindings 通过 getter 读取（跟随设置实时更新）。
 * 按键时遍历 handlers 中提供的动作，与当前绑定组合匹配则触发。
 */
export function useHotkeys(
  getBindings: () => Record<HotkeyAction, string>,
  handlers: HotkeyHandlers,
) {
  function onKey(e: KeyboardEvent) {
    const bindings = getBindings()
    for (const action of Object.keys(handlers) as HotkeyAction[]) {
      const combo = bindings[action]
      if (combo && comboMatches(combo, e)) {
        handlers[action]?.()
        e.preventDefault()
        return
      }
    }
  }
  onMounted(() => window.addEventListener('keydown', onKey))
  onUnmounted(() => window.removeEventListener('keydown', onKey))
}
