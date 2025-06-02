use zerocopy::IntoBytes;
use base::{println, HexEncode};
use drivers::kmac::Kmac;
use traits::digest::{
    DigestInit, DigestOp,
    Sha3_512,
};


pub fn demo() {
    let mut kmac = Kmac::new();

    let mut hasher = kmac.init(Sha3_512).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");

    println!("The SHA3-512 digest of 'test' is {}", HexEncode(result.as_bytes()));
}
