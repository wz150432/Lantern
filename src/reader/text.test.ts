import { describe, it, expect } from 'vitest'
import { toParagraphs } from './text'

describe('toParagraphs', () => {
  it('splits paragraphs on blank lines', () => {
    const ps = toParagraphs('第一段。\n\n第二段。', { compressBlankLines: true })
    expect(ps.length).toBe(2)
    expect(ps[0].text).toBe('第一段。')
  })
  it('marks headings', () => {
    const ps = toParagraphs('第一章 风起\n内容。', { compressBlankLines: true })
    expect(ps[0].isHeading).toBe(true)
    expect(ps[1].isHeading).toBe(false)
  })
  it('compresses multiple blank lines', () => {
    const ps = toParagraphs('甲。\n\n\n\n乙。', { compressBlankLines: true })
    expect(ps.length).toBe(2)
  })
  it('keeps blank lines when not compressing', () => {
    const ps = toParagraphs('甲。\n\n乙。', { compressBlankLines: false })
    expect(ps.length).toBeGreaterThanOrEqual(2)
  })
})
