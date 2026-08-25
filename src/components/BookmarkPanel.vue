<script setup lang="ts">
import { useReaderStore } from '../stores/reader'
import * as ipc from '../ipc'
import { zh } from '../i18n/zh'
const reader = useReaderStore()
const emit = defineEmits<{ close: [] }>()

async function jump(b: { chapterIndex: number; position: number }) {
  await reader.loadChapter(b.chapterIndex)
  reader.chapterProgress = b.position
  emit('close')
}
async function remove(id: number) {
  await ipc.deleteBookmark(id)
  await reader.refreshBookmarks()
}
</script>
<template>
  <div class="bm">
    <div class="bm-head">
      <span>{{ zh.reader.bookmarks }}</span>
      <button @click="emit('close')">{{ zh.common.close }}</button>
    </div>
    <ul class="bm-list">
      <li v-for="b in reader.bookmarks" :key="b.id" @click="jump(b)">
        <span class="bm-ch">{{ reader.chapters[b.chapterIndex]?.title ?? `第${b.chapterIndex + 1}章` }}</span>
        <span class="bm-pct">{{ Math.round(b.position * 100) }}%</span>
        <button class="bm-del" @click.stop="remove(b.id)">{{ zh.common.delete }}</button>
      </li>
      <li v-if="reader.bookmarks.length === 0" class="bm-empty">{{ zh.reader.noBookmarks }}</li>
    </ul>
  </div>
</template>
<style scoped>
.bm { display: flex; flex-direction: column; height: 100%; }
.bm-head { display: flex; justify-content: space-between; align-items: center; padding: 10px 14px; font-weight: 600; border-bottom: 1px solid var(--border); }
.bm-list { flex: 1; overflow-y: auto; list-style: none; }
.bm-list li { display: flex; align-items: center; gap: 8px; padding: 8px 14px; cursor: pointer; }
.bm-list li:hover { background: var(--border); }
.bm-ch { flex: 1; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
.bm-pct { color: var(--text-dim); font-size: 12px; }
.bm-del { border: none; background: none; color: var(--accent); cursor: pointer; font-size: 12px; }
.bm-empty { color: var(--text-dim); padding: 14px; }
</style>