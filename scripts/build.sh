#!/bin/bash
set -e

echo "[*] Initiating _beautifulOS Compilation Sequence..."

# 1. Compile the Bare-Metal Microkernel
echo "[*] Compiling Rust logic for x86_64-unknown-uefi target..."
cargo build --target x86_64-unknown-uefi --release

# The Rust compiler outputs the raw PE/COFF file here:
RAW_EFI="target/x86_64-unknown-uefi/release/beautifulOS.efi"

# 2. Cryptographic Bootloader Signing (Epic 3 Integration)
echo "[*] Cryptographically signing the bootloader (Zero-Trust Enforcement)..."
sbsign --key docs/secure_boot/keys/db.key \
       --cert docs/secure_boot/keys/db.crt \
       --output bootx64.efi \
       $RAW_EFI

# 3. Forging the EFI System Partition (ESP) Image
echo "[*] Constructing the FAT32 boot image..."
IMAGE_NAME="beautifulOS.img"

# Allocate a blank 64MB image file
dd if=/dev/zero of=$IMAGE_NAME bs=1M count=64 status=none

# Format the image as FAT32
mformat -i $IMAGE_NAME -F ::

# 4. Injecting the Signed Bootloader
echo "[*] Injecting signed bootloader into the EFI directory structure..."
mmd -i $IMAGE_NAME ::/EFI
mmd -i $IMAGE_NAME ::/EFI/BOOT
mcopy -i $IMAGE_NAME bootx64.efi ::/EFI/BOOT/BOOTX64.EFI

echo "[+] Compilation Complete. The artifact '$IMAGE_NAME' is ready for deployment."