# Preview
![Preview error](https://github.com/yaroslav957/efifetch/blob/master/previews/v0.1.9.0.png)

Efifetch currently supports only VGA output (some resolution issues with serialport0 on QEMU)

# How to use

## In a VM
1. Download dependencies and firmware files (Fedora linux):
```
sudo dnf install qemu-kvm edk2-ovmf
```

```
cp /usr/share/OVMF/OVMF_CODE.fd .
cp /usr/share/OVMF/OVMF_VARS.fd .
```

2. Prepare dir and copy efifetch.efi:
```
mkdir -p esp/efi/boot && cp efifetch.efi esp/efi/boot/bootx64.efi
```

3. Run VM:
```
qemu-system-x86_64 -enable-kvm \
-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
-drive format=raw,file=fat:rw:esp
```

## On hardware

Efifetch has never been tested on real hardware. Use it on a real device at your own risk

# Features info & todos

TODO list:
- PCI configuration space 
- PCIe MMIO (optional idk)
- CPU MSRs (AMD and Intel)
- SMBIOS data
- SMBUS data (partially working)
- CPU frequency
- UEFI configuration tables
- UEFI variables
- ACPI tables
- Read LBA from HDD
- AHCI MMIO (optional)
- INT15 E820 tables (optional)
