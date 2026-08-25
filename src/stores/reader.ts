import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as ipc from '../ipc'
import type { Bookmark, ChapterInfo } from '../types'

export const useReaderStore = defineStore('reader', () => {
  const bookId = ref(0)
  const bookPath = ref('')
  const chapters = ref<ChapterInfo[]>([])
  const currentChapter = ref(0)
  const chapterText = ref('')
  const progress = ref(0)          // 0~1 书内进度（按章节加权）
  const chapterProgress = ref(0)   // 当前章内进度 0~1
  const bookmarks = ref<Bookmark[]>([])
  const autoPageOn = ref(false)

  async function open(id: number, path: string, savedChapter: number, savedProgress: number) {
    bookId.value = id
    bookPath.value = path
    chapters.value = await ipc.getChapters(id, path)
    currentChapter.value = Math.min(Math.max(0, savedChapter), Math.max(0, chapters.value.length - 1))
    await loadChapter(currentChapter.value)
    bookmarks.value = await ipc.getBookmarks(id)
    if (savedProgress > 0) chapterProgress.value = Math.min(1, savedProgress)
  }

  async function loadChapter(index: number) {
    if (index < 0 || index >= chapters.value.length) return
    currentChapter.value = index
    chapterText.value = await ipc.getChapterText(bookId.value, index)
    chapterProgress.value = 0
  }

  function overallProgress(): number {
    if (chapters.value.length === 0) return 0
    return (currentChapter.value + chapterProgress.value) / chapters.value.length
  }

  async function persist() {
    await ipc.saveProgress(bookId.value, currentChapter.value, chapterProgress.value)
    progress.value = overallProgress()
  }

  async function nextChapter() {
    if (currentChapter.value < chapters.value.length - 1) {
      await loadChapter(currentChapter.value + 1)
      await persist()
    }
  }

  async function prevChapter() {
    if (currentChapter.value > 0) {
      await loadChapter(currentChapter.value - 1)
      await persist()
    }
  }

  async function refreshBookmarks() {
    bookmarks.value = await ipc.getBookmarks(bookId.value)
  }

  async function addBookmarkHere() {
    await ipc.addBookmark(bookId.value, currentChapter.value, chapterProgress.value, null)
    await refreshBookmarks()
  }

  return {
    bookId, bookPath, chapters, currentChapter, chapterText, progress, chapterProgress,
    bookmarks, autoPageOn, open, loadChapter, persist, nextChapter, prevChapter,
    refreshBookmarks, addBookmarkHere, overallProgress,
  }
})
