# rustpass

A secure, cross-platform terminal-based password manager written in Rust.

## Usage

Download the latest release archive from the [Releases page](https://github.com/YOUR_USERNAME/rustpass/releases). Archives are available for Linux (`tar.xz`), macOS (`tar.gz`), and Windows (`zip`).

After downloading:

bash
# Linux (tar.xz)
``` tar -xJf rustpass-linux-x86_64.tar.xz ```

# macOS x86_64 (tar.gz)
```tar -xzf rustpass-macos-x86_64.tar.gz```

# macOS ARM64 (tar.gz)
```tar -xzf rustpass-macos-arm64.tar.gz```

# Windows (PowerShell)
```Expand-Archive -Path rustpass-windows-x86_64.zip -DestinationPath .```


#Once Extracted

## On Linux
```chmod +x rustpass```
```./rustpass```

## On macOS
```xattr -d com.apple.quarantine ./rustpass```
```chmod +x ./rustpass```
```./rustpass```

## On Windows (Command Prompt or PowerShell)
```.\rustpass.exe```


## From source
```git clone git@github.com:sudhanvarao28/RustPass.git```
```cd rustpass```
```cargo build --release```

Binary will be located at target/release/rustpass

