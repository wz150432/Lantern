<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
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
const showJump = ref(false)
const jumpValue = ref('50')
const jumpInput = ref<HTMLInputElement | null>(null)

const indentStyle = computed(() => ({
  textIndent: settings.settings?.firstLineIndent ? '2em' : '0',
}))

const COLUMN_GAP = 48
const toast = ref('')
let toastTimer: ReturnType<typeof setTimeout> | null = null
let resizeObserver: ResizeObserver | null = null
let lastWheelTurn = 0
let hideButtonMask = 0

function showToast(msg: string) {
  toast.value = msg
  if (toastTimer) clearTimeout(toastTimer)
  toastTimer = setTimeout(() => { toast.value = '' }, 1500)
}

const paragraphs = computed(() =>
  toParagraphs(reader.chapterText, { compressBlankLines: settings.settings?.compressBlankLines ?? true }),
)

function pageWidth(): number {
  const el = scrollEl.value
  if (!el) return 0
  const padding = settings.settings?.innerPadding ?? 48
  const contentWidth = el.clientWidth - padding * 2
  return Math.max(100, contentWidth) + COLUMN_GAP
}

function applyColumns() {
  const el = scrollEl.value
  if (!el) return
  if (settings.settings?.pageMode === 'page') {
    el.classList.add('page-mode')
    const padding = settings.settings?.innerPadding ?? 48
    const contentWidth = el.clientWidth - padding * 2
    el.style.columnWidth = `${Math.max(100, contentWidth)}px`
  } else {
    el.classList.remove('page-mode')
    el.style.columnWidth = ''
  }
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

function openJump() {
  jumpValue.value = String(Math.round(reader.overallProgress() * 100))
  showJump.value = true
  void nextTick(() => jumpInput.value?.focus())
}

function closeJump() {
  showJump.value = false
}

function doJump(value: number) {
  const target = Math.min(100, Math.max(0, value)) / 100
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
  // Ctrl/Alt + 滚轮：调整窗口透明度（CSS 层面生效）
  if (e.ctrlKey || e.altKey) {
    e.preventDefault()
    const s = settings.settings!
    const next = Math.min(1, Math.max(0.3, s.windowOpacity + (e.deltaY > 0 ? -0.05 : 0.05)))
    void settings.update({ windowOpacity: next })
    void ipc.setOpacity(next).catch(() => { /* ignore */ })
    return
  }
  // 翻页模式：滚轮上下翻页（节流）；滚动模式：交给默认滚动
  if (settings.settings?.pageMode !== 'page') return
  e.preventDefault()
  const now = Date.now()
  if (now - lastWheelTurn < 350) return
  lastWheelTurn = now
  if (e.deltaY > 0) nextPage()
  else prevPage()
}

function onReaderMouseDown(e: MouseEvent) {
  if (e.button === 0) hideButtonMask |= 1
  else if (e.button === 2) hideButtonMask |= 2
  if (hideButtonMask === 3) {
    hideButtonMask = 0
    void ipc.toggleWindowVisible()
  }
}
function onReaderMouseUp() { hideButtonMask = 0 }

async function addBookmarkWithFeedback() {
  try {
    await reader.addBookmarkHere()
    showToast(zh.reader.bookmarkAdded)
  } catch (err) {
    console.error('add bookmark failed:', err)
  }
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
  toggleFullscreen, toggleImmersive, toggleAutoPage, toggleSearch, jumpPercent: openJump,
  addBookmark: () => void addBookmarkWithFeedback(),
  toggleWindowVisible: () => void ipc.toggleWindowVisible(),
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

watch(
  () => [settings.settings?.pageMode, settings.settings?.innerPadding],
  () => { void nextTick(applyColumns) },
)

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
  applyColumns()
  if (!resizeObserver && scrollEl.value) {
    resizeObserver = new ResizeObserver(() => applyColumns())
    resizeObserver.observe(scrollEl.value)
  }
  restoreProgress()
})

onBeforeUnmount(() => {
  if (autoTimer) clearInterval(autoTimer)
  if (persistTimer) clearTimeout(persistTimer)
  resizeObserver?.disconnect()
  if (reader.bookId) void reader.persist()
})
</script>

<template>
  <div class="reader" :class="{ immersive: settings.settings?.immersiveMode }" @mousedown="onReaderMouseDown" @mouseup="onReaderMouseUp">
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
      :class="{ 'page-mode': settings.settings?.pageMode === 'page' }"
      @click="onReaderClick" @wheel="onWheel"
      @contextmenu="onReaderContextMenu"
      @scroll="syncProgressFromScroll"
    >
      <div class="page-body" :style="indentStyle">
        <p
          v-for="(p, i) in paragraphs"
          :key="i"
          :class="{ heading: p.isHeading, marked: isMarked(p.text) }"
        >{{ p.text }}</p>
      </div>
    </main>

    <div v-if="showJump" class="jump-overlay">
      <input
        ref="jumpInput"
        v-model="jumpValue"
        type="number"
        min="0"
        max="100"
        :placeholder="zh.reader.jumpPlaceholder"
        @keydown.enter.prevent="doJump(Number(jumpValue)); closeJump()"
        @keydown.esc.prevent="closeJump"
        @blur="closeJump"
      />
    </div>

    <div v-if="toast" class="toast">{{ toast }}</div>

    <footer class="bottombar">
      <span class="pct">{{ Math.round(reader.overallProgress() * 100) }}%</span>
      <button @click="reader.prevChapter()">{{ zh.reader.prevChapter }}</button>
      <button @click="toggleAutoPage">{{ reader.autoPageOn ? zh.reader.autoStop : zh.reader.autoStart }}</button>
      <button @click="addBookmarkWithFeedback()">{{ zh.reader.addBookmark }}</button>
      <button @click="openJump">{{ zh.reader.progress }}</button>
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
  overflow-x: hidden;
  overflow-y: auto;
  padding: 0 var(--reader-padding);
}
.reader-scroll.page-mode {
  overflow-x: auto;
  overflow-y: hidden;
  column-gap: 48px;
  column-fill: auto;
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
.jump-overlay {
  position: absolute;
  left: 0;
  right: 0;
  bottom: 52px;
  display: flex;
  justify-content: center;
  z-index: 30;
}
.jump-overlay input {
  width: 180px;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 8px;
  background: var(--panel);
  color: var(--text);
  font-size: 14px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, .2);
}
.toast {
  position: absolute;
  top: 60px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, .72);
  color: #fff;
  padding: 8px 16px;
  border-radius: 8px;
  z-index: 40;
  font-size: 13px;
  pointer-events: none;
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
