use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::io::{self, Write};
use std::path::Path;
use uuid::Uuid;
use std::process;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Prompt {
    id: String,
    title: String,
    tags: Vec<String>,
    content: String,
}

// Load prompts from JSON file
fn load_prompts() -> Vec<Prompt> {
    if !Path::new("prompts.json").exists() {
        return Vec::new();
    }
    
    fs::read_to_string("prompts.json")
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_else(|| {
            eprintln!("❌ Error reading prompts file. It might be corrupted.");
            Vec::new()
        })
}

// Save prompts to JSON file
fn save_prompts(prompts: &[Prompt]) -> io::Result<()> {
    let json = serde_json::to_string_pretty(prompts).unwrap();
    fs::write("prompts.json", json).map_err(|e| {
        eprintln!("❌ Failed to save prompts: {}", e);
        e
    })
}

// Read multi-line input for longer prompts
fn read_multiline_input(prompt_text: &str) -> String {
    println!("{}", prompt_text);
    println!("Enter your content. Type 'END' on a new line when finished:");
    
    let mut content = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        
        if line.trim() == "END" {
            break;
        }
        
        content.push_str(&line);
    }
    
    content.trim().to_string()
}

// Add a new prompt with support for longer content
fn add_prompt() {
    println!("\n📝 Adding a new prompt");
    println!("---------------------");
    
    let mut title = String::new();
    let mut tags = String::new();

    print!("Title: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();

    let content = read_multiline_input("Content:");

    print!("Tags (comma-separated): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut tags).unwrap();

    let prompt = Prompt {
        id: Uuid::new_v4().to_string(),
        title: title.trim().to_string(),
        content,
        tags: tags.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
    };

    let mut prompts = load_prompts();
    prompts.push(prompt);
    
    if let Err(e) = save_prompts(&prompts) {
        eprintln!("❌ Failed to save prompt: {}", e);
    } else {
        println!("✅ Prompt added successfully!");
    }
}

// Edit a prompt with support for longer content
fn edit_prompt_content(prompt: &mut Prompt) {
    println!("Current content:\n{}", prompt.content);
    println!("Enter new content (type 'END' on a new line when finished, or just press Enter to keep current):");
    
    let mut new_content = String::new();
    let mut line_count = 0;
    
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        
        if line.trim() == "END" {
            break;
        }
        
        // If first line is empty, keep original content
        if line_count == 0 && line.trim().is_empty() {
            return; // Keep original content
        }
        
        new_content.push_str(&line);
        line_count += 1;
    }
    
    if !new_content.trim().is_empty() {
        prompt.content = new_content.trim().to_string();
    }
}

// List all prompts
fn list_prompts() {
    let prompts = load_prompts();
    
    if prompts.is_empty() {
        println!("📭 No prompts found.");
        return;
    }
    
    println!("\n📋 All Prompts");
    println!("--------------");
    
    for (i, prompt) in prompts.iter().enumerate() {
        println!("{}. {} [{}]", i + 1, prompt.title, prompt.tags.join(", "));
    }
}

// View a specific prompt in detail
fn view_prompt() {
    let prompts = load_prompts();
    
    if prompts.is_empty() {
        println!("📭 No prompts found.");
        return;
    }
    
    list_prompts();
    
    print!("\nEnter the number of the prompt to view: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    let index: usize = match choice.trim().parse::<usize>() {
        Ok(n) if n > 0 && n <= prompts.len() => n - 1,
        _ => {
            println!("❌ Invalid selection.");
            return;
        }
    };
    
    let prompt = &prompts[index];
    println!("\n🔍 Prompt Details");
    println!("----------------");
    println!("Title: {}", prompt.title);
    println!("Tags: {}", prompt.tags.join(", "));
    println!("Content:\n{}\n", prompt.content);
    println!("Content length: {} characters", prompt.content.len());
}

// Edit an existing prompt
fn edit_prompt() {
    let mut prompts = load_prompts();
    
    if prompts.is_empty() {
        println!("📭 No prompts found to edit.");
        return;
    }
    
    list_prompts();
    
    print!("\nEnter the number of the prompt to edit: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    let index: usize = match choice.trim().parse::<usize>() {
        Ok(n) if n > 0 && n <= prompts.len() => n - 1,
        _ => {
            println!("❌ Invalid selection.");
            return;
        }
    };
    
    let prompt = &mut prompts[index];
    println!("\n✏️  Editing: {}", prompt.title);
    
    let mut new_title = String::new();
    let mut new_tags = String::new();
    
    print!("New title (press Enter to keep current: '{}'): ", prompt.title);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_title).unwrap();
    
    // Edit content with multi-line support
    edit_prompt_content(prompt);
    
    print!("New tags (comma-separated, press Enter to keep current: '{}'): ", 
           prompt.tags.join(", "));
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut new_tags).unwrap();
    
    if !new_title.trim().is_empty() {
        prompt.title = new_title.trim().to_string();
    }
    
    if !new_tags.trim().is_empty() {
        prompt.tags = new_tags.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }
    
    if let Err(e) = save_prompts(&prompts) {
        eprintln!("❌ Failed to save changes: {}", e);
    } else {
        println!("✅ Prompt updated successfully!");
    }
}

// Delete a prompt
fn delete_prompt() {
    let mut prompts = load_prompts();
    
    if prompts.is_empty() {
        println!("📭 No prompts found to delete.");
        return;
    }
    
    list_prompts();
    
    print!("\nEnter the number of the prompt to delete: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    let index: usize = match choice.trim().parse::<usize>() {
        Ok(n) if n > 0 && n <= prompts.len() => n - 1,
        _ => {
            println!("❌ Invalid selection.");
            return;
        }
    };
    
    let deleted = prompts.remove(index);
    println!("🗑️  Deleted prompt: {}", deleted.title);
    
    if let Err(e) = save_prompts(&prompts) {
        eprintln!("❌ Failed to save changes: {}", e);
    } else {
        println!("✅ Prompt deleted successfully!");
    }
}

// Search prompts by tag
fn search_by_tag() {
    let prompts = load_prompts();
    
    if prompts.is_empty() {
        println!("📭 No prompts found.");
        return;
    }
    
    print!("Enter tag to search for: ");
    io::stdout().flush().unwrap();
    
    let mut tag = String::new();
    io::stdin().read_line(&mut tag).unwrap();
    let tag = tag.trim();
    
    if tag.is_empty() {
        println!("❌ Please enter a tag to search for.");
        return;
    }
    
    let results: Vec<_> = prompts
        .into_iter()
        .filter(|p| p.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
        .collect();
 
    if results.is_empty() {
        println!("❌ No prompts found with tag '{}'", tag);
    } else {
        println!("\n🔍 Found {} prompts with tag '{}':", results.len(), tag);
        println!("-----------------------------------");
        for prompt in results {
            println!("🔹 {}: {}\nTags: {}\n", 
                    prompt.title, 
                    if prompt.content.len() > 100 {
                        format!("{}...", &prompt.content[..100])
                    } else {
                        prompt.content
                    },
                    prompt.tags.join(", "));
        }
    }
}

// Search prompts by keyword in title or content
fn search_by_keyword() {
    let prompts = load_prompts();
    
    if prompts.is_empty() {
        println!("📭 No prompts found.");
        return;
    }
    
    print!("Enter keyword to search for: ");
    io::stdout().flush().unwrap();
    
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword).unwrap();
    let keyword = keyword.trim().to_lowercase();
    
    if keyword.is_empty() {
        println!("❌ Please enter a keyword to search for.");
        return;
    }
    
    let results: Vec<_> = prompts
        .into_iter()
        .filter(|p| 
            p.title.to_lowercase().contains(&keyword) || 
            p.content.to_lowercase().contains(&keyword))
        .collect();
 
    if results.is_empty() {
        println!("❌ No prompts found containing '{}'", keyword);
    } else {
        println!("\n🔍 Found {} prompts containing '{}':", results.len(), keyword);
        println!("-----------------------------------------");
        for prompt in results {
            println!("🔹 {}: {}\nTags: {}\n", 
                    prompt.title, 
                    if prompt.content.len() > 100 {
                        format!("{}...", &prompt.content[..100])
                    } else {
                        prompt.content
                    },
                    prompt.tags.join(", "));
        }
    }
}

// List all unique tags
fn list_all_tags() {
    let prompts = load_prompts();
    
    if prompts.is_empty() {
        println!("📭 No prompts found.");
        return;
    }
    
    let mut all_tags: HashSet<String> = HashSet::new();
    for prompt in prompts {
        for tag in prompt.tags {
            all_tags.insert(tag);
        }
    }
    
    if all_tags.is_empty() {
        println!("🏷️  No tags found.");
        return;
    }
    
    let mut sorted_tags: Vec<String> = all_tags.into_iter().collect();
    sorted_tags.sort();
    
    println!("\n🏷️  All Tags");
    println!("-----------");
    for tag in sorted_tags {
        println!("🔖 {}", tag);
    }
}

// Export prompts to different formats
fn export_prompts() {
    let prompts = load_prompts();
    
    if prompts.is_empty() {
        println!("📭 No prompts found to export.");
        return;
    }
    
    println!("\n📤 Export Prompts");
    println!("----------------");
    println!("1) Export as JSON (default format)");
    println!("2) Export as CSV");
    println!("3) Export as Text file");
    println!("4) Export as Markdown");
    print!("Choose format: ");
    io::stdout().flush().unwrap();
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    
    print!("Enter filename (without extension): ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();
    
    if filename.is_empty() {
        println!("❌ Filename cannot be empty.");
        return;
    }
    
    match choice.trim() {
        "1" | "" => export_json(&prompts, filename),
        "2" => export_csv(&prompts, filename),
        "3" => export_text(&prompts, filename),
        "4" => export_markdown(&prompts, filename),
        _ => {
            println!("❌ Invalid option, exporting as JSON.");
            export_json(&prompts, filename);
        }
    }
}

// Export to JSON format
fn export_json(prompts: &[Prompt], filename: &str) {
    let filepath = format!("{}.json", filename);
    let json = serde_json::to_string_pretty(prompts).unwrap();
    
    match fs::write(&filepath, json) {
        Ok(_) => println!("✅ Prompts exported to {}", filepath),
        Err(e) => eprintln!("❌ Failed to export JSON: {}", e),
    }
}

// Export to CSV format
fn export_csv(prompts: &[Prompt], filename: &str) {
    let filepath = format!("{}.csv", filename);
    let mut wtr = csv::Writer::from_path(&filepath).unwrap();
    
    for prompt in prompts {
        let tags = prompt.tags.join(";");
        if let Err(e) = wtr.serialize((
            &prompt.id,
            &prompt.title,
            &tags,
            &prompt.content
        )) {
            eprintln!("❌ Failed to write CSV record: {}", e);
            return;
        }
    }
    
    wtr.flush().unwrap();
    println!("✅ Prompts exported to {}", filepath);
}

// Export to Text format
fn export_text(prompts: &[Prompt], filename: &str) {
    let filepath = format!("{}.txt", filename);
    let mut content = String::new();
    
    for prompt in prompts {
        content.push_str(&format!("Title: {}\n", prompt.title));
        content.push_str(&format!("ID: {}\n", prompt.id));
        content.push_str(&format!("Tags: {}\n", prompt.tags.join(", ")));
        content.push_str(&format!("Content:\n{}\n", prompt.content));
        content.push_str(&"-".repeat(50));
        content.push_str("\n\n");
    }
    
    match fs::write(&filepath, content) {
        Ok(_) => println!("✅ Prompts exported to {}", filepath),
        Err(e) => eprintln!("❌ Failed to export text: {}", e),
    }
}

// Export to Markdown format
fn export_markdown(prompts: &[Prompt], filename: &str) {
    let filepath = format!("{}.md", filename);
    let mut content = String::from("# Prompt Library\n\n");
    
    for prompt in prompts {
        content.push_str(&format!("## {}\n\n", prompt.title));
        content.push_str(&format!("**ID:** {}\n\n", prompt.id));
        content.push_str(&format!("**Tags:** {}\n\n", prompt.tags.join(", ")));
        content.push_str(&format!("**Content:**\n\n```\n{}\n```\n\n", prompt.content));
        content.push_str(&"---\n\n");
    }
    
    match fs::write(&filepath, content) {
        Ok(_) => println!("✅ Prompts exported to {}", filepath),
        Err(e) => eprintln!("❌ Failed to export Markdown: {}", e),
    }
}

// Import prompts from a file
fn import_prompts() {
    println!("\n📥 Import Prompts");
    println!("----------------");
    println!("Note: This will merge imported prompts with existing ones.");
    print!("Enter filename to import (with extension): ");
    io::stdout().flush().unwrap();
    
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();
    
    if filename.is_empty() {
        println!("❌ Filename cannot be empty.");
        return;
    }
    
    if !Path::new(filename).exists() {
        println!("❌ File '{}' does not exist.", filename);
        return;
    }
    
    let extension = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    match extension.to_lowercase().as_str() {
        "json" => import_json(filename),
        "csv" => import_csv(filename),
        _ => println!("❌ Unsupported file format. Only JSON and CSV are supported."),
    }
}

// Import from JSON format
fn import_json(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(data) => {
            let imported_prompts: Vec<Prompt> = match serde_json::from_str(&data) {
                Ok(prompts) => prompts,
                Err(e) => {
                    eprintln!("❌ Failed to parse JSON: {}", e);
                    return;
                }
            };
            
            let mut existing_prompts = load_prompts();
            let original_count = existing_prompts.len();
            
            for mut new_prompt in imported_prompts {
                // Generate new ID to avoid conflicts
                new_prompt.id = Uuid::new_v4().to_string();
                existing_prompts.push(new_prompt);
            }
            
            if let Err(e) = save_prompts(&existing_prompts) {
                eprintln!("❌ Failed to save imported prompts: {}", e);
            } else {
                println!("✅ Imported {} prompts. Total prompts now: {}", 
                        existing_prompts.len() - original_count,
                        existing_prompts.len());
            }
        },
        Err(e) => eprintln!("❌ Failed to read file: {}", e),
    }
}

// Import from CSV format
fn import_csv(filename: &str) {
    let mut rdr = match csv::Reader::from_path(filename) {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("❌ Failed to read CSV file: {}", e);
            return;
        }
    };
    
    let mut existing_prompts = load_prompts();
    let original_count = existing_prompts.len();
    let mut imported_count = 0;
    
    for result in rdr.deserialize() {
        let record: (String, String, String, String) = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("❌ Failed to parse CSV record: {}", e);
                continue;
            }
        };
        
        let tags: Vec<String> = record.2.split(';')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
            
        let prompt = Prompt {
            id: Uuid::new_v4().to_string(),
            title: record.1,
            tags,
            content: record.3,
        };
        
        existing_prompts.push(prompt);
        imported_count += 1;
    }
    
    if let Err(e) = save_prompts(&existing_prompts) {
        eprintln!("❌ Failed to save imported prompts: {}", e);
    } else {
        println!("✅ Imported {} prompts. Total prompts now: {}", 
                imported_count,
                existing_prompts.len());
    }
}

// Display main menu
fn show_menu() {
    println!("\n📂 Prompt Pack Manager");
    println!("======================");
    println!("1) Add a new prompt");
    println!("2) List all prompts");
    println!("3) View a prompt in detail");
    println!("4) Edit a prompt");
    println!("5) Delete a prompt");
    println!("6) Search by tag");
    println!("7) Search by keyword");
    println!("8) List all tags");
    println!("9) Export prompts");
    println!("10) Import prompts");
    println!("11) Exit");
    print!("\nChoose an option: ");
    io::stdout().flush().unwrap();
}

fn main() {
    // Add csv to dependencies
    println!("🚀 Welcome to Prompt Pack Manager!");
    println!("Manage your AI prompts efficiently.\n");
    
    // Add the csv crate to Cargo.toml if not already present
    println!("💡 Make sure to add this to your Cargo.toml:");
    println!("csv = \"1.1\"");
    println!("serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("serde_json = \"1.0\"");
    println!("uuid = {{ version = \"1.3\", features = [\"v4\"] }}");
    println!();
    
    loop {
        show_menu();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => add_prompt(),
            "2" => list_prompts(),
            "3" => view_prompt(),
            "4" => edit_prompt(),
            "5" => delete_prompt(),
            "6" => search_by_tag(),
            "7" => search_by_keyword(),
            "8" => list_all_tags(),
            "9" => export_prompts(),
            "10" => import_prompts(),
            "11" => {
                println!("👋 Goodbye!");
                process::exit(0);
            }
            _ => println!("❌ Invalid option. Please try again."),
        }
        
        println!("\nPress Enter to continue...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
}