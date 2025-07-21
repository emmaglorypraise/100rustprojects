mod models;
mod ui;

use models::{AssetItem, AssetManager};
use ui::{show_menu, handle_add_asset, handle_remove_asset, handle_edit_asset};

fn main() {
    let mut manager = AssetManager::new();

    loop {
        match show_menu() {
            1 => handle_add_asset(&mut manager),
            2 => manager.view_assets(),
            3 => handle_remove_asset(&mut manager),
            4 => handle_edit_asset(&mut manager),
            5 => {
                println!("Exiting program...");
                break;
            }
            _ => unreachable!(),
        }
    }
}