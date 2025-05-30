use uart;
use ufmt::uWrite;
use core::convert::Infallible;

pub struct Console {
    uart: uart::Uart0,
}

impl Default for Console {
    fn default() -> Self {
        Console {
            uart: unsafe { uart::Uart0::new() },
        }
    }
}

impl uWrite for Console {
    type Error = Infallible;

    fn write_str(&mut self, data: &str) -> Result<(), Self::Error> {
        let reg = self.uart.regs_mut();
        for byte in data.as_bytes() {
            while reg.status().read().txfull() {
                // Wait.
            }
            reg.wdata().write(|w| w.wdata(*byte as u32));
        }
        Ok(())
    }

}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        ufmt::uwrite!(&mut $crate::Console::default(), $($arg)*).unwrap();
    };
}

#[macro_export]
macro_rules! println {
    () => {
        ufmt::uwrite!(&mut $crate::Console::default(), "\r\n").unwrap();
    };

    ($($arg:tt)*) => {
        ufmt::uwrite!(&mut $crate::Console::default(), $($arg)*).unwrap();
        ufmt::uwrite!(&mut $crate::Console::default(), "\r\n").unwrap();
    };
}
