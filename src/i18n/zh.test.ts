import { describe, it, expect } from 'vitest'
import { zh } from './zh'

describe('zh dict', () => {
  it('has app name', () => {
    expect(zh.appName).toBe('Lantern')
  })
  it('has shelf copy', () => {
    expect(zh.shelf.title).toBe('书架')
    expect(zh.shelf.importBook).toBe('导入书籍')
    expect(zh.shelf.empty).toContain('导入书籍')
  })
  it('has reader copy', () => {
    expect(zh.reader.back).toBe('返回书架')
    expect(zh.reader.prevChapter).toBe('上一章')
    expect(zh.reader.nextChapter).toBe('下一章')
  })
  it('has settings copy', () => {
    expect(zh.settings.title).toBe('设置')
    expect(zh.settings.theme).toBe('主题')
    expect(zh.settings.immersiveMode).toContain('隐藏顶栏/底栏')
  })
  it('has common copy', () => {
    expect(zh.common.confirm).toBe('确定')
    expect(zh.common.cancel).toBe('取消')
  })
})
