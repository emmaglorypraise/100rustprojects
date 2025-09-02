use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead};
use std::str;
 
type Aes256Cbc = Cbc<Aes256, Pkcs7>;
 
const KEY: &[u8; 32] = b"an_example_very_secure_key_32byt!";
const IV: &[u8; 16] = b"unique_iv_16byte";
 
fn encrypt(plain_text: &str) -> String {
    let cipher = Aes256Cbc::new_from_slices(KEY, IV).unwrap();
    let ciphertext = cipher.encrypt_vec(plain_text.as_bytes());
    hex::encode(ciphertext)
}
 
fn decrypt(cipher_hex: &str) -> String {
    let cipher = Aes256Cbc::new_from_slices(KEY, IV).unwrap();
    let bytes = hex::decode(cipher_hex).expect("Invalid hex");
    let decrypted = cipher.decrypt_vec(&bytes).expect("Decryption failed");
    String::from_utf8_lossy(&decrypted).to_string()
}
 
fn write_note() {
    print!("📝 Enter your note: ");
    io::stdout().flush().unwrap();
 
    let mut note = String::new();
    io::stdin().read_line(&mut note).unwrap();
    let note = note.trim();
 
    let encrypted = encrypt(note);
 
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.db")
        .expect("❌ Cannot open file");
 
    writeln!(file, "{}", encrypted).unwrap();
    println!("🔐 Note saved securely.");
}
 
fn read_notes() {
    println!("\n🔓 Decrypting all notes:\n");
 
    let file = fs::File::open("notes.db").expect("❌ No notes found.");
    let reader = io::BufReader::new(file);
 
    for (i, line) in reader.lines().enumerate() {
        let encrypted = line.unwrap();
        let decrypted = decrypt(&encrypted);
        println!("{}. {}", i + 1, decrypted);
    }
}
 
fn main() {
    println!("🔏 Encrypted Notes Manager");
    println!("1) Write a note");
    println!("2) Read all notes");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();
 
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
 
    match choice.trim() {
        "1" => write_note(),
        "2" => read_notes(),
        _ => println!("❌ Invalid option"),
    }
}