import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { defineComponent } from 'vue'
import { useHotkeys } from './useHotkeys'
import { DEFAULT_HOTKEYS } from '../hotkeys/actions'

let handler: () => void = () => {}

const Comp = defineComponent({
  setup() {
    useHotkeys(() => DEFAULT_HOTKEYS, { addBookmark: () => handler() })
    return () => null
  },
})

describe('useHotkeys integration (window keydown)', () => {
  it('fires the handler when the recorded combo is pressed', () => {
    const cb = vi.fn()
    handler = cb
    const w = mount(Comp)
    window.dispatchEvent(new KeyboardEvent('keydown', { key: 'm', ctrlKey: true, bubbles: true }))
    expect(cb).toHaveBeenCalledTimes(1)
    w.unmount()
  })

  it('does not fire for a different combo', () => {
    const cb = vi.fn()
    handler = cb
    const w = mount(Comp)
    window.dispatchEvent(new KeyboardEvent('keydown', { key: 'x', ctrlKey: true, bubbles: true }))
    expect(cb).not.toHaveBeenCalled()
    w.unmount()
  })

  it('does not fire when a required modifier is missing', () => {
    const cb = vi.fn()
    handler = cb
    const w = mount(Comp)
    window.dispatchEvent(new KeyboardEvent('keydown', { key: 'm', bubbles: true }))
    expect(cb).not.toHaveBeenCalled()
    w.unmount()
  })
})
