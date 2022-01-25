#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

pub mod exit_qemu;
pub mod fb_logger;
pub mod gdt;
pub mod init;
pub mod interrupts;
pub mod panic_handler;
pub mod serial;
#[cfg(test)]
pub mod test;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::fb_println!($($arg)*);
        $crate::serial_println!($($arg)*);
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::fb_print!($($arg)*);
        $crate::serial_print!($($arg)*);
    };
}

entry_point!(kernel_main);

/// The main starting point of the program
#[cfg(test)]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init::init(boot_info);
    test_main();
    loop {}
}

/// The main starting point of the program
#[cfg(not(test))]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init::init(boot_info);
    //panic!("My panic world");
    loop {}
}
