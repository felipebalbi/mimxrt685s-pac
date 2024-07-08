#[doc = "CRC mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc = "CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
    #[inline(always)]
    pub const fn crc_poly(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn set_crc_poly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub const fn bit_rvs_wr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn set_bit_rvs_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub const fn cmpl_wr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn set_cmpl_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub const fn bit_rvs_sum(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn set_bit_rvs_sum(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
    #[inline(always)]
    pub const fn cmpl_sum(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
    #[inline(always)]
    pub fn set_cmpl_sum(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Mode {
    #[inline(always)]
    fn default() -> Mode {
        Mode(0)
    }
}
#[doc = "CRC seed register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seed(pub u32);
impl Seed {
    #[doc = "A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    pub const fn crc_seed(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A write access to this register will load CRC seed value to CRC_SUM register with selected bit order and 1's complement pre-processes. A write access to this register will overrule the CRC calculation in progresses."]
    #[inline(always)]
    pub fn set_crc_seed(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Seed {
    #[inline(always)]
    fn default() -> Seed {
        Seed(0)
    }
}
#[doc = "CRC checksum register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sum(pub u32);
impl Sum {
    #[doc = "The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
    #[inline(always)]
    pub const fn crc_sum(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The most recent CRC sum can be read through this register with selected bit order and 1's complement post-processes."]
    #[inline(always)]
    pub fn set_crc_sum(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sum {
    #[inline(always)]
    fn default() -> Sum {
        Sum(0)
    }
}
#[doc = "CRC data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrData(pub u32);
impl WrData {
    #[doc = "Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[inline(always)]
    pub const fn crc_wr_data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data written to this register will be taken to perform CRC calculation with selected bit order and 1's complement pre-process. Any write size 8, 16 or 32-bit are allowed and accept back-to-back transactions."]
    #[inline(always)]
    pub fn set_crc_wr_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for WrData {
    #[inline(always)]
    fn default() -> WrData {
        WrData(0)
    }
}
