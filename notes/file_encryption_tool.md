# Project 047 – File Encryption Tool  

## What I Built
A CLI utility that encrypts and decrypts files using AES symmetric encryption. aes and cipher crates are used to safely encode file contents—perfect for securing sensitive files or building password-protected backups.

## What I Learned

## Notes
##### Generate a key (32 bytes = 64 hex chars):
bash
```
# On Linux/macOS:
head -c 32 /dev/urandom | xxd -p -c 32
# Or with OpenSSL:
openssl rand -hex 32
```
Example key: 5f4dcc3b5aa765d61d8327deb882cf99b6c2e5d8a83c07b0e7f5a7a53c7f5e5a


##### Sample Interaction:
```
Enter path to file to encrypt: secret.txt
Enter 32-byte key (hex): 000102030405060708090a0b0c0d0e0f000102030405060708090a0b0c0d0e0f
Output file path: secret.enc
✅ File encrypted to 'secret.enc'
```

The encrypted file format is: [16-byte IV][encrypted data]

