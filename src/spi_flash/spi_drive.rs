pub trait SpiDrive {
    fn write_after_read(
        &self,
        write_len: u32,
        read_len: u32,
        iobuf: &mut [u8],
    ) -> Result<(), &'static str>;
    fn transfer(&self, iobuf: &mut [u8]) -> Result<(), &'static str>;
}
