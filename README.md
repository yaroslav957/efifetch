# Efifetch
![Stars](https://img.shields.io/github/stars/yaroslav957/efifetch?style=social)
![Issues](https://img.shields.io/github/issues/yaroslav957/efifetch?style=social)
![Commit](https://img.shields.io/github/last-commit/yaroslav957/efifetch?style=social)
![Preview](https://github.com/yaroslav957/efifetch/blob/main/assets/preview.png)

A simple system information tool for UEFI environments, designed to run directly from a UEFI-shell. `efifetch` aims to provide concise and useful system details in a format inspired by popular system fetch tools like `fastfetch` and `neofetch`, but tailored for the unique UEFI runtime environment.

## Targets support
* `x86_64-unknown-uefi`
* `i686-unknown-uefi`
* `aarch64-unknown-uefi`

## Installation & Setup
To build `efifetch` from source, you will need:
* **Rust Toolchain** (see MSRV in `Cargo.toml`)
* **UEFI-shell** for execution
1.  **Clone the repo:**
    ```sh
    git clone https://github.com/yaroslav957/efifetch.git
    cd efifetch
    ```
2.  **Build UEFI executable:**
    ```sh
    # build for x86_64-unknown-uefi, for example
    rustup target add x86_64-unknown-uefi
    cargo build --release
    ```
## Running on real hardware
**Copy UEFI-Shell executable & efifetch:**
```sh
cp uefi-shell.efi /path/to/usb_drive/boot/efi/bootx64.efi
cp target/x86_64-unknown-uefi/release/efifetch.efi /path/to/usb_drive/efi/boot/
```

