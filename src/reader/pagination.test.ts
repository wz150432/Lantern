import { describe, it, expect } from 'vitest'
import { pageCount, pageIndexFromScroll, scrollLeftFromPage } from './pagination'

describe('pagination math', () => {
  it('computes page count', () => {
    expect(pageCount(300, 100)).toBe(3)
    expect(pageCount(250, 100)).toBe(3)
    expect(pageCount(100, 100)).toBe(1)
  })
  it('maps scroll to page', () => {
    expect(pageIndexFromScroll(350, 100)).toBe(4)
    expect(pageIndexFromScroll(0, 100)).toBe(0)
  })
  it('maps page to scroll', () => {
    expect(scrollLeftFromPage(2, 100)).toBe(200)
  })
})
