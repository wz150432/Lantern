<template>
  <div class="shell">
    <main class="content"><router-view /></main>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { useLibraryStore } from './stores/library'

const router = useRouter()
const lib = useLibraryStore()
let unlisten: (() => void) | undefined

onMounted(async () => {
  unlisten = await getCurrentWindow().onDragDropEvent(async (e) => {
    if (e.payload.type === 'drop') {
      const path = e.payload.paths[0]
      if (path) {
        const rec = await lib.open(path)
        router.push({ path: '/reader', query: { id: String(rec.id) } })
      }
    }
  })
})
onUnmounted(() => { unlisten?.() })
</script>

<style>
.shell { height: 100%; display: flex; }
.content { flex: 1; min-width: 0; position: relative; }
</style>