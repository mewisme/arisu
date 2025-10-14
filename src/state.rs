use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowLevel, WindowPosition};
use crossbeam_channel::Receiver;

#[derive(Resource, Default)]
pub struct AnimControl {
  pub paused: bool,
  pub hidden: bool,
  pub locked: bool,
  pub dragging: bool,
  pub last_cursor: Option<Vec2>,
  pub window_pos: IVec2,
  pub follow_taskbar: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum TrayCmd {
  Quit,
  TogglePause,
  ToggleHide,
  ToggleLock,
}

#[derive(Resource)]
pub struct TrayRx(pub Receiver<TrayCmd>);

pub fn pump_tray_commands(
  tray_rx: Res<TrayRx>,
  mut anim_ctl: ResMut<AnimControl>,
  mut q_win: Query<&mut Window, With<PrimaryWindow>>,
  mut exit: EventWriter<AppExit>,
) {
  while let Ok(cmd) = tray_rx.0.try_recv() {
    match cmd {
      TrayCmd::Quit => {
        exit.send(AppExit::Success);
      }
      TrayCmd::TogglePause => {
        anim_ctl.paused = !anim_ctl.paused;
        if !anim_ctl.paused {
          anim_ctl.hidden = false;
          if let Ok(mut w) = q_win.get_single_mut() {
            w.visible = true;
          }
        }
      }
      TrayCmd::ToggleHide => {
        anim_ctl.hidden = !anim_ctl.hidden;
        if let Ok(mut w) = q_win.get_single_mut() {
          w.visible = !anim_ctl.hidden;
        }
        if anim_ctl.hidden {
          anim_ctl.paused = true;
        }
      }
      TrayCmd::ToggleLock => {
        anim_ctl.locked = !anim_ctl.locked;
        if anim_ctl.locked {
          anim_ctl.dragging = false;
          anim_ctl.last_cursor = None;
          anim_ctl.follow_taskbar = true;
        } else {
          anim_ctl.follow_taskbar = false;
        }
      }
    }
  }
}

pub fn keep_always_on_top(mut q_win: Query<&mut Window, With<PrimaryWindow>>) {
  if let Ok(mut w) = q_win.get_single_mut() {
    w.window_level = WindowLevel::AlwaysOnTop;
    if let WindowPosition::At(pos) = w.position {
      w.position = WindowPosition::At(pos);
    }
  }
}
