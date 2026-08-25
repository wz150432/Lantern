<script setup lang="ts">
import type { BookRecord } from '../types'
import CoverPlaceholder from './CoverPlaceholder.vue'

import { computed } from 'vue'

const props = defineProps<{ book: BookRecord }>()
const emit = defineEmits<{ open: [BookRecord] }>()
const pct = computed(() => Math.round(props.book.progress * 100))
</script>

<template>
  <div class="book-card" @click="emit('open', book)" @contextmenu.prevent="emit('open', book)">
    <CoverPlaceholder v-if="!book.coverPath" :title="book.title" />
    <img v-else :src="`asset://localhost/${book.coverPath}`" class="cover-img" alt="" />
    <div class="info">
      <div class="title" :title="book.title">{{ book.title }}</div>
      <div class="meta"><span class="pct">{{ pct }}%</span></div>
      <div class="progress"><div class="bar" :style="{ width: pct + '%' }"></div></div>
    </div>
  </div>
</template>

<style scoped>
.book-card {
  position: relative;
  width: 140px;
  cursor: pointer;
  transition: transform 0.15s ease;
}
.book-card:hover {
  transform: translateY(-2px);
}
.book-card:hover .cover,
.book-card:hover .cover-img {
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.2);
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
.meta {
  display: flex;
  justify-content: flex-end;
  margin-top: 2px;
}
.pct {
  font-size: 11px;
  color: var(--text-dim);
}
.progress {
  height: 3px;
  background: var(--border);
  border-radius: 2px;
  margin-top: 2px;
}
.bar {
  height: 100%;
  background: var(--accent);
  border-radius: 2px;
}
.cover-img {
  width: 100%;
  aspect-ratio: 3 / 4;
  border-radius: 8px;
  object-fit: cover;
}
</style>

