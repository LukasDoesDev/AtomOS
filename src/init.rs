use bootloader::{BootInfo};
use crate::{interrupts, println, serial_println};
use crate::fb_logger::{init_logger, LOGGER};

pub fn init(boot_info: &'static mut BootInfo) {
    interrupts::init_idt();
    serial_println!("[INFO] IDT initialized!");
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        /*for byte in framebuffer.buffer_mut() {
            *byte = 0x30;
        }*/
        init_logger(framebuffer.buffer_mut(), info);
        println!("[INFO] Framebuffer logger initialized!");
        println!(
            "[INFO] Framebuffer is {} wide and {} tall and uses the {:?} pixel format",
            info.horizontal_resolution,
            info.vertical_resolution,
            info.pixel_format
        );
    }
    println!(
        "[INFO] Running AtomOS with bootloader version {}.{}.{}",
        boot_info.version_major,
        boot_info.version_minor,
        boot_info.version_patch
    );
}
