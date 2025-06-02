use kmac;
use ureg::{MmioMut, Mmio};
use core::result::Result;

use base::println;

pub struct Kmac {
    kmac: kmac::Kmac,
}

impl Kmac {
    const STATE_BITS: usize = 1600;
    const STATE_WORDS: usize = Self::STATE_BITS / 32;
    const STATE_SHARE_0: usize = 0;
    const STATE_SHARE_1: usize = 64;

    const FAKE_SEED: [u32; 9] = [
        1,2,3,4,5,6,7,8,9,
    ];
    pub fn new() -> Self {
        Kmac {
            kmac: unsafe { kmac::Kmac::new() },
        }
    }

    fn keccak_rate(strength: kmac::enums::Kstrength) -> Result<usize, ()> {
        use kmac::enums::Kstrength::*;
        match strength {
            L128 => Ok((Self::STATE_BITS - 2 * 128) / 32),
            L224 => Ok((Self::STATE_BITS - 2 * 224) / 32),
            L256 => Ok((Self::STATE_BITS - 2 * 256) / 32),
            L384 => Ok((Self::STATE_BITS - 2 * 384) / 32),
            L512 => Ok((Self::STATE_BITS - 2 * 512) / 32),
            _ => Err(()),
        }
    }


    fn poll_status<T, F>(regs: kmac::RegisterBlock<T>, check: F) -> Result<(), ()>
    where
        T: Mmio,
        F: Fn(kmac::regs::StatusReadVal) -> bool,
    {
        loop {
            if regs.intr_state().read().kmac_err() {
                return Err(());
            }
            let sts = regs.status().read();
            if check(sts) {
                return Ok(());
            }
        }
    }

    pub fn configure(&mut self,
        entropy_mode: kmac::enums::EntropyMode,
        strength: kmac::enums::Kstrength,
        //key_len: kmac::enums::Len,
        mode: kmac::enums::Mode,
    ) -> Result<(), ()> {
        let regs = self.kmac.regs_mut();
        Self::poll_status(regs, |status| status.sha3_idle())?;
        regs.entropy_period().write(|val| val.prescaler(u32::MAX).wait_timer(u32::MAX));

        regs.cfg_shadowed().write(|val|
            val.kmac_en(false)
               .kstrength(|_| strength)
               .mode(|_| mode)
               .msg_endianness(false)
               .state_endianness(false)
               .sideload(false)
               .entropy_mode(|_| entropy_mode)
               .entropy_fast_process(true)
               .entropy_ready(true)
               .msg_mask(false)
               .en_unsupported_modestrength(false)
        );
        regs.cfg_shadowed().write(|val|
            val.kmac_en(false)
               .kstrength(|_| strength)
               .mode(|_| mode)
               .msg_endianness(false)
               .state_endianness(false)
               .sideload(false)
               .entropy_mode(|sel| sel.sw_mode())
               .entropy_fast_process(true)
               .entropy_ready(true)
               .msg_mask(false)
               .en_unsupported_modestrength(false)
        );

        for val in Self::FAKE_SEED {
            regs.entropy_seed().write(|_| val.into());
        }
        Ok(())
    }

    fn issue_command<T: MmioMut>(regs: kmac::RegisterBlock<T>, cmd: kmac::enums::Cmd) {
        regs.cmd().write(|_| u32::from(cmd).into())
    }

    pub fn start(&mut self) {
        Self::issue_command(self.kmac.regs_mut(), kmac::enums::Cmd::Start);
    }

    pub fn absorb(&mut self, data: &[u8]) {
        let regs = self.kmac.regs_mut();
        Self::poll_status(regs, |status| status.sha3_absorb()).expect("absorb");
        let fifo = regs.msg_fifo().at(0);
        let ptr = fifo.ptr() as *mut u8;
        for byte in data.iter() {
            Self::poll_status(regs, |status| !status.fifo_full()).expect("fifo_full");
            // SAFETY: ptr is valid for byte writes.
            unsafe {
                ptr.write_volatile(*byte);
            }
        }
    }

    pub fn squeeze<const N: usize>(&mut self) -> Result<[u32; N], ()> {
        let regs = self.kmac.regs_mut();
        Self::issue_command(regs, kmac::enums::Cmd::Process);
        Self::poll_status(regs, |status| status.sha3_squeeze()).expect("sqeeze0");
        let rate = Self::keccak_rate(regs.cfg_shadowed().read().kstrength()).expect("rate");
        let mut result = [0u32; N];
        let mut i = 0;
        while i < N {
            Self::poll_status(regs, |status| status.sha3_squeeze()).expect("sqeeze1");

            let mut j = 0;
            while i < N && j < rate {
                let v0 = regs.state().at(Self::STATE_SHARE_0 + j).read();
                let v1 = regs.state().at(Self::STATE_SHARE_1 + j).read();
                result[i] = v0 ^ v1;
                i += 1;
                j += 1;
            }
            if j == Self::STATE_WORDS {
                Self::issue_command(regs, kmac::enums::Cmd::Run);
            }
        }
        Self::poll_status(regs, |status| status.sha3_squeeze()).expect("squeeze2");
        Self::issue_command(regs, kmac::enums::Cmd::Done);
        Ok(result)
    }

}
