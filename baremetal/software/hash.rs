use traits::digest::{
    Digest, DigestAlgorithm, ErrorKind, Error, ErrorType, DigestInit, DigestOp,
    Sha2_256,
    Sha2_384,
    Sha2_512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
};
use sha2::Digest as SwDigest;

pub struct Software;


impl ErrorType for Software {
    type Error = core::convert::Infallible;
}

pub struct Hasher<T> {
    alg: T,
}

impl<T> ErrorType for Hasher<T> {
    type Error = core::convert::Infallible;
}

macro_rules! impl_hashing {
    ($algo:ident, $digest_type:ty) => {
        impl DigestInit<$algo> for Software {
            type OpContext<'a> = Hasher<$digest_type>;
            type Output = <$algo as DigestAlgorithm>::Digest;

            fn init<'a>(&'a mut self, _init_params: $algo) -> Result<Self::OpContext<'a>, Self::Error> {
                Ok(Self::OpContext {
                    alg: <$digest_type>::new(),
                })
            }
        }

        impl DigestOp for Hasher<$digest_type> {
            type Output = <$algo as DigestAlgorithm>::Digest;
            fn update(&mut self, input: &[u8]) -> Result<(), Self::Error> {
                self.alg.update(input);
                Ok(())
            }
            fn finalize(self) -> Result<Self::Output, Self::Error> {
                Ok(Self::Output {
                    value: unsafe {
                        // SAFETY: probably not.
                        core::mem::transmute(self.alg.finalize())
                    },
                })
            }
        }
    };
}

impl_hashing!(Sha2_256, sha2::Sha256);
impl_hashing!(Sha2_384, sha2::Sha384);
impl_hashing!(Sha2_512, sha2::Sha512);
impl_hashing!(Sha3_256, sha3::Sha3_256);
impl_hashing!(Sha3_384, sha3::Sha3_384);
impl_hashing!(Sha3_512, sha3::Sha3_512);
