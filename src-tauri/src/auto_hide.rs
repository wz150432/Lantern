//! 鼠标移出窗口自动隐藏（Windows）。
//! 开启后：鼠标在窗口内 → 显示；移出 → 隐藏；隐藏时按住 Ctrl 且光标回到窗口区域 → 重新显示。
#![cfg(target_os = "windows")]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[repr(C)]
struct POINT {
    x: i32,
    y: i32,
}

#[repr(C)]
struct RECT {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

const VK_CONTROL: i32 = 0x11;
const SW_HIDE: i32 = 0;
const SW_SHOW: i32 = 5;

#[link(name = "user32")]
unsafe extern "system" {
    fn GetCursorPos(lpPoint: *mut POINT) -> i32;
    fn GetWindowRect(hWnd: *const core::ffi::c_void, lpRect: *mut RECT) -> i32;
    fn GetAsyncKeyState(vKey: i32) -> i16;
    fn IsWindowVisible(hWnd: *const core::ffi::c_void) -> i32;
    fn ShowWindow(hWnd: *const core::ffi::c_void, nCmdShow: i32) -> i32;
    fn SetForegroundWindow(hWnd: *const core::ffi::c_void) -> i32;
}

pub fn start(hwnd: usize, enabled: Arc<AtomicBool>) {
    std::thread::spawn(move || {
        unsafe {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(150));
                if !enabled.load(Ordering::Relaxed) {
                    continue;
                }
                let mut pt = POINT { x: 0, y: 0 };
                if GetCursorPos(&mut pt) == 0 {
                    continue;
                }
                let mut rc = RECT { left: 0, top: 0, right: 0, bottom: 0 };
                if GetWindowRect(hwnd as *const core::ffi::c_void, &mut rc) == 0 {
                    continue;
                }
                let inside = pt.x >= rc.left && pt.x <= rc.right && pt.y >= rc.top && pt.y <= rc.bottom;
                let ctrl = (GetAsyncKeyState(VK_CONTROL) as u16 & 0x8000) != 0;
                let visible = IsWindowVisible(hwnd as *const core::ffi::c_void) != 0;
                if visible && !inside {
                    ShowWindow(hwnd as *const core::ffi::c_void, SW_HIDE);
                } else if !visible && inside && ctrl {
                    ShowWindow(hwnd as *const core::ffi::c_void, SW_SHOW);
                    SetForegroundWindow(hwnd as *const core::ffi::c_void);
                }
            }
        }
    });
}
