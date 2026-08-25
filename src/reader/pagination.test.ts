import { describe, it, expect } from 'vitest'
import { pageCount, pageIndexFromScroll, scrollLeftFromPage, columnAdvance } from './pagination'

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


describe('columnAdvance', () => {
  const vp = 1000
  const pad = 48
  const gap = 48

  it('single page advances one full content width', () => {
    expect(columnAdvance(vp, pad, gap, false)).toBe(1000 - 96 + 48)
  })

  it('double page advances one column (half viewport), not a whole viewport', () => {
    const adv = columnAdvance(vp, pad, gap, true)
    const full = vp - 96
    expect(adv).toBeLessThan(full)
    expect(adv).toBeCloseTo((full - gap) / 2 + gap)
    expect(adv).toBeLessThan(full * 0.8)
  })

  it('double page yields no fewer pages than single for the same content width', () => {
    const scrollW = 6000
    const pagesSingle = Math.max(1, Math.ceil(scrollW / columnAdvance(vp, pad, gap, false)))
    const pagesDouble = Math.max(1, Math.ceil(scrollW / columnAdvance(vp, pad, gap, true)))
    expect(pagesDouble).toBeGreaterThanOrEqual(pagesSingle)
  })
})
