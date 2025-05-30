#![no_std]
mod driver;

use base::println;
use ufmt;

use driver::*;
pub use driver::Hmac;
use hmac::enums::DigestSize;

use traits::digest::{
    Digest, DigestAlgorithm, ErrorKind, Error, ErrorType, DigestInit, DigestOp,
    Sha2_256,
    Sha2_384,
    Sha2_512,
};

#[derive(Debug)]
pub struct HmacError(ErrorKind);

impl Error for HmacError {
    fn kind(&self) -> ErrorKind {
        self.0
    }
}

impl ErrorType for Hmac {
    type Error = HmacError;
}

pub struct ContextSha256<'a> {
    hw: &'a mut Hmac,
}

impl ErrorType for ContextSha256<'_> {
    type Error = HmacError;
}

impl DigestInit<Sha2_256> for Hmac {
    type OpContext<'a> = ContextSha256<'a>;

    fn init<'a>(&'a mut self, _init_params: Sha2_256) -> Result<Self::OpContext<'a>, Self::Error> {
        self.configure(DigestSize::Sha2256);
        self.start();
        Ok(ContextSha256 {
            hw: self,
        })
    }
}

impl DigestOp for ContextSha256<'_> {
    type Output = <Sha2_256 as DigestAlgorithm>::Digest;
    fn update(&mut self, input: &[u8]) -> Result<(), Self::Error> {
        self.hw.update(input);
        Ok(())
    }
    fn finalize(self) -> Result<Self::Output, Self::Error> {
        self.hw.process();
        self.hw.wait_for_done();
        Ok(Self::Output {
            value: self.hw.digest(),
        })
    }
}




pub struct ContextSha384<'a> {
    hw: &'a mut Hmac,
}

impl ErrorType for ContextSha384<'_> {
    type Error = HmacError;
}

impl DigestInit<Sha2_384> for Hmac {
    type OpContext<'a> = ContextSha384<'a>;

    fn init<'a>(&'a mut self, _init_params: Sha2_384) -> Result<Self::OpContext<'a>, Self::Error> {
        self.configure(DigestSize::Sha2384);
        self.start();
        Ok(ContextSha384 {
            hw: self,
        })
    }
}

impl DigestOp for ContextSha384<'_> {
    type Output = <Sha2_384 as DigestAlgorithm>::Digest;
    fn update(&mut self, input: &[u8]) -> Result<(), Self::Error> {
        self.hw.update(input);
        Ok(())
    }
    fn finalize(self) -> Result<Self::Output, Self::Error> {
        self.hw.process();
        self.hw.wait_for_done();
        Ok(Self::Output {
            value: self.hw.digest(),
        })
    }
}




pub struct ContextSha512<'a> {
    hw: &'a mut Hmac,
}

impl ErrorType for ContextSha512<'_> {
    type Error = HmacError;
}

impl DigestInit<Sha2_512> for Hmac {
    type OpContext<'a> = ContextSha512<'a>;

    fn init<'a>(&'a mut self, _init_params: Sha2_512) -> Result<Self::OpContext<'a>, Self::Error> {
        self.configure(DigestSize::Sha2512);
        self.start();
        Ok(ContextSha512 {
            hw: self,
        })
    }
}

impl DigestOp for ContextSha512<'_> {
    type Output = <Sha2_512 as DigestAlgorithm>::Digest;
    fn update(&mut self, input: &[u8]) -> Result<(), Self::Error> {
        self.hw.update(input);
        Ok(())
    }
    fn finalize(self) -> Result<Self::Output, Self::Error> {
        self.hw.process();
        self.hw.wait_for_done();
        Ok(Self::Output {
            value: self.hw.digest(),
        })
    }
}
