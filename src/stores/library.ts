import { defineStore } from 'pinia'
import { ref } from 'vue'
import * as ipc from '../ipc'
import type { BookRecord } from '../types'

export const useLibraryStore = defineStore('library', () => {
  const books = ref<BookRecord[]>([])
  async function refresh() { books.value = await ipc.listLibrary() }
  async function open(path: string): Promise<BookRecord> {
    const rec = await ipc.openBook(path)
    await refresh()
    return rec
  }
  async function importBook(path: string, mode: 'linked' | 'copied') {
    await ipc.importBook(path, mode)
    await refresh()
  }
  async function remove(id: number, deleteCopy: boolean) {
    await ipc.deleteBook(id, deleteCopy)
    await refresh()
  }
  return { books, refresh, open, importBook, remove }
})
