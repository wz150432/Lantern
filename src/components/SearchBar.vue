<script setup lang="ts">
import { ref } from 'vue'
import { useReaderStore } from '../stores/reader'
import * as ipc from '../ipc'
import type { SearchHit } from '../types'
import { zh } from '../i18n/zh'

const reader = useReaderStore()
const emit = defineEmits<{
  close: []
  update: [{ query: string; hits: SearchHit[] }]
}>()

const query = ref('')
const hits = ref<SearchHit[]>([])
const cursor = ref(0)

async function runSearch() {
  const q = query.value.trim()
  if (!q) {
    hits.value = []
    cursor.value = 0
    emit('update', { query: '', hits: [] })
    return
  }
  hits.value = await ipc.searchText(reader.bookId, q, 500)
  cursor.value = 0
  emit('update', { query: q, hits: hits.value })
  if (hits.value.length) await jumpTo(hits.value[0])
}

async function jumpTo(h: SearchHit) {
  await reader.loadChapter(h.chapterIndex)
  reader.chapterProgress = 0
  emit('close')
}

function next() {
  if (!hits.value.length) return
  cursor.value = (cursor.value + 1) % hits.value.length
  void jumpTo(hits.value[cursor.value])
}

function prev() {
  if (!hits.value.length) return
  cursor.value = (cursor.value - 1 + hits.value.length) % hits.value.length
  void jumpTo(hits.value[cursor.value])
}

function onClose() {
  query.value = ''
  hits.value = []
  cursor.value = 0
  emit('update', { query: '', hits: [] })
  emit('close')
}
</script>

<template>
  <div class="searchbar">
    <input v-model="query" :placeholder="zh.reader.searchPlaceholder" @keydown.enter="runSearch" />
    <button @click="runSearch">{{ zh.reader.searchButton }}</button>
    <span class="count">{{ hits.length ? `${cursor + 1}/${hits.length}` : zh.reader.noResult }}</span>
    <button :disabled="!hits.length" @click="prev">{{ zh.reader.searchPrev }}</button>
    <button :disabled="!hits.length" @click="next">{{ zh.reader.searchNext }}</button>
    <button @click="onClose">{{ zh.common.close }}</button>
  </div>
</template>

<style scoped>
.searchbar { position: absolute; top: 52px; right: 20px; z-index: 20; display: flex; gap: 8px; align-items: center; background: var(--panel); border: 1px solid var(--border); border-radius: 8px; padding: 8px 12px; box-shadow: 0 4px 16px rgba(0,0,0,.08); }
.searchbar input { width: 220px; padding: 5px 8px; border: 1px solid var(--border); border-radius: 6px; }
.searchbar button { border: 1px solid var(--border); background: none; border-radius: 6px; padding: 4px 10px; cursor: pointer; }
.searchbar .count { color: var(--text-dim); font-size: 12px; min-width: 60px; text-align: center; }
</style>
