import { invoke } from '@tauri-apps/api/core'
import type { AppSettings, Bookmark, BookRecord, ChapterInfo, SearchHit } from './types'

export function invokeBook<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  return invoke<T>(cmd, args)
}

export function listLibrary(): Promise<BookRecord[]> { return invokeBook<BookRecord[]>('list_library') }
export function openBook(path: string): Promise<BookRecord> { return invokeBook<BookRecord>('open_book', { path }) }
export function importBook(path: string, mode: 'linked' | 'copied'): Promise<BookRecord> { return invokeBook<BookRecord>('import_book', { path, mode }) }
export function deleteBook(id: number, deleteCopy: boolean): Promise<void> { return invokeBook<void>('delete_book', { id, deleteCopy }) }
export function saveProgress(id: number, chapter: number, progress: number): Promise<void> { return invokeBook<void>('save_progress', { id, chapter, progress }) }
export function getBookmarks(bookId: number): Promise<Bookmark[]> { return invokeBook<Bookmark[]>('get_bookmarks', { bookId }) }
export function addBookmark(bookId: number, chapter: number, position: number, note?: string | null): Promise<number> { return invokeBook<number>('add_bookmark', { bookId, chapter, position, note: note ?? null }) }
export function deleteBookmark(id: number): Promise<void> { return invokeBook<void>('delete_bookmark', { id }) }
export function getChapters(bookId: number, path: string): Promise<ChapterInfo[]> { return invokeBook<ChapterInfo[]>('get_chapters', { bookId, path }) }
export function getChapterText(bookId: number, index: number): Promise<string> { return invokeBook<string>('get_chapter_text', { bookId, index }) }
export function searchText(bookId: number, query: string, limit = 500): Promise<SearchHit[]> { return invokeBook<SearchHit[]>('search_text', { bookId, query, limit }) }
export function getSettings(): Promise<AppSettings> { return invokeBook<AppSettings>('get_settings') }
export function saveSettings(settings: AppSettings): Promise<void> { return invokeBook<void>('save_settings', { settings }) }
export function setFullscreen(full: boolean): Promise<void> { return invokeBook<void>('set_fullscreen', { full }) }
export function setTopmost(top: boolean): Promise<void> { return invokeBook<void>('set_topmost', { top }) }
export function setOpacity(opacity: number): Promise<void> { return invokeBook<void>('set_opacity', { opacity }) }
export function setDecorations(decorated: boolean): Promise<void> { return invokeBook<void>('set_decorations', { decorated }) }
export function toggleWindowVisible(): Promise<void> { return invokeBook<void>('toggle_window_visible') }
export function exitApp(): Promise<void> { return invokeBook<void>('exit_app') }
