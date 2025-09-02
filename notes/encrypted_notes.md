# Project 083 – Encrypted Notes Manager (AES Secure CLI)

## What I Built
A command-line notes manager that allows you to write and read encrypted notes using AES-256 encryption. Using aes, block-modes, and hex crates to implement secure local storage.

## What I Learned
This application is a secure command-line tool for storing and retrieving private notes. Its core function is to protect your data at rest (i.e., when saved to disk). Instead of saving your notes as plain text, it encrypts them using the robust AES-256 encryption algorithm before writing them to a file (notes.db). When you want to read your notes, it reads the encrypted data from the file and decrypts it back to readable text. This ensures that even if someone gains access to the notes.db file, they cannot read its contents without the specific encryption key.

### Detailed Explanation of the Encryption Logic
The encryption process is the heart of this application. Here’s a step-by-step breakdown:

1. Algorithm and Mode: AES-256 in CBC Mode
AES-256 (Advanced Encryption Standard, 256-bit): This is a widely trusted, military-grade symmetric encryption algorithm. "Symmetric" means the same key is used to encrypt and decrypt. "256-bit" refers to the size of the key, making it extremely resistant to brute-force attacks.

CBC Mode (Cipher Block Chaining): AES encrypts data in fixed-size blocks (128 bits/16 bytes). CBC mode enhances security by XORing each plaintext block with the previous ciphertext block before encryption. This creates a "chain," meaning identical plaintext blocks will encrypt to different ciphertext blocks, hiding patterns in the data.

2. Key and Initialization Vector (IV)
Key (KEY): A 32-byte (256-bit) secret value used to encrypt and decrypt the data. Crucial Security Note: In this example, the key is hardcoded (b"an_example_very_secure_key_32byt!"). This is insecure for real-world use because anyone with access to the source code can find it. A secure application would derive a key from a user-provided master password.

- Initialization Vector (IV): A 16-byte random or unique value required for CBC mode. The IV ensures that encrypting the same message multiple times will produce completely different ciphertexts each time. Security Note: Like the key, a fixed IV (b"unique_iv_16byte") is used here for simplicity, which is a security weakness. In practice, a random IV should be generated for each encryption and stored alongside the ciphertext (typically prepended to it in plaintext).

3. The Encryption Process (encrypt function)
Input: A plain text string (e.g., "My secret").

- Cipher Setup: Aes256Cbc::new_from_slices(KEY, IV).unwrap() creates a new cipher instance ready to use our specific key and IV.

- Encryption: cipher.encrypt_vec(plain_text.as_bytes()) takes the raw bytes of the message and performs the encryption. This function also handles PKCS#7 padding, which adds bytes to the end of the message to ensure its length is a multiple of the AES block size (16 bytes).

- Encoding: The resulting ciphertext (a Vec<u8>) is converted into a hexadecimal string using hex::encode(). This creates a safe, readable string (e.g., a3f5...1b) that can be easily stored in a text file without corruption.

4. The Decryption Process (decrypt function)
Input: A hexadecimal string read from the file.

- Decoding: hex::decode(cipher_hex) converts the hex string back into the raw, encrypted bytes (Vec<u8>).

- Cipher Setup: The same cipher instance is created again with the identical KEY and IV.

- Decryption: cipher.decrypt_vec(&bytes) reverses the encryption process. It also automatically validates and removes the PKCS#7 padding that was added during encryption.

- Output: The decrypted bytes are converted from a Vec<u8> back into a readable String.

5. Data Storage
The application uses a simple text file, notes.db.

Each encrypted note (a hex string) is written to its own line in the file.

The read_notes() function reads the file line-by-line, decrypting each line individually to reconstruct the original list of notes.

Summary of Data Flow

``` 
User's Text -> (Encrypt) -> Encrypted Bytes -> (Hex Encode) -> Hex String -> Save to File

Hex String from File -> (Hex Decode) -> Encrypted Bytes -> (Decrypt) -> User's Text

```

Key Concepts for the Project:
- aes & block-modes Crates: Provide the low-level implementations of the AES algorithm and the CBC mode of operation.

- hex Crate: Essential for converting binary data (the ciphertext) into a storable/writable text format and back again.

- block-padding Crate: Handled internally by block-modes to manage data that isn't a perfect multiple of the block size.

- File I/O (std::fs, std::io): Manages reading from and writing to the persistent storage file (notes.db).

## Notes
### How to Run the Application:
```
cargo run
```
