<script setup lang="ts">
import { useReaderStore } from '../stores/reader'
import { zh } from '../i18n/zh'
const reader = useReaderStore()
const emit = defineEmits<{ jump: [number] }>()
</script>
<template>
  <div class="toc">
    <div class="toc-head">{{ zh.reader.chapters }}（{{ reader.chapters.length }}）</div>
    <ul class="toc-list">
      <li v-for="c in reader.chapters" :key="c.index"
          :class="{ active: c.index === reader.currentChapter }"
          @click="emit('jump', c.index)">
        <span class="toc-title" :title="c.title">{{ c.title }}</span>
      </li>
    </ul>
  </div>
</template>
<style scoped>
.toc { display: flex; flex-direction: column; height: 100%; }
.toc-head { padding: 10px 14px; font-weight: 600; border-bottom: 1px solid var(--border); }
.toc-list { flex: 1; overflow-y: auto; list-style: none; }
.toc-list li { padding: 6px 14px; cursor: pointer; }
.toc-list li:hover { background: var(--border); }
.toc-list li.active { background: var(--accent); color: #fff; }
.toc-title { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; display: block; }
</style>