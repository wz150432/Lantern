<script setup lang="ts">
import type { BookRecord } from '../types'
import CoverPlaceholder from './CoverPlaceholder.vue'
import { zh } from '../i18n/zh'

const props = defineProps<{ book: BookRecord; showActions?: boolean }>()
const emit = defineEmits<{
  open: [BookRecord]
  addToShelf: [BookRecord]
  remove: [BookRecord]
  showInFolder: [BookRecord]
}>()
const pct = Math.round(props.book.progress * 100)
</script>

<template>
  <div class="book-card" @click="emit('open', book)" @contextmenu.prevent="emit('open', book)">
    <CoverPlaceholder v-if="!book.coverPath" :title="book.title" />
    <img v-else :src="`asset://localhost/${book.coverPath}`" class="cover-img" alt="" />
    <div class="info">
      <div class="title" :title="book.title">{{ book.title }}</div>
      <div class="progress"><div class="bar" :style="{ width: pct + '%' }"></div></div>
    </div>
    <div v-if="showActions" class="actions">
      <button @click.stop="emit('addToShelf', book)">{{ zh.shelf.addToShelf }}</button>
      <button @click.stop="emit('remove', book)">{{ zh.shelf.remove }}</button>
      <button @click.stop="emit('showInFolder', book)">{{ zh.shelf.showInFolder }}</button>
    </div>
  </div>
</template>

<style scoped>
.book-card {
  position: relative;
  width: 140px;
  cursor: pointer;
}
.info {
  padding: 6px 2px;
}
.title {
  font-size: 13px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.progress {
  height: 3px;
  background: var(--border);
  border-radius: 2px;
  margin-top: 4px;
}
.bar {
  height: 100%;
  background: var(--accent);
  border-radius: 2px;
}
.actions {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.35);
  border-radius: 8px;
  display: none;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 8px;
}
.book-card:hover .actions {
  display: flex;
}
.actions button {
  padding: 4px 10px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}
.cover-img {
  width: 100%;
  aspect-ratio: 3 / 4;
  border-radius: 8px;
  object-fit: cover;
}
</style>

