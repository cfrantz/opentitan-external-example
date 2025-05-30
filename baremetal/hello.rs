#![no_std]
#![no_main]

use zerocopy::IntoBytes;
use base::Console;
use base::println;
use drivers::hmac::Hmac;
use traits::digest::{
    DigestInit, DigestOp,
    Sha2_256,
    Sha2_384,
};

#[no_mangle]
pub fn not_main(console: &mut Console) {
    println!("Hello World");
    println!("The console object is {:#?}", console as *const Console);

    let mut hmac = Hmac::new();
    let mut hasher = hmac.init(Sha2_256).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The SHA256 digest of 'test' is {:?}", result.as_bytes());

}
