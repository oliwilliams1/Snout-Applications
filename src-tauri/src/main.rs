// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, WebviewWindow};
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::thread;
use std::time::Duration;

fn set_window_position(window: &WebviewWindow) {
    let size = window.inner_size().unwrap();
    let half_width: i32 = (size.width / 2).try_into().unwrap();

    let device_state = DeviceState::new();
    let mouse_position = device_state.get_mouse().coords;
    window.set_position(tauri::Position::Physical(tauri::PhysicalPosition::new(
        mouse_position.0 - half_width,
        mouse_position.1 - 25,
    ))).unwrap();
}

fn close_window(window: &WebviewWindow) {
    window.close().unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Snout Apps")?;
            main_window.eval("window.location.href = 'https://oliwilliams1.github.io/Snout-Apps-Website/';")?;

            let main_window_clone = main_window.clone();
            thread::spawn(move || {
                let device_state = DeviceState::new();
                loop {
                    let keys = device_state.get_keys();
                    if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::LShift) {
                        if keys.contains(&Keycode::D) {
                            set_window_position(&main_window_clone);
                        } else if keys.contains(&Keycode::A) || keys.contains(&Keycode::W) {
                            close_window(&main_window_clone);
                        }
                    }
                    thread::sleep(Duration::from_millis(16)); // Smooth enough for 60 fps, Slow enough for minimal cpu usage
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}