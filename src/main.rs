#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

mod vga_buffer;
mod serial;
mod test;
mod exit_qemu;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[PANIC] {}", info);
    exit_qemu::exit(exit_qemu::QemuExitCode::Failed);
    loop {}
}

/// This function is called on panic.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test::test_panic_handler(info);
}

entry_point!(kernel_main);

/// The main starting point of the program
#[cfg(test)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial_println!("[INFO] Running AtomOS with bootloader version {}.{}.{}", boot_info.version_major, boot_info.version_minor, boot_info.version_patch);

    test_main();

    loop {}
}

/// The main starting point of the program
#[cfg(not(test))]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    serial_println!("[INFO] Running AtomOS with bootloader version {}.{}.{}", boot_info.version_major, boot_info.version_minor, boot_info.version_patch);

    // turn the screen dark gray
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        serial_println!(
                    "[INFO] Framebuffer is {} wide and {} tall and uses the {:?} pixel format",
                    info.horizontal_resolution,
                    info.vertical_resolution,
                    info.pixel_format
                );
        for byte in framebuffer.buffer_mut() {
            *byte = 0x30;
        }
    }

    // turn the screen light gray
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        for byte in framebuffer.buffer_mut() {
            *byte = 0x90;
        }
    }

    //panic!("My panic world");

    loop {}
}
