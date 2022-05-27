#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod menu;

fn main() {
    tauri::Builder::default()
        .menu(menu::menu())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
        })
        .on_menu_event(|event| {
            let window = event.window();

            match event.menu_item_id() {
                "new game" | "new game 2" => {
                    println!("New game started.");
                    window
                        .eval("window.location.href = \"http://krunker.io/\"")
                        .unwrap();
                },
                "toggle fullscreen" => {
                    window.eval("window.__TAURI__.tauri.invoke(\"toggle_fullscreen\"")
                }
                _ => println!("Got event id {}.", event.menu_item_id()),
            }
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
}
