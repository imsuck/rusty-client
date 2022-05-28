#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{GlobalShortcutManager, Manager, RunEvent, WindowEvent};

mod shortcuts;

#[allow(clippy::collapsible_match)]
fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            window
                .eval("window.location.href = 'https://krunker.io/'")
                .unwrap();

            Ok(())
        })
        .on_window_event(|event| {
            if let WindowEvent::Focused(focused) = event.event() {
                if *focused {
                    shortcuts::register(event.window().app_handle());
                } else {
                    event
                        .window()
                        .app_handle()
                        .global_shortcut_manager()
                        .unregister_all()
                        .unwrap();
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("Error while running tauri application.");

    app.run(|app_handle, event| {
        if let RunEvent::Ready = event {
            shortcuts::register(app_handle.clone());
            app_handle.get_window("main").unwrap().set_focus().unwrap();
        }
    })
}
