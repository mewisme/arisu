use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct ArisuConfig {
  pub fps: u32,
  pub auto_startup: bool,
  pub frame_digits: u32,
  pub frame_width: f32,
  pub frame_height: f32,
  pub window_title: String,
  #[serde(default = "default_frame_folder")]
  pub frame_folder: String,
  #[serde(default)]
  pub gif_path: Option<String>,
  #[serde(default = "default_mode")]
  pub mode: String,
}

fn default_frame_folder() -> String {
  "frames".to_string()
}

fn default_mode() -> String {
  "auto".to_string()
}

impl Default for ArisuConfig {
  fn default() -> Self {
    Self {
      fps: 5,
      auto_startup: false,
      frame_digits: 4,
      frame_width: 128.0,
      frame_height: 128.0,
      window_title: "Arisu".to_string(),
      frame_folder: "frames".to_string(),
      gif_path: None,
      mode: "auto".to_string(),
    }
  }
}

fn get_config_path() -> Option<PathBuf> {
  if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_dir) = exe_path.parent() {
      return Some(exe_dir.join("arisu.config.json"));
    }
  }
  None
}

pub fn load_or_create_config() -> ArisuConfig {
  if let Some(config_path) = get_config_path() {
    if let Ok(content) = fs::read_to_string(&config_path) {
      if let Ok(config) = serde_json::from_str::<ArisuConfig>(&content) {
        println!("✅ Config loaded from: {:?}", config_path);

        #[cfg(windows)]
        {
          apply_startup_setting(&config);
        }

        return config;
      }
    }

    let config = ArisuConfig::default();
    if let Ok(json) = serde_json::to_string_pretty(&config) {
      if fs::write(&config_path, json).is_ok() {
        println!("📝 Config created at: {:?}", config_path);
      }
    }
    return config;
  }

  ArisuConfig::default()
}

#[cfg(windows)]
pub fn apply_startup_setting(config: &ArisuConfig) {
  let startup_enabled = is_startup_enabled();
  if config.auto_startup != startup_enabled {
    if config.auto_startup {
      if enable_startup() {
        println!("✅ Auto startup enabled");
      } else {
        println!("❌ Failed to enable auto startup");
      }
    } else {
      disable_startup();
      println!("🚫 Auto startup disabled");
    }
  }
}

#[cfg(windows)]
fn is_startup_enabled() -> bool {
  use windows::core::w;
  use windows::Win32::System::Registry::*;

  unsafe {
    let mut key = HKEY::default();
    if RegOpenKeyExW(
      HKEY_CURRENT_USER,
      w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
      0,
      KEY_READ,
      &mut key,
    )
    .is_ok()
    {
      let mut buffer = [0u16; 512];
      let mut size = (buffer.len() * 2) as u32;

      let result = RegQueryValueExW(
        key,
        w!("Arisu"),
        None,
        None,
        Some(buffer.as_mut_ptr() as *mut u8),
        Some(&mut size),
      );

      let _ = RegCloseKey(key);
      return result.is_ok();
    }
  }
  false
}

#[cfg(windows)]
fn enable_startup() -> bool {
  use windows::core::w;
  use windows::Win32::System::Registry::*;

  if let Ok(exe_path) = env::current_exe() {
    if let Some(exe_path_str) = exe_path.to_str() {
      let mut path_wide: Vec<u16> = exe_path_str.encode_utf16().collect();
      path_wide.push(0);

      let bytes =
        unsafe { std::slice::from_raw_parts(path_wide.as_ptr() as *const u8, path_wide.len() * 2) };

      unsafe {
        let mut key = HKEY::default();
        if RegOpenKeyExW(
          HKEY_CURRENT_USER,
          w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
          0,
          KEY_WRITE,
          &mut key,
        )
        .is_ok()
        {
          let result = RegSetValueExW(key, w!("Arisu"), 0, REG_SZ, Some(bytes));

          let _ = RegCloseKey(key);
          return result.is_ok();
        }
      }
    }
  }
  false
}

#[cfg(windows)]
fn disable_startup() {
  use windows::core::w;
  use windows::Win32::System::Registry::*;

  unsafe {
    let mut key = HKEY::default();
    if RegOpenKeyExW(
      HKEY_CURRENT_USER,
      w!("Software\\Microsoft\\Windows\\CurrentVersion\\Run"),
      0,
      KEY_WRITE,
      &mut key,
    )
    .is_ok()
    {
      let _ = RegDeleteValueW(key, w!("Arisu"));
      let _ = RegCloseKey(key);
    }
  }
}
