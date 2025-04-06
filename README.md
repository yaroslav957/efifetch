# Preview
![Preview error](https://github.com/yaroslav957/efifetch/blob/master/previews/v0.1.9.0.png)

efifetch currently supports only VGA output (some resolution issues with serialport0 on QEMU)

# How to use

## in a VM
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

## on Hardware
Preparing USB drive (linux):
```
sgdisk \
    --clear \
    --new=1:1M:10M \
    --typecode=1:C12A7328-F81F-11D2-BA4B-00A0C93EC93B \
    "here /path"

mkfs.fat "here /path_to_disk_partition" 

mkdir esp
mount "here /path_to_disk_partition" esp

mkdir esp/EFI/BOOT

cp efifetch.efi esp/EFI/BOOT/BOOTX64.efi
```

# Features info & todos

TODO:
1. Information:
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
   

2. Redisign
  - Eventloop for `F{x}` to open pages
  - NET page = `<F1>`
  - CPU page = `<F2>`
  - MEMORY page = `<F3>`
  - EFI page = `<F3>`
  - PCI page = `<F4>`
  - ACPI tables page = `<F5>`
  - SMBIOS page = `<F6>`
  - HOST page = `<F7>`
