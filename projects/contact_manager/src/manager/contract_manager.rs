use crate::models::Contact;

pub struct ContactManager {
    pub contacts: Vec<Contact>,
    pub next_id: usize,
}

impl ContactManager {
    pub fn new() -> Self {
        Self {
            contacts: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_contact(&mut self, name: String, phone: String, email: String) {
        let id = self.next_id;

        let new_contact = Contact {
            id,
            name,
            phone,
            email,
        };

        self.contacts.push(new_contact);
        self.next_id += 1;

    }

    pub fn view_contacts(&self) {
        if self.contacts.is_empty() {
            println!("📭 No contacts.");
            return;
        } else {
            for c in &self.contacts {
                println!("[{}] {} | {} | {}", c.id, c.name, c.phone, c.email);
            }
        }

    }

    pub fn search_contacts(&self, query: &str) -> Vec<&Contact> {
       let query = query.trim().to_lowercase();

       self.contacts.iter()
             .filter(|c| c.name.to_lowercase().contains(&query.to_lowercase()) || c.email.to_lowercase().contains(&query.to_lowercase()) || c.phone.contains(&query))
             .collect()
    }

    pub fn delete_contact(&mut self, id: usize) -> bool {
        let original_len = self.contacts.len();
        self.contacts.retain(|c| c.id != id);
        original_len != self.contacts.len()
    }
}
