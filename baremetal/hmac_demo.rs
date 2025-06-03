use zerocopy::IntoBytes;
use base::{println, HexEncode};
use drivers::hmac::Hmac;
use traits::digest::{
    DigestInit, DigestOp,
    DigestAlgorithm,
    Sha2_256,
    Sha2_384,
};

use crypto::Software;

fn hash_2_things<A: DigestAlgorithm, D: DigestInit<A>>(hw: &mut D, algo: A) {

    let mut hasher = hw.init(algo).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The 1st SHA2-256 digest of 'test' is {}", HexEncode(result.as_bytes()));


    let mut hasher = hw.init(algo).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The 2nd SHA2-256 digest of 'test' is {}", HexEncode(result.as_bytes()));
}

fn hash_differently<D: DigestInit<Sha2_256> + DigestInit<Sha2_384>>(hw: &mut D) {

    let mut hasher = hw.init(Sha2_256).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The SHA2-256 digest of 'test' is {}", HexEncode(result.as_bytes()));

    let mut hasher = hw.init(Sha2_384).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The SHA2-384 digest of 'test' is {}", HexEncode(result.as_bytes()));

}

pub fn demo() {
    println!("Demonstrate Hmac hardware block");
    let mut hmac = Hmac::new();
    hash_2_things(&mut hmac, Sha2_256);
    hash_differently(&mut hmac);

    println!("Demonstrate Software implementation");
    hash_2_things(&mut Software, Sha2_256);
    hash_differently(&mut Software);
}
