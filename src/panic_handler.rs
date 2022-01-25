#[cfg(test)]
use crate::test::test_panic_handler;
#[cfg(not(test))]
use crate::{exit_qemu, serial_println};
use core::panic::PanicInfo;

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
    test_panic_handler(info);
}
