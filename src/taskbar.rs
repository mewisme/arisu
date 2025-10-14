#[cfg(target_os = "windows")]
use windows::{
  core::PCWSTR,
  Win32::{
    Foundation::RECT,
    UI::Shell::{
      SHAppBarMessage, ABE_BOTTOM, ABE_LEFT, ABE_RIGHT, ABE_TOP, ABM_GETTASKBARPOS, APPBARDATA,
    },
    UI::WindowsAndMessaging::{FindWindowW, GetWindowRect},
  },
};

#[derive(Clone, Copy, Debug)]
pub struct TaskbarInfo {
  pub rect: (i32, i32, i32, i32),
  pub edge: TaskbarEdge,
  pub size: (i32, i32),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskbarEdge {
  Top,
  Bottom,
  Left,
  Right,
}

#[cfg(target_os = "windows")]
fn wide(s: &str) -> Vec<u16> {
  use std::os::windows::prelude::OsStrExt;
  std::ffi::OsStr::new(s)
    .encode_wide()
    .chain(std::iter::once(0))
    .collect()
}

#[cfg(target_os = "windows")]
pub fn get_taskbar_info() -> Option<TaskbarInfo> {
  unsafe {
    let class_name = wide("Shell_TrayWnd");
    let hwnd = FindWindowW(PCWSTR::from_raw(class_name.as_ptr()), PCWSTR::null());
    if hwnd.0 == 0 {
      return None;
    }

    let mut abd = APPBARDATA::default();
    abd.hWnd = hwnd;
    let ok = SHAppBarMessage(ABM_GETTASKBARPOS, &mut abd);
    if ok == 0 {
      let mut rc = RECT::default();
      if GetWindowRect(hwnd, &mut rc).is_ok() {
        let w = rc.right - rc.left;
        let h = rc.bottom - rc.top;
        return Some(TaskbarInfo {
          rect: (rc.left, rc.top, rc.right, rc.bottom),
          edge: if w >= h {
            TaskbarEdge::Bottom
          } else {
            TaskbarEdge::Left
          },
          size: (w, h),
        });
      }
      return None;
    }

    let rc = abd.rc;
    let edge = match abd.uEdge {
      ABE_TOP => TaskbarEdge::Top,
      ABE_BOTTOM => TaskbarEdge::Bottom,
      ABE_LEFT => TaskbarEdge::Left,
      ABE_RIGHT => TaskbarEdge::Right,
      _ => TaskbarEdge::Bottom,
    };
    let w = rc.right - rc.left;
    let h = rc.bottom - rc.top;
    Some(TaskbarInfo {
      rect: (rc.left, rc.top, rc.right, rc.bottom),
      edge,
      size: (w, h),
    })
  }
}

#[cfg(target_os = "windows")]
pub fn calc_anchor_on_taskbar(
  info: TaskbarInfo,
  win_size: (i32, i32),
  margin: i32,
  align: &str,
) -> (i32, i32) {
  let (tb_l, tb_t, tb_r, tb_b) = info.rect;
  let (tb_w, tb_h) = info.size;
  let (ww, wh) = win_size;

  match info.edge {
    TaskbarEdge::Bottom => {
      let y = tb_t - wh - margin;
      let x = match align {
        "start" => tb_l + margin,
        "end" => tb_r - ww - margin,
        _ => tb_l + (tb_w - ww) / 2,
      };
      (x, y)
    }
    TaskbarEdge::Top => {
      let y = tb_b + margin;
      let x = match align {
        "start" => tb_l + margin,
        "end" => tb_r - ww - margin,
        _ => tb_l + (tb_w - ww) / 2,
      };
      (x, y)
    }
    TaskbarEdge::Left => {
      let x = tb_r + margin;
      let y = tb_t + (tb_h - wh) / 2;
      (x, y)
    }
    TaskbarEdge::Right => {
      let x = tb_l - ww - margin;
      let y = tb_t + (tb_h - wh) / 2;
      (x, y)
    }
  }
}
