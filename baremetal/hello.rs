#![no_std]
#![no_main]

use zerocopy::IntoBytes;
use base::Console;
use base::println;
use drivers::hmac::{Hmac, Algorithm};
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
    println!("got hmac");
    let mut hasher = <Hmac as DigestInit<Sha2_384>>::init(&mut hmac, Sha2_384).expect("init");
    println!("got hasher");
    hasher.update(b"test").expect("update");
    println!("got updated");
    let result = hasher.finalize().expect("finalize");

    /*
    println!("Configure");
    hmac.configure(Algorithm::Sha2_256);
    hmac.start();
    println!("update");
    hmac.update(b"test");
    println!("process");
    hmac.process();
    //println!("wait");
    hmac.wait_for_done();
    let result = hmac.digest::<8>();
    */
    println!("The digest of 'test' is {:?}", result.as_bytes());

}
