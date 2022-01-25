#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::fmt::Write;
use bootloader::{entry_point, BootInfo};

mod exit_qemu;
mod interrupts;
mod serial;
mod test;
mod init;
mod panic_handler;
mod fb_logger;


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
