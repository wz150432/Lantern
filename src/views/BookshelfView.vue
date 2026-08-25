<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { open as dialogOpen } from '@tauri-apps/plugin-dialog'
import { revealItemInDir } from '@tauri-apps/plugin-opener'
import { useLibraryStore } from '../stores/library'
import { useSettingsStore } from '../stores/settings'
import { useHotkeys } from '../composables/useHotkeys'
import BookCard from '../components/BookCard.vue'
import type { BookRecord } from '../types'
import { zh } from '../i18n/zh'

const router = useRouter()
const lib = useLibraryStore()
const settings = useSettingsStore()
const filter = ref<'all' | 'recent' | 'unfinished' | 'bookmarked'>('all')
const keyword = ref('')

const filtered = computed(() => {
  let list = lib.books
  if (filter.value === 'unfinished') list = list.filter((b) => b.progress < 1 && b.progress > 0)
  else if (filter.value === 'bookmarked') list = list.filter((b) => b.progress < 1)
  if (keyword.value.trim()) list = list.filter((b) => b.title.includes(keyword.value.trim()))
  return list
})

async function pickBookPath(): Promise<string | null> {
  const picked = await dialogOpen({
    multiple: false,
    filters: [{ name: zh.shelf.bookFilter, extensions: ['txt'] }],
  })
  if (!picked) return null
  return Array.isArray(picked) ? (picked[0] ?? null) : picked
}

async function onOpen(book: BookRecord) {
  const opened = await lib.open(book.filePath)
  router.push({ path: '/reader', query: { id: String(opened.id) } })
}

async function onImport() {
  const path = await pickBookPath()
  if (!path) return
  const mode = window.confirm(zh.shelf.importPrompt) ? 'copied' : 'linked'
  await lib.importBook(path, mode)
}

async function pickAndOpen() {
  const path = await pickBookPath()
  if (!path) return
  const opened = await lib.open(path)
  router.push({ path: '/reader', query: { id: String(opened.id) } })
}

function onRemove(book: BookRecord) {
  const deleteCopy = book.storageMode === 'copied' && window.confirm(zh.shelf.deleteCopyPrompt)
  void lib.remove(book.id, deleteCopy)
}

function onAddToShelf(book: BookRecord) {
  void onOpen(book)
}

async function showInFolder(book: BookRecord) {
  await revealItemInDir(book.filePath)
}

useHotkeys({ openFile: pickAndOpen })

onMounted(async () => {
  await settings.load()
  await lib.refresh()
})
</script>

<template>
  <div class="shelf">
    <header class="shelf-header">
      <h1>{{ zh.appName }}</h1>
      <div class="tools">
        <input v-model="keyword" class="search" :placeholder="zh.shelf.searchPlaceholder" />
        <button class="btn primary" @click="onImport">{{ zh.shelf.importBook }}</button>
        <button class="btn" @click="pickAndOpen">{{ zh.shelf.openFile }}</button>
        <button class="btn" @click="router.push('/settings')">{{ zh.settings.title }}</button>
      </div>
    </header>
    <nav class="filters">
      <button
        v-for="f in (['all', 'recent', 'unfinished', 'bookmarked'] as const)"
        :key="f"
        :class="{ active: filter === f }"
        @click="filter = f"
      >
        {{ zh.shelf[f] }}
      </button>
    </nav>
    <main class="grid">
      <BookCard
        v-for="b in filtered"
        :key="b.id"
        :book="b"
        show-actions
        @open="onOpen"
        @add-to-shelf="onAddToShelf"
        @remove="onRemove"
        @show-in-folder="showInFolder"
      />
    </main>
    <p v-if="lib.books.length === 0" class="empty">{{ zh.shelf.empty }}</p>
  </div>
</template>

<style scoped>
.shelf {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 24px 32px;
  gap: 16px;
}
.shelf-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.tools {
  display: flex;
  gap: 8px;
  align-items: center;
}
.search {
  padding: 6px 10px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--panel);
  color: var(--text);
  width: 200px;
}
.btn {
  padding: 6px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--panel);
  color: var(--text);
  cursor: pointer;
}
.btn.primary {
  background: var(--accent);
  border-color: var(--accent);
  color: #fff;
}
.filters {
  display: flex;
  gap: 8px;
}
.filters button {
  padding: 4px 12px;
  border: none;
  background: none;
  color: var(--text-dim);
  cursor: pointer;
  border-radius: 999px;
}
.filters button.active {
  background: var(--accent);
  color: #fff;
}
.grid {
  flex: 1;
  overflow-y: auto;
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
  gap: 20px;
  align-content: start;
}
.empty {
  color: var(--text-dim);
  text-align: center;
  margin-top: 60px;
}
</style>
