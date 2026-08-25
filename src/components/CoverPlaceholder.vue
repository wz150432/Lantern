<script setup lang="ts">
const props = defineProps<{ title: string }>()
const hue = [...props.title].reduce((a, c) => a + c.charCodeAt(0), 0) % 360
</script>

<template>
  <div class="cover" :style="{ background: `linear-gradient(135deg, hsl(${hue} 35% 45%), hsl(${(hue + 40) % 360} 40% 30%))` }">
    <span class="cover-title">{{ title }}</span>
  </div>
</template>

<style scoped>
.cover {
  width: 100%;
  aspect-ratio: 3 / 4;
  border-radius: 8px;
  display: flex;
  align-items: flex-end;
  padding: 10px;
}
.cover-title {
  color: rgba(255, 255, 255, 0.95);
  font-size: 14px;
  font-weight: 600;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
