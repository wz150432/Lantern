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
