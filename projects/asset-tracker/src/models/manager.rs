use super::asset::AssetItem;
use std::collections::HashMap;

pub struct AssetManager {
    pub assets_vec: Vec<AssetItem>,        
    pub assets_map: HashMap<u32, usize>,
    pub next_id: u32,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets_vec: Vec::new(),
            assets_map: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_asset(&mut self, name: String, value: f64) -> u32 {
        let serial_number = self.next_id;
        let asset = AssetItem {
            name,
            serial_number,
            value,
        };

        self.next_id += 1;
        self.assets_vec.push(asset);
        self.assets_map.insert(serial_number, self.assets_vec.len() - 1);
        serial_number
    }

    pub fn view_assets(&self) {
        if self.assets_vec.is_empty() {
            println!("\nNo assets found.");
            return;
        }

        println!("\nCompany Assets (Total: {}):", self.assets_vec.len());
        println!("┌{0:─<5}┬{0:─<20}┬{0:─<15}┬{0:─<12}┐", "─");
        println!("│ {:<3} │ {:<18} │ {:<13} │ {:<10} │", 
                "ID", "Name", "Serial No.", "Value");
        println!("├{0:─<5}┼{0:─<20}┼{0:─<15}┼{0:─<12}┤", "─");

        for (i, asset) in self.assets_vec.iter().enumerate() {
            println!("│ {:<3} │ {:<18} │ SN-{:<10} │ ${:<9.2} │", 
                i + 1, 
                Self::truncate(&asset.name, 18),
                asset.serial_number, 
                asset.value);
        }

        println!("└{0:─<5}┴{0:─<20}┴{0:─<15}┴{0:─<12}┘", "─");
        println!("* Values shown in USD");
    }

    // Helper function to truncate long names
    pub fn truncate(s: &str, max_len: usize) -> String {
        if s.len() > max_len {
            format!("{}...", &s[..max_len.saturating_sub(3)])
        } else {
            s.to_string()
        }
    }

    pub fn remove_asset(&mut self, serial_number: u32) -> Option<AssetItem> {
        let index = self.assets_map.remove(&serial_number)?;
        let removed_asset = self.assets_vec.swap_remove(index);

        if index < self.assets_vec.len() {
            let swapped_serial = self.assets_vec[index].serial_number;
            self.assets_map.insert(swapped_serial, index);
        }
    
        Some(removed_asset)
    }

    pub fn edit_asset(
        &mut self,
        serial_number: u32,
        new_name: Option<String>,
        new_value: Option<f64>,
    ) -> Result<(), &'static str> {
        let asset = self.assets_vec.iter_mut()
            .find(|a| a.serial_number == serial_number)
            .ok_or("Asset not found")?;

        if let Some(name) = new_name {
            asset.name = name;
        }
        if let Some(value) = new_value {
            asset.value = value;
        }

        Ok(())
    }

    pub fn get_asset(&self, serial_number: u32) -> Option<&AssetItem> {
        self.assets_vec.iter()
            .find(|a| a.serial_number == serial_number)
    }
}

