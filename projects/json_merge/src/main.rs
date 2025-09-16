use serde_json::{Value};
use std::env;
use std::fs;
 
fn merge_json(base: &mut Value, patch: &Value) {
    match (base, patch) {
        (Value::Object(base_map), Value::Object(patch_map)) => {
            for (key, patch_value) in patch_map {
                match base_map.get_mut(key) {
                    Some(base_value) => merge_json(base_value, patch_value),
                    None => {
                        base_map.insert(key.clone(), patch_value.clone());
                    }
                }
            }
        }
        (base_value, patch_value) => {
            *base_value = patch_value.clone(); // overwrite
        }
    }
}
 
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <base.json> <patch.json> <output.json>", args[0]);
        return;
    }
 
    let base_path = &args[1];
    let patch_path = &args[2];
    let output_path = &args[3];
 
    let base_data = fs::read_to_string(base_path).expect("Failed to read base.json");
    let patch_data = fs::read_to_string(patch_path).expect("Failed to read patch.json");
 
    let mut base_json: Value = serde_json::from_str(&base_data).expect("Invalid base.json");
    let patch_json: Value = serde_json::from_str(&patch_data).expect("Invalid patch.json");
 
    merge_json(&mut base_json, &patch_json);
 
    fs::write(output_path, serde_json::to_string_pretty(&base_json).unwrap())
        .expect("Failed to write merged output");
 
    println!("✅ Merged JSON saved to {}", output_path);
}