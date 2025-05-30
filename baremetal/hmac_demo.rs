use zerocopy::IntoBytes;
use base::{println, HexEncode};
use drivers::hmac::Hmac;
use traits::digest::{
    DigestInit, DigestOp,
    Sha2_256,
    Sha2_384,
};

pub fn demo() {
    let mut hmac = Hmac::new();
    let mut hasher = hmac.init(Sha2_256).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The SHA2-256 digest of 'test' is {}", HexEncode(result.as_bytes()));

    let mut hasher = hmac.init(Sha2_384).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The SHA2-384 digest of 'test' is {}", HexEncode(result.as_bytes()));
}
