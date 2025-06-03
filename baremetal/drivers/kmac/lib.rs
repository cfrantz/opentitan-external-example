#![no_std]
mod driver;
pub use driver::Kmac;
use kmac::enums::{EntropyMode, Kstrength, Mode};

use traits::digest::{
    Digest, DigestAlgorithm, ErrorKind, Error, ErrorType, DigestInit, DigestOp,
    Sha3_224,
    Sha3_256,
    Sha3_384,
    Sha3_512,
};


#[derive(Debug)]
pub struct KmacError(ErrorKind);

impl Error for KmacError {
    fn kind(&self) -> ErrorKind {
        self.0
    }
}

impl ErrorType for Kmac {
    type Error = KmacError;
}

pub struct Hasher<'a, T> {
    hw: &'a mut Kmac,
    _alg: T,
}

impl<T> ErrorType for Hasher<'_, T> {
    type Error = KmacError;
}

macro_rules! impl_sha3 {
    ($algo:ident, $strength:expr) => {
        impl DigestInit<$algo> for Kmac {
            type OpContext<'a> = Hasher<'a, $algo>;
            type Output = <$algo as DigestAlgorithm>::Digest;

            fn init<'a>(&'a mut self, init_params: $algo) -> Result<Self::OpContext<'a>, Self::Error> {
                self.configure(EntropyMode::SwMode, $strength, Mode::Sha3)
                    .map_err(|_| KmacError(ErrorKind::InitializationError))?;
                self.start();
                Ok(Self::OpContext {
                    hw: self,
                    _alg: init_params,
                })
            }
        }

        impl DigestOp for Hasher<'_, $algo> {
            type Output = <$algo as DigestAlgorithm>::Digest;
            fn update(&mut self, input: &[u8]) -> Result<(), Self::Error> {
                self.hw.absorb(input);
                Ok(())
            }
            fn finalize(self) -> Result<Self::Output, Self::Error> {
                Ok(Self::Output {
                    value: self.hw.squeeze()
                        .map_err(|_| KmacError(ErrorKind::FinalizationError))?,
                })
            }
        }
    };
}

impl_sha3!(Sha3_224, Kstrength::L224);
impl_sha3!(Sha3_256, Kstrength::L256);
impl_sha3!(Sha3_384, Kstrength::L384);
impl_sha3!(Sha3_512, Kstrength::L512);
