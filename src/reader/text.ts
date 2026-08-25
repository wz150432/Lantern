export interface Paragraph { text: string; isHeading: boolean }

const HEADING_RE = /^\s*(第[0-9零一二三四五六七八九十百千万两]+[章卷节回部集]|序章|序言|前言|楔子|番外|后记|尾声|正文)([\s:：、.．·\-—–].*)?\s*$/

export function toParagraphs(raw: string, opts: { compressBlankLines: boolean }): Paragraph[] {
  const lines = raw.split(/\r?\n/)
  const out: Paragraph[] = []
  let buf = ''
  const flush = () => {
    if (buf.trim()) out.push({ text: buf.trim(), isHeading: HEADING_RE.test(buf.trim()) })
    buf = ''
  }
  for (const line of lines) {
    const t = line.trim()
    if (!t) {
      // 空行分段：压缩模式下空行仅作分隔（连续空行自然合并为一个分隔）；
      // 非压缩模式下保留空行（输出空段落占位，保留版式空隙）
      flush()
      if (!opts.compressBlankLines) out.push({ text: '', isHeading: false })
      continue
    }
    // 标题行强制分段（标题独占一段）；行首全角缩进行亦分段
    if (buf && (HEADING_RE.test(t) || t.startsWith('　'))) flush()
    if (HEADING_RE.test(t)) {
      buf = t
      flush()
      continue
    }
    buf = buf ? `${buf}\n${t}` : t
  }
  flush()
  return out
}
