use ufmt::{uDisplay, uWrite, Formatter};

pub struct HexEncode<'a>(pub &'a [u8]);

impl uDisplay for HexEncode<'_> {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        const HEX: [u8; 16] = *b"0123456789abcdef";
        for &byte in self.0.iter() {
            f.write_char(HEX[(byte>>4) as usize] as char)?;
            f.write_char(HEX[(byte&15) as usize] as char)?;
        }
        Ok(())
    }
}
