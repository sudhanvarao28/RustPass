# RustPass

A secure, cross-platform terminal-based password manager written in Rust.

# Encryption & Security

RustPass uses AES-256-GCM encryption and Argon2id key derivation to securely encrypt and store your passwords.

## How it works
- Master Password: When setting up RustPass, you're asked to create a master password.
- Salt + Argon2: Your master password is hashed with a random 16-byte salt using the Argon2id algorithm.
- AES Encryption: A 32-byte encryption key is derived from the master password hash and used with AES-256-GCM to encrypt/decrypt password entries.
- Nonce: A 12-byte random nonce is generated for each encryption to ensure uniqueness and protect against replay attacks.
- Storage: The encrypted data is stored in a local embedded database (using sled), in the following format:
```[salt (16 bytes)] + [nonce (12 bytes)] + [ciphertext]```

## Usage

Download the latest release archive from the [Releases page](https://github.com/sudhanvarao28/RustPass/releases). Archives are available for Linux (`tar.xz`), macOS (`tar.gz`), and Windows (`zip`).

Its recommended to create a new dedicated directory and uncompress the binary to the newly created folder.


# After downloading


### Linux (tar.xz)
``` tar -xJf rustpass-linux-x86_64.tar.xz ```

### macOS x86_64 (tar.gz)
```tar -xzf rustpass-macos-x86_64.tar.gz```

### macOS ARM64 (tar.gz)
```tar -xzf rustpass-macos-arm64.tar.gz```

### Windows (PowerShell)
```Expand-Archive -Path rustpass-windows-x86_64.zip -DestinationPath .```


# Once Extracted

### On Linux
```chmod +x rustpass```
```./rustpass```

### On macOS
```xattr -d com.apple.quarantine ./rustpass```
```chmod +x ./rustpass```
```./rustpass```

### On Windows (Command Prompt or PowerShell)
```.\rustpass.exe```


### From source
```git clone git@github.com:sudhanvarao28/RustPass.git```
```cd rustpass```
```cargo build --release```

Binary will be located at target/release/rustpass

