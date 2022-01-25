use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

const SERIAL_IO_PORT: u16 = 0x3F8;

lazy_static! {
    pub static ref SERIAL_WRITER: Mutex<SerialPort> =
        Mutex::new(unsafe { SerialPort::new(SERIAL_IO_PORT) });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL_WRITER
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

/*use alloc::vec;
use alloc::vec::Vec;

#[macro_export]
macro_rules! serialrln {
    () => ($crate::serialr_until!('\n'));
}

#[macro_export]
macro_rules! serialr_until {
    ($character:expr) => {
        $crate::serial::_serialr_until($character)
    };
}

#[doc(hidden)]
pub fn _serialr_until(character: char) -> Vec<char> {
    let mut output: Vec<char> = vec![];
    loop {
        let input = SERIAL_WRITER.lock().receive() as char;

        if character == input {
            break output;
        }

        output.push(input);
    }
}
*/
