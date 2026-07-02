#![no_std]
#![no_main]

use uefi::prelude::*;
use log::info;

// This macro defines the true entry point, replacing the standard main()
#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // 1. Initialize UEFI services (Provides us with a screen/logger via GOP)
    uefi_services::init(&mut system_table).expect("Failed to initialize UEFI services");
    
    info!("_beautifulDATA Genesis Protocol Initiated.");
    info!("Architect recognized. Preparing for severance...");

    // 2. Prepare a raw byte buffer to hold the physical memory map.
    // The UEFI firmware requires us to provide memory for it to write the map into.
    let mut mmap_storage = [0u8; 1024 * 16]; 

    // 3. THE SEVERANCE (ExitBootServices)
    // We request the memory map and immediately terminate the firmware's control.
    let (_system_table, _memory_map) = system_table
        .exit_boot_services(image, &mut mmap_storage)
        .expect("Critical Failure: Unable to sever UEFI firmware link.");

    // --- RING 0 BARE METAL ESTABLISHED ---
    // At this exact line, the motherboard is deaf and blind. We are in absolute control.
    // Standard library is gone. UEFI is gone. We own the silicon.

    // 4. Infinite spin loop. 
    // Since we have no kernel logic yet, if we drop out of this function, the CPU will triple-fault and crash.
    loop {
        core::hint::spin_loop();
    }
}

// A custom panic handler is strictly required in #![no_std] environments.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // In the future, this will trigger our Sentinel autonomous quarantine protocols.
    loop {
        core::hint::spin_loop();
    }
}