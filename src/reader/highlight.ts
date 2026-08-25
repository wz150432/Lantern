export interface Range { start: number; end: number }
export function highlightRanges(text: string, hits: { offsetInChapter: number; length: number }[]): Range[] {
  return hits
    .filter((h) => h.offsetInChapter < text.length)
    .map((h) => ({ start: h.offsetInChapter, end: Math.min(text.length, h.offsetInChapter + h.length) }))
    .sort((a, b) => a.start - b.start)
}
