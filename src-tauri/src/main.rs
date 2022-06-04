#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{GlobalShortcutManager, Manager, RunEvent, WindowEvent};

mod shortcuts;

#[allow(clippy::collapsible_match)]
fn main() {
    let app = tauri::Builder::default()
        .on_window_event(|event| {
            if let WindowEvent::Focused(focused) = event.event() {
                if *focused {
                    shortcuts::register(event.window().app_handle());
                } else {
                    shortcuts::unregister(event.window().app_handle());
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("Error while building tauri application");

    app.run(|app_handle, event| {
        if let RunEvent::Ready = event {
            shortcuts::register(app_handle.clone());
            let window = app_handle.get_window("main").unwrap();
            window.set_focus().unwrap();
            window.show().unwrap();
        }
    })
}
