use hmac;
use base::println;

pub struct HmacKey {
    pub key: [u32; 8],
}

pub struct Hmac {
    hmac: hmac::Hmac,
}

impl Hmac {
    pub fn new() -> Self {
        Hmac {
            hmac: unsafe { hmac::Hmac::new() },
        }
    }

    pub fn configure(&mut self, digest_size: hmac::enums::DigestSize) {
//        key: Option<&HmacKey>) {
        let regs = self.hmac.regs_mut();

        // Clear the configuration, stopping any operation in progress.
        regs.cfg().write(|_| 0.into());
        regs.intr_state().write(|_| u32::MAX.into());

        regs.cfg().modify(|val|
            val.digest_swap(true)
               .endian_swap(false)
               .sha_en(true)
               .hmac_en(false)
               .digest_size(|_| digest_size)
               .key_length(|x| x.key_none())
        );

    }

    pub fn start(&mut self) {
        let regs = self.hmac.regs_mut();
        // Note: `hash_start_clear` writes a 1 bit.  The opentitan documentation
        // says this bit is r0w1c.
        regs.cmd().write(|val| val.hash_start_clear());
    }

    pub fn update(&mut self, data: &[u8]) {
        let regs = self.hmac.regs_mut();
        let fifo = regs.msg_fifo().at(0);
        let ptr = fifo.ptr() as *mut u8;
        for byte in data.iter() {
            // SAFETY: ptr is valid for byte writes.
            unsafe {
                ptr.write_volatile(*byte);
            }
        }
    }

    #[inline(always)]
    pub fn process(&mut self) {
        let regs = self.hmac.regs_mut();
        regs.cmd().write(|val| val.hash_process_clear());
    }

    #[inline(always)]
    pub fn wait_for_done(&mut self) {
        let regs = self.hmac.regs_mut();
        while !regs.status().read().hmac_idle() {
            // Wait until done.
        }
        if !regs.intr_state().read().hmac_done() {
            println!("fatal hmac err");
        }
        regs.intr_state().write(|val| val.hmac_done_clear());
    }

    pub fn digest<const N: usize>(&self) -> [u32; N] {
        let regs = self.hmac.regs();
        let result = regs.digest().at(0).ptr();
        core::array::from_fn(|i| unsafe {
            // SAFETY: result is valid for N < 16
            core::ptr::read_volatile(result.wrapping_add(i))
        })
    }
}
