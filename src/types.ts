export interface BookRecord {
  id: number
  title: string
  author: string | null
  format: string
  filePath: string
  storageMode: 'linked' | 'copied'
  coverPath: string | null
  addedAt: number
  lastOpenedAt: number
  totalChapters: number
  currentChapter: number
  progress: number
}
export interface ChapterInfo { index: number; title: string; offset: number; length: number }
export interface Bookmark { id: number; bookId: number; chapterIndex: number; position: number; note: string | null; createdAt: number }
export interface SearchHit { chapterIndex: number; offsetInChapter: number; length: number; snippet: string }
export interface AppSettings {
  theme: string
  fontFamily: string
  fontSize: number
  lineHeight: number
  paragraphSpacing: number
  firstLineIndent: boolean
  charSpacing: number
  compressBlankLines: boolean
  wordWrap: boolean
  innerPadding: number
  scrollSpeed: number
  pageMode: 'page' | 'scroll'
  autoPageIntervalMs: number
  clickMode: 'thirds' | 'left-right'
  pageDouble: boolean
  windowTopmost: boolean
  windowOpacity: number
  immersiveMode: boolean
}
