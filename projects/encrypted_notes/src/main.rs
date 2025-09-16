use aes::Aes256;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use cbc::{Decryptor, Encryptor};
use hex;
use std::fs::{self, OpenOptions};
use std::io::{self, Write, BufRead};

// Define our encryption/decryption types
type Aes256Enc = Encryptor<Aes256>;
type Aes256Dec = Decryptor<Aes256>;

const KEY: &[u8; 32] = b"an_example_very_secure_key_32byt"; // 32 bytes
const IV: &[u8; 16] = b"unique_iv_16byte";

fn encrypt(plain_text: &str) -> String {
    let plain_bytes = plain_text.as_bytes();
    
    // Create a buffer with enough capacity for padding
    let block_size = 16;
    let padded_len = plain_bytes.len() + (block_size - (plain_bytes.len() % block_size));
    let mut buffer = vec![0u8; padded_len];
    buffer[..plain_bytes.len()].copy_from_slice(plain_bytes);
    
    // Encrypt using CBC mode
    let cipher = Aes256Enc::new(KEY.into(), IV.into());
    let ciphertext = cipher.encrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(
        &mut buffer,
        plain_bytes.len()
    ).expect("Encryption failed");
    
    hex::encode(ciphertext)
}

fn decrypt(cipher_hex: &str) -> String {
    let mut bytes = hex::decode(cipher_hex).expect("Invalid hex");
    
    // Decrypt using CBC mode
    let cipher = Aes256Dec::new(KEY.into(), IV.into());
    let decrypted = cipher.decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(
        &mut bytes
    ).expect("Decryption failed");
    
    String::from_utf8_lossy(decrypted).to_string()
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

    match fs::File::open("notes.db") {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            for (i, line) in reader.lines().enumerate() {
                let encrypted = line.unwrap();
                let decrypted = decrypt(&encrypted);
                println!("{}. {}", i + 1, decrypted);
            }
        }
        Err(_) => {
            println!("❌ No notes found.");
        }
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