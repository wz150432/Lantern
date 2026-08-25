export const zh = {
  appName: 'Lantern',
  shelf: {
    title: '书架', all: '全部', recent: '最近阅读', unfinished: '未读完', bookmarked: '书签',
    importBook: '导入书籍', openFile: '打开文件', searchPlaceholder: '搜索书架…', bookFilter: '书籍',
    importPrompt: '复制进书库？确定复制，取消则链接原文件（不移动原文件）', deleteCopyPrompt: '同时删除书库副本文件？',
    empty: '书架还是空的，点击"导入书籍"或"打开文件"开始阅读',
    recentBar: '最近阅读', addToShelf: '加入书架', remove: '删除记录', showInFolder: '在文件夹中显示',
  },
  reader: {
    back: '返回书架', chapters: '目录', bookmarks: '书签', prevChapter: '上一章', nextChapter: '下一章',
    search: '查找', searchButton: '搜索', searchPlaceholder: '输入关键字…', searchPrev: '上一个', searchNext: '下一个', noResult: '没有找到匹配内容',
    progress: '进度', jumpPlaceholder: '输入跳转百分比（0-100）', bookmarkAdded: '已添加书签', hideBorder: '隐藏边框', showBorder: '显示边框', editMode: '编辑模式',
    fileMissing: '文件缺失', relocate: '重新定位文件',
    fullscreen: '全屏', immersive: '沉浸', exitImmersive: '退出沉浸', autoStart: '自动', autoStop: '停止', addBookmark: '书签',
    noBookmarks: '暂无书签（Ctrl+M 添加）',
  },
  settings: {
    title: '设置', display: '显示设置', typography: '排版设置', reading: '阅读设置', hotkeys: '按键设置', window: '窗口', advanced: '高级',
    theme: '主题', fontFamily: '字体', fontSize: '字号', textColor: '文字颜色',
    backgroundColor: '背景颜色', lineHeight: '行距', paragraphSpacing: '段距',
    firstLineIndent: '首行缩进', charSpacing: '字距', compressBlankLines: '压缩空行',
    wordWrap: '英文自动换行', innerPadding: '内部边距', pageMode: '翻页模式',
    pageModePage: '翻页', pageModeScroll: '滚动', autoPageInterval: '自动翻页间隔(ms)',
    clickMode: '点击翻页方式', clickThirds: '左右区域', clickLeftRight: '左键下页/右键上页',
    scrollSpeed: '滚动速度', windowTopmost: '窗口置顶', windowOpacity: '窗口透明度',
    immersiveMode: '沉浸模式（F12 隐藏顶栏/底栏）', pageDouble: '翻页双页（左右双页）', autoHideOnLeave: '鼠标移出自动隐藏（Ctrl+Alt+Shift+P 切换）', recording: '按新组合键…', hotkeysHint: '点击右侧按键后按下新的组合键即可录制（Esc 取消）', resetHotkeys: '还原默认快捷键', autoHideOn: '已开启鼠标移出自动隐藏', autoHideOff: '已关闭鼠标移出自动隐藏', restoreDefault: '还原默认设置', about: '关于',
    preview: '排版预览', resetConfirm: '确定要还原默认设置吗？',
    fontSystem: '系统默认', fontSerif: '思源宋体', fontNotoSerif: 'Noto Serif',
    previewChapter: '第一章 预览', previewBody: '这是排版预览文本，用于实时查看行距、段距、字号与首行缩进的效果。点击设置即刻生效。',
    version: 'v0.1.0',
  },
  hotkeyActions: {
    nextPage: '下一页', prevPage: '上一页', nextChapter: '下一章', prevChapter: '上一章',
    scrollUp: '向上滚动一行', scrollDown: '向下滚动一行', toggleSearch: '查找', jumpPercent: '跳转进度',
    addBookmark: '添加书签', openFile: '打开文件', toggleTopmost: '窗口置顶', zoomIn: '放大', zoomOut: '缩小',
    toggleAutoPage: '自动翻页', toggleFullscreen: '全屏', toggleImmersive: '隐藏顶栏/底栏', toggleWindowVisible: '隐藏/显示窗口', toggleAutoHide: '自动隐藏开关',
  },
  common: { confirm: '确定', cancel: '取消', delete: '删除', close: '关闭', exit: '退出' },
}
export type ZhDict = typeof zh

