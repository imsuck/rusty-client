#[allow(dead_code)]

#[tauri::command]
pub(crate) async fn toggle_fullscreen(window: tauri::Window) {
    if let Ok(is_fullscreen) = window.is_fullscreen() {
        window.set_fullscreen(!is_fullscreen).ok();
    }
}

#[tauri::command]
pub(crate) async fn menu_toggle(window: tauri::Window) {
    window.menu_handle().toggle().unwrap();
}
