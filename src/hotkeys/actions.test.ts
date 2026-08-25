import { describe, it, expect } from 'vitest'
import { buildBindings, comboMatches, comboToText } from './actions'

function keyEvent(key: string, ctrl = false, shift = false, alt = false): KeyboardEvent {
  return { key, ctrlKey: ctrl, shiftKey: shift, altKey: alt } as KeyboardEvent
}

describe('buildBindings', () => {
  it('merges custom hotkeys over defaults', () => {
    const b = buildBindings({ hotkeys: { nextPage: 'Ctrl+P' } } as any)
    expect(b.nextPage).toBe('Ctrl+P')
    expect(b.prevPage).toBe('ArrowLeft')
  })
})

describe('comboMatches', () => {
  it('matches simple key', () => {
    expect(comboMatches('ArrowRight', keyEvent('ArrowRight'))).toBe(true)
    expect(comboMatches('ArrowRight', keyEvent('ArrowLeft'))).toBe(false)
  })
  it('matches modifiers exactly', () => {
    expect(comboMatches('Ctrl+F', keyEvent('f', true))).toBe(true)
    expect(comboMatches('Ctrl+F', keyEvent('f'))).toBe(false)
    expect(comboMatches('Ctrl+F', keyEvent('f', true, true))).toBe(false)
  })
  it('matches Alt+H and space', () => {
    expect(comboMatches('Alt+H', keyEvent('h', false, false, true))).toBe(true)
    expect(comboMatches('Space', keyEvent(' '))).toBe(true)
  })
  it('does not fire when other modifiers held', () => {
    expect(comboMatches('ArrowRight', keyEvent('ArrowRight', false, false, true))).toBe(false)
  })
})

describe('comboToText', () => {
  it('pretty prints arrows and space', () => {
    expect(comboToText('Ctrl+ArrowRight')).toContain('→')
    expect(comboToText('Space')).toContain('空格')
  })
})
