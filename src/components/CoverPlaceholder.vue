<script setup lang="ts">
defineProps<{ title: string }>()
</script>

<template>
  <div class="cover">
    <span class="cover-title">{{ title }}</span>
  </div>
</template>

<style scoped>
.cover {
  width: 100%;
  aspect-ratio: 3 / 4;
  border-radius: 8px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding: 24px 12px 10px;
  background: linear-gradient(135deg, var(--accent), color-mix(in srgb, var(--accent) 62%, #000));
}
.cover-title {
  color: var(--panel);
  font-size: 15px;
  font-weight: 600;
  line-height: 1.4;
  width: 100%;
  text-align: center;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
</style>
