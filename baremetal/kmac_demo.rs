use zerocopy::IntoBytes;
use base::{println, HexEncode};
use drivers::kmac::Kmac;
use traits::digest::{
    DigestInit, DigestOp, DigestAlgorithm,
    Sha3_256,
    Sha3_384,
    Sha3_512,
};

use crypto::Software;

fn hash_2_things<A: DigestAlgorithm, D: DigestInit<A>>(hw: &mut D, algo: A) {

    let mut hasher = hw.init(algo).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The 1st SHA3-256 digest of 'test' is {}", HexEncode(result.as_bytes()));


    let mut hasher = hw.init(algo).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The 2nd SHA3-256 digest of 'test' is {}", HexEncode(result.as_bytes()));
}

fn hash_differently<D: DigestInit<Sha3_256> + DigestInit<Sha3_384>>(hw: &mut D) {

    let mut hasher = hw.init(Sha3_256).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The SHA3-256 digest of 'test' is {}", HexEncode(result.as_bytes()));

    let mut hasher = hw.init(Sha3_384).expect("init");
    hasher.update(b"test").expect("update");
    let result = hasher.finalize().expect("finalize");
    println!("The SHA3-384 digest of 'test' is {}", HexEncode(result.as_bytes()));

}

pub fn demo() {
    println!("Demonstrate KMAC hardware block");
    let mut kmac = Kmac::new();
    hash_2_things(&mut kmac, Sha3_512);
    hash_differently(&mut kmac);

    println!("Demonstrate Software implementation");
    hash_2_things(&mut Software, Sha3_512);
    hash_differently(&mut Software);
}
