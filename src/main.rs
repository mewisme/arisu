#![allow(dead_code)]
#![windows_subsystem = "windows"]

use bevy::prelude::*;
use bevy::window::{WindowLevel, WindowPosition};
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod anim;
mod drag;
mod state;
mod system;
mod taskbar;

use anim::{animate_frames, setup_animation};
use drag::drag_move_window;
use state::{keep_always_on_top, AnimControl};
use system::snap_to_taskbar_on_startup;

fn main() {
  App::new()
    .add_plugins(EmbeddedAssetPlugin::default())
    .insert_resource(ClearColor(Color::NONE))
    .insert_resource(AnimControl::default())
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "Arisu".into(),
        transparent: true,
        decorations: false,
        resizable: false,
        window_level: WindowLevel::AlwaysOnTop,
        position: WindowPosition::At(IVec2::new(800, 920)),
        resolution: (128.0, 128.0).into(),
        ..default()
      }),
      ..default()
    }))
    .add_systems(Startup, (setup_animation, snap_to_taskbar_on_startup))
    .add_systems(
      Update,
      (animate_frames, drag_move_window, keep_always_on_top),
    )
    .run();
}
