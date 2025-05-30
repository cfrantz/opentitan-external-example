use zerocopy::IntoBytes;
use base::{println, HexEncode};
use drivers::kmac::Kmac;

use kmac::enums::{EntropyMode, Kstrength, Mode};

pub fn demo() {
    let mut kmac = Kmac::new();

    kmac.configure(EntropyMode::SwMode, Kstrength::L512, Mode::Sha3).expect("configure");
    kmac.start();
    kmac.absorb(b"test");
    let result = kmac.squeeze::<16>().expect("squeeze");

    println!("The SHA3-512 digest of 'test' is {}", HexEncode(result.as_bytes()));
}
