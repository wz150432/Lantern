import { describe, it, expect } from 'vitest'
import { streamCharPosition } from './stream'

const texts = ['第一章内容'.repeat(10), '第二章内容'.repeat(10), '第三章内容'.repeat(10)]

describe('streamCharPosition', () => {
  it('maps top to first chapter start', () => {
    const p = streamCharPosition({ scrollTop: 0, maxScroll: 1000, texts, startIndex: 0, totalChapters: 3 })
    expect(p.chapterIndex).toBe(0)
    expect(p.frac).toBe(0)
  })

  it('maps bottom to last chapter end', () => {
    const p = streamCharPosition({ scrollTop: 1000, maxScroll: 1000, texts, startIndex: 0, totalChapters: 3 })
    expect(p.chapterIndex).toBe(2)
    expect(p.frac).toBeCloseTo(1)
  })

  it('maps middle into the middle chapter', () => {
    const p = streamCharPosition({ scrollTop: 500, maxScroll: 1000, texts, startIndex: 0, totalChapters: 3 })
    expect(p.chapterIndex).toBe(1)
    expect(p.frac).toBeGreaterThan(0)
    expect(p.frac).toBeLessThan(1)
  })

  it('respects startIndex and totalChapters clamp', () => {
    const p = streamCharPosition({ scrollTop: 1000, maxScroll: 1000, texts, startIndex: 5, totalChapters: 6 })
    expect(p.chapterIndex).toBe(5)
  })

  it('handles empty texts', () => {
    const p = streamCharPosition({ scrollTop: 0, maxScroll: 0, texts: [''], startIndex: 0, totalChapters: 1 })
    expect(p.chapterIndex).toBe(0)
  })
})
