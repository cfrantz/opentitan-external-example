#![no_std]
#![no_main]

use base::Console;
use base::println;

mod hmac_demo;
mod kmac_demo;

#[no_mangle]
pub fn not_main(console: &mut Console) {
    println!("Hello World");
    println!("The console object is {:#?}", console as *const Console);

    hmac_demo::demo();
    kmac_demo::demo();
}
