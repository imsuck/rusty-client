use tauri::{CustomMenuItem, Menu, Submenu};

pub(crate) fn menu() -> Menu {
    let new_game = CustomMenuItem::new("new game", "New Game").accelerator("F6");
    let new_game_2 = CustomMenuItem::new("new game 2", "New Game 2").accelerator("CmdOrControl+N");
    let toggle_fullscreen =
        CustomMenuItem::new("toggle fullscreen", "Toggle Fullscreen").accelerator("F11");
    let open_dev_tools = CustomMenuItem::new("toggle fullscreen", "Toggle Fullscreen")
        .accelerator("CmdOrControl+Shift+I");

    Menu::new().add_submenu(Submenu::new(
        "Rusty client",
        Menu::new()
            .add_item(new_game)
            .add_item(new_game_2)
            .add_item(toggle_fullscreen)
            .add_item(open_dev_tools),
    ))
}
