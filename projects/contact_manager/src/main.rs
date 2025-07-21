mod models;
mod manager;
mod ui;

use manager::ContactManager;
use ui::{show_menu, get_input, get_number_input};

fn handle_add_contact(manager: &mut ContactManager) {
    let name = get_input("Name: ");
    if name.is_empty() {
        println!("Name cannot be empty.");
        return;
    }

    let phone = get_input("Phone: ");
    let email = get_input("Email: ");

    manager.add_contact(name, phone, email);

    println!("✅ Contact added successfully!");
   
}

fn handle_search_contacts(manager: &ContactManager) {
    let query = get_input("Search (name/phone/email): ");
    if query.is_empty() {
        println!("❌ Please enter a search term.");
        return;
    }

    let results = manager.search_contacts(&query);
    if results.is_empty() {
        println!("🔍 No matching contacts found.");
    } else {
        println!("\n🔍 Search Results ({} found):", results.len());
        for contact in results {
            println!("[{}] {} | {} | {}", contact.id, contact.name, contact.phone, contact.email);
        }
    }
}

fn handle_delete_contact(manager: &mut ContactManager) {
    manager.view_contacts();
    if manager.contacts.is_empty() {
        return;
    }

    let id = match get_number_input("Enter ID to delete (0 to cancel): ") {
        Some(0) => {
            println!("🚫 Deletion canceled.");
            return;
        }
        Some(id) => id,
        None => {
            println!("❌ Invalid ID.");
            return;
        }
    };

    if manager.delete_contact(id) {
        println!("🗑️ Contact deleted successfully.");
    } else {
        println!("❌ Contact not found.");
    }
}

 
fn main() {
    let mut manager = ContactManager::new();

    loop {
        show_menu();

        let choice = get_input("Select an option (1-5) ");

         match choice.as_str() {
            "1" => handle_add_contact(&mut manager),
            "2" => manager.view_contacts(),
            "3" => handle_search_contacts(&manager),
            "4" => handle_delete_contact(&mut manager),
            "5" => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid option. Please choose 1-5."),
        }
    }

   
}
 