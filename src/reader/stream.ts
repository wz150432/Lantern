export interface StreamPos {
  chapterIndex: number
  frac: number
}

/**
 * 将滚动位置映射为（章节索引, 章内进度）。
 * texts 为已加载的连续章节文本（含当前章），startIndex 为 texts[0] 的章节索引。
 */
export function streamCharPosition(opts: {
  scrollTop: number
  maxScroll: number
  texts: string[]
  startIndex: number
  totalChapters: number
}): StreamPos {
  const { scrollTop, maxScroll, texts, startIndex, totalChapters } = opts
  if (totalChapters <= 0) return { chapterIndex: 0, frac: 0 }
  const totalLen = texts.reduce((a, t) => a + t.length, 0) || 1
  const frac = maxScroll > 0 ? Math.min(1, Math.max(0, scrollTop / maxScroll)) : 1
  const charPos = frac * totalLen
  let acc = 0
  let idx = 0
  for (let i = 0; i < texts.length; i++) {
    if (charPos <= acc + texts[i].length) { idx = i; break }
    acc += texts[i].length
    idx = i + 1
  }
  const within = Math.min(1, Math.max(0, (charPos - acc) / Math.max(1, texts[idx]?.length ?? 1)))
  return { chapterIndex: Math.min(startIndex + idx, totalChapters - 1), frac: within }
}
