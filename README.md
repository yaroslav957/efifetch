# Preview
![Preview error](https://github.com/yaroslav957/efifetch/blob/master/previews/v0.1.8.1.png)

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

cp efifetch.efi esp/EFI/BOOT/BOOTX64.EFI
```

# Features info

- [ ] **Miscellaneous (75% done)**
    - [x] Resolution
    - [x] Date
    - [x] Colors 
    - [ ] Image info
- [ ] **Cpu (55% done)**
    - [x] Vendor
    - [x] Brand info
    - [ ] Cores
    - [ ] Threads
    - [ ] Logical ids
    - [x] Vmx
    - [x] Smx
    - [x] Hypervisor info
    - [ ] Syscall info
- [ ] **Memory map (66% done)**
    - [x] Pages
    - [x] Physical/Virtual addr
    - [ ] Type
