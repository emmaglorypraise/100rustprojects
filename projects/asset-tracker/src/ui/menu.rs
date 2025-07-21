use crate::models::{AssetItem, AssetManager};
use crate::ui::input::{get_input, get_number_input};

pub fn show_menu() -> u32 {
    println!("\nWelcome to XYZ Company's Asset Manager!");
    println!("1. Add new asset");
    println!("2. View all assets");
    println!("3. Delete asset");
    println!("4. Edit asset details");
    println!("5. Exit");
    
    match get_number_input("Choose an option (1-5): ") {
        Some(num @ 1..=5) => num,
        _ => {
            println!("Invalid choice. Please enter a number between 1-5.");
            show_menu()
        }
    }
}


pub fn confirm_edit(current_asset: &AssetItem, new_name: &Option<String>, new_value: &Option<f64>) -> bool {
    println!("\nProposed Changes:");
    println!("Current Name: {}", current_asset.name);
    println!("New Name: {}", new_name.as_ref().unwrap_or(&current_asset.name));
    println!("Current Value: ${:.2}", current_asset.value);
    println!("New Value: ${:.2}", new_value.unwrap_or(current_asset.value));

    let confirmation = get_input("\nSave changes? (y/n): ");
    confirmation.eq_ignore_ascii_case("y") || confirmation.eq_ignore_ascii_case("yes")
}

pub fn handle_add_asset(manager: &mut AssetManager) {
    let name = get_input("Enter asset name: ");
    let value: f64 = match get_number_input("Enter asset value: ") {
        Some(v) => v,
        None => {
            println!("Invalid value. Operation canceled.");
            return;
        }
    };

    let sn = manager.add_asset(name, value);
    println!("Asset added successfully with serial number: {}", sn);
}

pub fn handle_remove_asset(manager: &mut AssetManager) {
    manager.view_assets();
    if manager.assets_vec.is_empty() {
        return;
    }

    let sn: u32 = match get_number_input("Enter serial number to delete: ") {
        Some(s) => s,
        None => {
            println!("Invalid serial number.");
            return;
        }
    };

    match manager.remove_asset(sn) {
        Some(asset) => println!("Removed asset: {} (SN: {})", asset.name, asset.serial_number),
        None => println!("Asset not found."),
    }
}

pub fn handle_edit_asset(manager: &mut AssetManager) {
    manager.view_assets();
    if manager.assets_vec.is_empty() {
        println!("No assets to edit.");
        return;
    }
    
    let sn: u32 = match get_number_input("Enter serial number to edit: ") {
        Some(s) => s,
        None => {
            println!("Invalid serial number.");
            return;
        }
    };

    let current = match manager.get_asset(sn) {
        Some(asset) => asset,
        None => {
            println!("Asset not found.");
            return;
        }
    };

    println!("Editing: {} (Current Value: ${:.2})", current.name, current.value);
    
    let new_name = {
        let input = get_input(&format!("Enter new name (current: {}): ", current.name));
        if input.is_empty() { None } else { Some(input) }
    };

    let new_value: Option<f64> = get_number_input(&format!(
        "Enter new value (current: {:.2}): ", current.value
    ));

    if !confirm_edit(current, &new_name, &new_value) {
        println!("Edit canceled.");
        return;
    }

    match manager.edit_asset(sn, new_name, new_value) {
        Ok(()) => println!("Asset updated successfully."),
        Err(e) => println!("Error: {}", e),
    }
}
