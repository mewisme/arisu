use crate::config::load_or_create_config;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Component)]
pub struct FrameAnimation {
  pub frames: Vec<Handle<Image>>,
  pub current: usize,
  pub timer: Timer,
}

impl FrameAnimation {
  pub fn set_fps(&mut self, fps: u32) {
    let duration = std::time::Duration::from_millis(1000 / fps as u64);
    self.timer = Timer::new(duration, TimerMode::Repeating);
  }
}

struct FrameInfo {
  path: PathBuf,
  count: usize,
  fps: u32,
  digits: u32,
  width: f32,
  height: f32,
}

fn check_external_frames() -> Option<FrameInfo> {
  let config = load_or_create_config();
  
  if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_dir) = exe_path.parent() {
      let assets_path = exe_dir.join("assets").join(&config.frame_folder);

      if assets_path.exists() && assets_path.is_dir() {
        if let Ok(entries) = fs::read_dir(&assets_path) {
          let png_count = entries
            .filter_map(|e| e.ok())
            .filter(|e| {
              e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("png"))
                .unwrap_or(false)
            })
            .count();

          if png_count > 0 {
            return Some(FrameInfo {
              path: assets_path,
              count: png_count,
              fps: config.fps,
              digits: config.frame_digits,
              width: config.frame_width,
              height: config.frame_height,
            });
          }
        }
      }
    }
  }
  None
}

fn check_gif_file() -> Option<PathBuf> {
  let config = load_or_create_config();
  
  if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_dir) = exe_path.parent() {
      let assets_dir = exe_dir.join("assets");
      
      // Use gif_path from config, or default to "anim.gif"
      let gif_file_name = config.gif_path.as_deref().unwrap_or("anim.gif");
      let gif_path = assets_dir.join(gif_file_name);
      
      if gif_path.exists() && gif_path.is_file() {
        return Some(gif_path);
      }
    }
  }
  None
}

fn decode_gif_to_images(gif_path: &PathBuf) -> Option<Vec<Image>> {
  use image::AnimationDecoder;
  use std::io::BufReader;
  
  if let Ok(file) = fs::File::open(gif_path) {
    let reader = BufReader::new(file);
    if let Ok(decoder) = image::codecs::gif::GifDecoder::new(reader) {
      if let Ok(gif_frames) = decoder.into_frames().collect_frames() {
        let mut images = Vec::new();
        
        for frame in gif_frames {
          let rgba_frame = frame.into_buffer();
          let (width, height) = rgba_frame.dimensions();
          let raw_data = rgba_frame.into_raw();
          
          let image = Image::new(
            Extent3d {
              width,
              height,
              depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            raw_data,
            TextureFormat::Rgba8UnormSrgb,
            bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
          );
          
          images.push(image);
        }
        
        if !images.is_empty() {
          return Some(images);
        }
      }
    }
  }
  None
}

pub fn setup_animation(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut images: ResMut<Assets<Image>>,
) {
  commands.spawn(Camera2dBundle::default());

  let mut frames = Vec::new();
  let config = load_or_create_config();
  let mode = config.mode.to_lowercase();
  
  // Initialize with default values from config
  let mut fps = config.fps;
  let mut frame_count = 620;

  // Check mode: "frame" -> only frames, "gif" -> only gif, "auto" -> auto-detect
  let should_load_frames = mode == "frame" || mode == "auto";
  let should_load_gif = mode == "gif" || mode == "auto";

  // Priority based on mode: PNG frames > GIF > embedded frames
  let mut loaded = false;

  if should_load_frames {
    if let Some(frame_info) = check_external_frames() {
      info!(
        "✨ Loading {} custom frames from: {:?}",
        frame_info.count, frame_info.path
      );
      info!("⚙️ FPS from config: {}", frame_info.fps);
      info!("🔢 Frame digits: {}", frame_info.digits);
      info!("📐 Frame size: {}x{}", frame_info.width, frame_info.height);

      fps = frame_info.fps;
      frame_count = frame_info.count;

      for i in 1..=frame_count {
        let path = format!(
          "{}/frame_{:0width$}.png",
          config.frame_folder,
          i,
          width = frame_info.digits as usize
        );
        frames.push(asset_server.load::<Image>(path));
      }
      loaded = true;
    } else if mode == "frame" {
      panic!("❌ Frame mode specified but no frames found in assets/{}/!", config.frame_folder);
    }
  }

  if !loaded && should_load_gif {
    if let Some(gif_path) = check_gif_file() {
      info!("🎬 Loading GIF animation from: {:?}", gif_path);
      
      if let Some(gif_images) = decode_gif_to_images(&gif_path) {
        fps = config.fps;
        frame_count = gif_images.len();
        
        info!("✅ Decoded {} frames from GIF", frame_count);
        info!("⚙️ FPS from config: {}", fps);
        
        // Create Image assets from decoded frames
        for image in gif_images.into_iter() {
          let handle = images.add(image);
          frames.push(handle);
        }
        
        info!("📐 Frame size: {}x{}", config.frame_width, config.frame_height);
        loaded = true;
      } else {
        if mode == "gif" {
          panic!("❌ GIF mode specified but failed to decode GIF file!");
        }
        warn!("❌ Failed to decode GIF, falling back to embedded frames");
      }
    } else if mode == "gif" {
      panic!("❌ GIF mode specified but no GIF file found!");
    }
  }

  if !loaded {
    // Fall back to embedded frames
    info!("📦 Loading embedded frames");

    fps = config.fps;
    frame_count = 620;

    info!("⚙️ FPS from config: {}", fps);
    info!(
      "📐 Frame size: {}x{}",
      config.frame_width, config.frame_height
    );

    for i in 1..=frame_count {
      let path = format!("embedded://frames/frame_{:04}.png", i);
      frames.push(asset_server.load::<Image>(path));
    }
  }

  assert!(!frames.is_empty(), "❌ No frames found!");

  let frame_duration = Duration::from_millis(1000 / fps as u64);
  info!(
    "🎬 Animation: {} frames @ {} FPS ({:?} per frame)",
    frame_count, fps, frame_duration
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
      timer: Timer::new(frame_duration, TimerMode::Repeating),
    },
  ));
}

use crate::window::AnimControl;

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
