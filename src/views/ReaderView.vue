<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useReaderStore } from '../stores/reader'
import { useSettingsStore } from '../stores/settings'
import { useLibraryStore } from '../stores/library'
import { toParagraphs } from '../reader/text'
import { pageCount, pageIndexFromScroll, scrollLeftFromPage } from '../reader/pagination'
import { useHotkeys } from '../composables/useHotkeys'
import { zh } from '../i18n/zh'
import TocPanel from '../components/TocPanel.vue'
import BookmarkPanel from '../components/BookmarkPanel.vue'
import SearchBar from '../components/SearchBar.vue'
import * as ipc from '../ipc'
import type { SearchHit } from '../types'

const route = useRoute()
const router = useRouter()
const reader = useReaderStore()
const settings = useSettingsStore()
const lib = useLibraryStore()

const scrollEl = ref<HTMLElement | null>(null)
const showToc = ref(false)
const showBookmarks = ref(false)
const showSearch = ref(false)
const isFullscreen = ref(false)
const activeSearch = ref<{ query: string; hits: SearchHit[] } | null>(null)

const paragraphs = computed(() =>
  toParagraphs(reader.chapterText, { compressBlankLines: settings.settings?.compressBlankLines ?? true })
    .filter((p) => p.text !== ''),
)

function pageWidth(): number {
  const el = scrollEl.value
  return el ? el.clientWidth : 0
}

function currentPage(): number {
  const el = scrollEl.value
  if (!el) return 0
  if (settings.settings?.pageMode === 'page') return pageIndexFromScroll(el.scrollLeft, pageWidth())
  return 0
}

function goToPage(page: number) {
  const el = scrollEl.value
  if (!el) return
  el.scrollTo({ left: scrollLeftFromPage(page, pageWidth()), behavior: 'smooth' })
  syncProgressFromScroll()
}

let persistTimer: ReturnType<typeof setTimeout> | null = null
function schedulePersist() {
  if (persistTimer) clearTimeout(persistTimer)
  persistTimer = setTimeout(() => { void reader.persist() }, 300)
}

function syncProgressFromScroll() {
  const el = scrollEl.value
  if (!el) return
  if (settings.settings?.pageMode === 'page') {
    const total = pageCount(el.scrollWidth || 1, pageWidth() || 1)
    const page = pageIndexFromScroll(el.scrollLeft, pageWidth() || 1)
    reader.chapterProgress = total > 1 ? page / (total - 1) : 1
  } else {
    const max = el.scrollHeight - el.clientHeight
    reader.chapterProgress = max > 0 ? el.scrollTop / max : 1
  }
  schedulePersist()
}

async function resetScrollToTop() {
  await nextTick()
  const el = scrollEl.value
  if (!el) return
  if (settings.settings?.pageMode === 'page') el.scrollTo({ left: 0 })
  else el.scrollTop = 0
}

function isAtEndOfChapter(): boolean {
  const el = scrollEl.value
  if (!el) return false
  if (settings.settings?.pageMode === 'page') {
    return currentPage() >= pageCount(el.scrollWidth, pageWidth()) - 1
  }
  return el.scrollTop + el.clientHeight >= el.scrollHeight - 1
}

function isAtStartOfChapter(): boolean {
  const el = scrollEl.value
  if (!el) return false
  if (settings.settings?.pageMode === 'page') return currentPage() <= 0
  return el.scrollTop <= 0
}

async function advanceChapterOrStop() {
  if (reader.currentChapter >= reader.chapters.length - 1) {
    if (reader.autoPageOn) toggleAutoPage()
    return
  }
  await reader.nextChapter()
  await resetScrollToTop()
}

function nextPage() {
  const el = scrollEl.value
  if (!el) return
  if (isAtEndOfChapter()) { void advanceChapterOrStop(); return }
  if (settings.settings?.pageMode === 'page') {
    goToPage(currentPage() + 1)
  } else {
    el.scrollBy({ top: el.clientHeight, behavior: 'smooth' })
    syncProgressFromScroll()
  }
}

function prevPage() {
  const el = scrollEl.value
  if (!el) return
  if (isAtStartOfChapter()) {
    if (reader.currentChapter > 0) void reader.prevChapter().then(resetScrollToTop)
    return
  }
  if (settings.settings?.pageMode === 'page') {
    goToPage(currentPage() - 1)
  } else {
    el.scrollBy({ top: -el.clientHeight, behavior: 'smooth' })
    syncProgressFromScroll()
  }
}

let autoTimer: ReturnType<typeof setInterval> | null = null
function toggleAutoPage() {
  reader.autoPageOn = !reader.autoPageOn
  if (reader.autoPageOn) {
    autoTimer = setInterval(() => {
      const el = scrollEl.value
      if (!el) return
      if (isAtEndOfChapter()) {
        if (reader.currentChapter >= reader.chapters.length - 1) { toggleAutoPage(); return }
        void reader.nextChapter().then(resetScrollToTop)
        return
      }
      nextPage()
    }, settings.settings?.autoPageIntervalMs ?? 3000)
  } else if (autoTimer) {
    clearInterval(autoTimer)
    autoTimer = null
  }
}

async function toggleFullscreen() {
  isFullscreen.value = !isFullscreen.value
  await ipc.setFullscreen(isFullscreen.value)
}

async function toggleImmersive() {
  const next = !settings.settings!.immersiveMode
  await settings.update({ immersiveMode: next })
  // 退出沉浸恢复装饰；透明度 < 1 时保留窗口装饰（macOS 透明 + 无边框组合受限，属已知降级）
  const decorated = !next || settings.settings!.windowOpacity < 1
  await ipc.setDecorations(decorated)
}

async function toggleTopmost() {
  const next = !settings.settings!.windowTopmost
  await settings.update({ windowTopmost: next })
  await ipc.setTopmost(next)
}

function toggleSearch() {
  showSearch.value = !showSearch.value
  if (!showSearch.value) activeSearch.value = null
}

function onSearchUpdate(payload: { query: string; hits: SearchHit[] }) {
  activeSearch.value = payload.query.trim() ? payload : null
}

function closeSearch() { showSearch.value = false }

function isMarked(text: string): boolean {
  const s = activeSearch.value
  if (!s || !s.query.trim()) return false
  if (!s.hits.some((h) => h.chapterIndex === reader.currentChapter)) return false
  return text.includes(s.query)
}

function jumpPercent() {
  const p = Number(prompt(zh.reader.progress, '50'))
  if (Number.isNaN(p)) return
  const target = Math.min(100, Math.max(0, p)) / 100
  const total = reader.chapters.length
  if (total === 0) return
  const ch = Math.min(Math.floor(target * total), total - 1)
  const frac = target * total - ch
  void reader.loadChapter(ch).then(async () => {
    await nextTick()
    const el = scrollEl.value
    if (!el) return
    if (settings.settings?.pageMode === 'page') {
      const pages = pageCount(el.scrollWidth, pageWidth())
      goToPage(Math.round(frac * (pages - 1)))
    } else {
      el.scrollTop = frac * Math.max(0, el.scrollHeight - el.clientHeight)
      syncProgressFromScroll()
    }
    void reader.persist()
  })
}

function zoom(delta: number) {
  const s = settings.settings!
  void settings.update({ fontSize: Math.min(48, Math.max(10, s.fontSize + delta)) })
}

function onWheel(e: WheelEvent) {
  // Ctrl/Alt + 滚轮调整窗口透明度（v0.1 统一按同一行为处理）
  if (!e.ctrlKey && !e.altKey) return
  e.preventDefault()
  const s = settings.settings!
  const next = Math.min(1, Math.max(0.3, s.windowOpacity + (e.deltaY > 0 ? -0.05 : 0.05)))
  void settings.update({ windowOpacity: next })
  // R6：v0.1 不实际改变窗口透明度，尽力持久化设置，失败忽略
  void ipc.setOpacity(next).catch(() => { /* ignore */ })
}

async function pickAndOpen() {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const picked = await open({ multiple: false, filters: [{ name: zh.shelf.bookFilter, extensions: ['txt'] }] })
  if (!picked) return
  const rec = await lib.open(picked as string)
  router.push({ path: '/reader', query: { id: String(rec.id) } })
}

useHotkeys({
  nextPage, prevPage,
  nextChapter: () => void reader.nextChapter().then(resetScrollToTop),
  prevChapter: () => void reader.prevChapter().then(resetScrollToTop),
  toggleFullscreen, toggleImmersive, toggleAutoPage, toggleSearch, jumpPercent,
  addBookmark: () => void reader.addBookmarkHere(),
  openFile: pickAndOpen,
  zoomIn: () => zoom(1), zoomOut: () => zoom(-1), toggleTopmost,
})

function onReaderClick(e: MouseEvent) {
  const el = scrollEl.value
  if (!el) return
  const mode = settings.settings?.clickMode ?? 'thirds'
  if (mode === 'left-right') { nextPage(); return }
  const rect = el.getBoundingClientRect()
  const x = e.clientX - rect.left
  if (x < rect.width / 3) prevPage()
  else if (x > (rect.width * 2) / 3) nextPage()
}

function onReaderContextMenu(e: MouseEvent) {
  if (settings.settings?.clickMode !== 'left-right') return
  e.preventDefault()
  prevPage()
}

function onTocJump(i: number) {
  showToc.value = false
  void (async () => {
    await reader.loadChapter(i)
    await resetScrollToTop()
    void reader.persist()
  })()
}

function restoreProgress() {
  requestAnimationFrame(() => {
    const el = scrollEl.value
    if (!el) return
    if (settings.settings?.pageMode === 'page') {
      const total = pageCount(el.scrollWidth || 1, pageWidth() || 1)
      const page = Math.round(reader.chapterProgress * Math.max(0, total - 1))
      el.scrollTo({ left: scrollLeftFromPage(page, pageWidth()) })
    } else {
      el.scrollTop = reader.chapterProgress * Math.max(0, el.scrollHeight - el.clientHeight)
    }
  })
}

onMounted(async () => {
  if (!settings.settings) await settings.load()
  const id = Number(route.query.id)
  let book = lib.books.find((b) => b.id === id)
  if (!book) {
    await lib.refresh()
    book = lib.books.find((b) => b.id === id)
  }
  if (!book) { void router.push('/'); return }
  await reader.open(book.id, book.filePath, book.currentChapter, book.progress)
  await nextTick()
  restoreProgress()
})

onBeforeUnmount(() => {
  if (autoTimer) clearInterval(autoTimer)
  if (persistTimer) clearTimeout(persistTimer)
  if (reader.bookId) void reader.persist()
})
</script>

<template>
  <div class="reader" :class="{ immersive: settings.settings?.immersiveMode }">
    <header class="topbar">
      <button class="link" @click="router.push('/')">{{ zh.reader.back }}</button>
      <span class="chapter-title">{{ reader.chapters[reader.currentChapter]?.title ?? '' }}</span>
      <div class="top-actions">
        <button @click="showToc = !showToc">{{ zh.reader.chapters }}</button>
        <button @click="showBookmarks = !showBookmarks">{{ zh.reader.bookmarks }}</button>
        <button @click="toggleSearch">{{ zh.reader.search }}</button>
        <button @click="toggleFullscreen">{{ zh.reader.fullscreen }}</button>
        <button @click="toggleImmersive">{{ zh.reader.immersive }}</button>
        <button @click="router.push('/settings')">{{ zh.settings.title }}</button>
      </div>
    </header>

    <aside v-if="showToc" class="panel left">
      <TocPanel @jump="onTocJump" />
    </aside>
    <aside v-if="showBookmarks" class="panel left">
      <BookmarkPanel @close="showBookmarks = false" />
    </aside>

    <div v-show="showSearch" class="searchbar-slot">
      <SearchBar @close="closeSearch" @update="onSearchUpdate" />
    </div>

    <main
      ref="scrollEl"
      class="reader-scroll"
      :class="{ scroll: settings.settings?.pageMode === 'scroll' }"
      @click="onReaderClick" @wheel="onWheel"
      @contextmenu="onReaderContextMenu"
      @scroll="syncProgressFromScroll"
    >
      <div class="page-body">
        <p
          v-for="(p, i) in paragraphs"
          :key="i"
          :class="{ heading: p.isHeading, marked: isMarked(p.text) }"
        >{{ p.text }}</p>
      </div>
    </main>

    <footer class="bottombar">
      <span class="pct">{{ Math.round(reader.overallProgress() * 100) }}%</span>
      <button @click="reader.prevChapter()">{{ zh.reader.prevChapter }}</button>
      <button @click="toggleAutoPage">{{ reader.autoPageOn ? zh.reader.autoStop : zh.reader.autoStart }}</button>
      <button @click="reader.addBookmarkHere()">{{ zh.reader.addBookmark }}</button>
      <button @click="jumpPercent()">{{ zh.reader.progress }}</button>
      <button @click="reader.nextChapter()">{{ zh.reader.nextChapter }}</button>
    </footer>
  </div>
</template>

<style scoped>
.reader {
  position: relative;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--reader-bg);
  color: var(--reader-text);
}
.topbar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 10px 20px;
  background: var(--panel);
  border-bottom: 1px solid var(--border);
  z-index: 20;
}
.topbar .link {
  border: none;
  background: none;
  color: var(--accent);
  cursor: pointer;
  font-size: 14px;
}
.chapter-title {
  flex: 1;
  text-align: center;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 14px;
  color: var(--text-dim);
}
.top-actions {
  display: flex;
  gap: 8px;
}
.top-actions button,
.bottombar button {
  padding: 4px 10px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--panel);
  color: var(--text);
  cursor: pointer;
  font-size: 13px;
}
.immersive .topbar,
.immersive .bottombar {
  display: none;
}
.reader-scroll {
  flex: 1;
  height: 0;
  overflow-x: auto;
  overflow-y: hidden;
  column-width: calc(100% - var(--reader-padding) * 2);
  column-gap: 48px;
  column-fill: auto;
  padding: 0 var(--reader-padding);
}
.reader-scroll.scroll {
  column-width: auto;
  column-count: 1;
  overflow-x: hidden;
  overflow-y: auto;
}
.page-body {
  font-family: var(--reader-font-family);
  font-size: var(--reader-font-size);
  line-height: var(--reader-line-height);
  letter-spacing: var(--reader-char-spacing);
  word-break: var(--reader-word-wrap);
  padding: 24px 0;
}
.page-body p {
  margin: 0 0 var(--reader-para-spacing) 0;
  text-indent: 2em;
}
.page-body p.marked { background: rgba(255, 213, 0, .35); border-radius: 2px; }
.page-body p.heading {
  font-weight: 700;
  text-align: center;
  text-indent: 0;
  font-size: 1.15em;
  margin: 1.2em 0;
}
.bottombar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 20px;
  background: var(--panel);
  border-top: 1px solid var(--border);
}
.bottombar .pct {
  font-size: 13px;
  color: var(--text-dim);
  min-width: 48px;
}
.searchbar-slot {
  /* SearchBar 自行以 .reader 为包含块定位（.searchbar 为 absolute） */
}
.panel {
  position: absolute;
  top: 52px;
  bottom: 44px;
  left: 0;
  width: 280px;
  background: var(--panel);
  border-right: 1px solid var(--border);
  z-index: 10;
  display: flex;
  flex-direction: column;
}
</style>
