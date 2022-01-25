mod tests;

use crate::exit_qemu;
use crate::{serial_print, serial_println};
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("[INFO] Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu::exit(exit_qemu::QemuExitCode::Success);
}

#[allow(dead_code)]
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n\n[PANIC] {}\n", info);
    exit_qemu::exit(exit_qemu::QemuExitCode::Failed);
    loop {}
}
