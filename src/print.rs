use core::fmt::{self, Write};
use ns16550a::*;
use spin::Mutex;

// Global UART instance
// pub static mut UART: Option<Uart> = None;

static UART: Mutex<Uart> = Mutex::new(Uart::new(0x8000_0000_1fe0_01e0));

/// Initialize the UART with standard settings
pub fn uart_init() {
    UART.lock().init(
        WordLength::EIGHT,
        StopBits::ONE,
        ParityBit::DISABLE,
        ParitySelect::EVEN,
        StickParity::DISABLE,
        Break::DISABLE,
        DMAMode::MODE0,
        Divisor::BAUD115200,
    );
}

struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // UART.lock().write_str(s).unwrap();
        for c in s.chars() {
            if c == '\n' {
                UART.lock().write_char('\r').unwrap();
            }
            UART.lock().write_char(c).unwrap();
        }
        Ok(())
    }
}

/// Print macro for UART output
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::print::_print(format_args!($($arg)*));
    };
}

/// Println macro for UART output
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    };
}

/// Internal print function used by the macros
pub fn _print(args: core::fmt::Arguments) {
    let _ = UartWriter.write_fmt(args);
}
