#![no_main]
#![no_std]

use uart;
use ureg::MmioMut;

#[inline(never)]
fn uart_write<T: MmioMut>(reg: uart::RegisterBlock<T>, data: &str) {
    for byte in data.as_bytes() {
        while reg.status().read().txfull() {
            // Wait.
        }
        reg.wdata().write(|w| w.wdata(*byte as u32));
    }
}

fn not_main() {
    let mut u = unsafe {
        uart::Uart0::new()
    };
    uart_write(u.regs_mut(), "Hello World\r\n");
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    not_main();
    loop {
        // The end.
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}
