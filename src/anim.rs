use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct FrameAnimation {
  pub frames: Vec<Handle<Image>>,
  pub current: usize,
  pub timer: Timer,
}

pub fn setup_animation(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle::default());

  let mut frames = Vec::new();
  for i in 1..=620 {
    let path = format!("embedded://frames/frame_{:04}.png", i);
    frames.push(asset_server.load::<Image>(path));
  }
  assert!(
    !frames.is_empty(),
    "❌ Không tìm thấy frame trong assets/frames/"
  );

  commands.spawn((
    SpriteBundle {
      texture: frames[0].clone(),
      transform: Transform::from_scale(Vec3::splat(1.0)),
      ..default()
    },
    FrameAnimation {
      frames,
      current: 0,
      timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
    },
  ));
}

use crate::state::AnimControl;

pub fn animate_frames(
  time: Res<Time>,
  anim_ctl: Res<AnimControl>,
  mut q: Query<(&mut FrameAnimation, &mut Handle<Image>)>,
) {
  if anim_ctl.paused || anim_ctl.hidden {
    return;
  }
  for (mut anim, mut tex) in &mut q {
    anim.timer.tick(time.delta());
    if anim.timer.just_finished() {
      anim.current = (anim.current + 1) % anim.frames.len();
      *tex = anim.frames[anim.current].clone();
    }
  }
}
