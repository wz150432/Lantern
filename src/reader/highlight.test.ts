import { describe, it, expect } from 'vitest'
import { highlightRanges } from './highlight'

describe('highlightRanges', () => {
  it('maps hits to ranges sorted', () => {
    const ranges = highlightRanges('甲乙丙丁', [{ offsetInChapter: 2, length: 2 }, { offsetInChapter: 0, length: 1 }])
    expect(ranges).toEqual([{ start: 0, end: 1 }, { start: 2, end: 4 }])
  })
  it('clamps out-of-range hits', () => {
    const ranges = highlightRanges('短', [{ offsetInChapter: 99, length: 5 }])
    expect(ranges).toEqual([])
  })
})
