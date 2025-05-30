use core::panic::PanicInfo;
use crate::{print, println};
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("PANIC");
    if let Some(loc) = info.location() {
        print!(" at {}:{}:{}", loc.file(), loc.line(), loc.column());
    }
    if let Some(msg) = info.message().as_str() {
        print!(": {}", *msg);
    }
    println!("!");
    loop {}
}
