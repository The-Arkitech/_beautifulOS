# Requires Run as Administrator
Write-Host "[*] Initiating _beautifulOS Native Windows Compilation Sequence..." -ForegroundColor Cyan

# 1. Compile the Bare-Metal Microkernel
Write-Host "[*] Compiling Rust logic for x86_64-unknown-uefi target..."
cargo build --target x86_64-unknown-uefi --release
$RawEfi = "target\x86_64-unknown-uefi\release\beautifulOS.efi"

# 2. Cryptographic Bootloader Signing 
# Note: This requires signtool.exe, which is installed via the Windows 10/11 SDK.
Write-Host "[*] Cryptographically signing the bootloader (Zero-Trust Enforcement)..."
# We sign the raw EFI file using the fused .pfx key we generated.
signtool sign /fd SHA256 /f docs\secure_boot\keys\db.pfx /p beautiful /out bootx64.efi $RawEfi

if (-Not (Test-Path "bootx64.efi")) {
    Write-Host "[!] Signing failed. Ensure Windows SDK is installed and signtool is in your PATH." -ForegroundColor Red
    exit
}

# 3. Forging the EFI System Partition (ESP) Image
Write-Host "[*] Constructing the FAT32 Virtual Boot Image (VHD)..."
$ImageName = "beautifulOS.vhd"
$VhdPath = Join-Path (Get-Location) $ImageName

# Destroy old image if it exists
if (Test-Path $VhdPath) { Remove-Item $VhdPath -Force }

# Allocate and mount a blank 64MB Virtual Hard Disk
New-VHD -Path $VhdPath -SizeBytes 64MB -Dynamic | Out-Null
$Disk = Mount-VHD -Path $VhdPath -PassThru
$DiskNumber = $Disk.DiskNumber

# Initialize and Format the raw disk as FAT32
Initialize-Disk -Number $DiskNumber -PartitionStyle MBR | Out-Null
$Partition = New-Partition -DiskNumber $DiskNumber -UseMaximumSize -IsActive -AssignDriveLetter
Format-Volume -Partition $Partition -FileSystem FAT32 -NewFileSystemLabel "EFI" -Confirm:$false | Out-Null

$DriveLetter = $Partition.DriveLetter

# 4. Injecting the Signed Bootloader
Write-Host "[*] Injecting signed bootloader into the EFI directory structure..."
$BootDir = "$DriveLetter`:\EFI\BOOT"
New-Item -Path $BootDir -ItemType Directory -Force | Out-Null
Copy-Item -Path "bootx64.efi" -Destination "$BootDir\BOOTX64.EFI" -Force

# Sever the mount
Dismount-VHD -Path $VhdPath | Out-Null

Write-Host "[+] Compilation Complete. The artifact '$ImageName' is ready for deployment." -ForegroundColor Green