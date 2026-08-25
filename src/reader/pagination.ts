export function pageCount(containerWidth: number, pageWidth: number): number {
  if (containerWidth <= 0 || pageWidth <= 0) return 1
  return Math.max(1, Math.ceil(containerWidth / pageWidth))
}
export function pageIndexFromScroll(scrollLeft: number, pageWidth: number): number {
  return Math.max(0, Math.round(scrollLeft / Math.max(1, pageWidth)))
}
export function scrollLeftFromPage(page: number, pageWidth: number): number {
  return Math.max(0, page) * pageWidth
}

/**
 * 计算翻页模式下每次前进的滚动距离（含列间距）。
 * double=true 时每列宽为内容宽的一半，前进一列 = 只更新半屏内容（左右双页像翻书）。
 */
export function columnAdvance(viewportWidth: number, padding: number, gap: number, double: boolean): number {
  const totalWidth = Math.max(1, viewportWidth - padding * 2)
  const colW = double ? Math.max(1, (totalWidth - gap) / 2) : Math.max(1, totalWidth)
  return colW + gap
}
