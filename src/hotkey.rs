use crossbeam_channel::Sender;
use global_hotkey::{
  hotkey::{Code, HotKey, Modifiers},
  GlobalHotKeyEvent, GlobalHotKeyManager,
};

use crate::state::TrayCmd;

pub fn spawn_hotkey(tx: Sender<TrayCmd>) {
  std::thread::spawn(move || {
    let manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");

    let pause_hotkey = HotKey::new(Some(Modifiers::ALT), Code::KeyP);
    let hide_hotkey = HotKey::new(Some(Modifiers::ALT), Code::KeyH);
    let lock_hotkey = HotKey::new(Some(Modifiers::ALT), Code::KeyL);
    let quit_hotkey = HotKey::new(Some(Modifiers::ALT), Code::KeyQ);

    manager
      .register(pause_hotkey)
      .expect("Failed to register Alt+P");
    manager
      .register(hide_hotkey)
      .expect("Failed to register Alt+H");
    manager
      .register(lock_hotkey)
      .expect("Failed to register Alt+L");
    manager
      .register(quit_hotkey)
      .expect("Failed to register Alt+Q");

    let receiver = GlobalHotKeyEvent::receiver();

    println!("   Hotkey active!");
    println!("   Alt+P: Toggle Pause/Resume");
    println!("   Alt+H: Toggle Hide/Visible");
    println!("   Alt+L: Toggle Lock/Unlock");
    println!("   Alt+Q: Quit");

    loop {
      if let Ok(event) = receiver.recv() {
        let cmd = if event.id == pause_hotkey.id() {
          println!("Pause/Resume toggled");
          Some(TrayCmd::TogglePause)
        } else if event.id == hide_hotkey.id() {
          println!("Hide/Visible toggled");
          Some(TrayCmd::ToggleHide)
        } else if event.id == lock_hotkey.id() {
          println!("Lock/Unlock toggled");
          Some(TrayCmd::ToggleLock)
        } else if event.id == quit_hotkey.id() {
          println!("Quit");
          Some(TrayCmd::Quit)
        } else {
          None
        };

        if let Some(c) = cmd {
          tx.send(c).ok();
        }
      }
    }
  });
}
