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
1. **Clone the repo:**
    ```sh
    git clone https://github.com/yaroslav957/efifetch.git
    cd efifetch
    ```
2. **Build UEFI executable:**
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
## Running on virtual machine
To run `efifetch` on VM, you will need:
* **Firmware files**
* **QEMU virtual machine**
1. **Install QEMU & OVMF:**
    ```sh
    # Debian/Ubuntu
    sudo apt-get install qemu ovmf

    # Fedora
    sudo dnf install qemu-kvm edk2-ovmf
    ```
2. **Copy Firmware files:**
    ```sh
    cp /usr/share/OVMF/OVMF_CODE.fd .
    cp /usr/share/OVMF/OVMF_VARS.fd .
    ```
3. **Copy UEFI-Shell executable & efifetch:**
    ```sh
    cp uefi-shell.efi esp/efi/boot/bootx64.efi
    cp target/x86_64-unknown-uefi/release/efifetch.efi esp/
    ```
4. **Run virtual machine**
    ```sh
    qemu-system-x86_64 -enable-kvm \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
    ```

## License
This project is licensed under the **MIT** License

