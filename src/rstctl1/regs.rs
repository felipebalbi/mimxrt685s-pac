#[doc = "peripheral reset control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl0(pub u32);
impl Prstctl0 {
    #[doc = "FLEXCOMM0 reset control"]
    #[inline(always)]
    pub const fn flexcomm0_rst(&self) -> super::vals::Flexcomm0Rst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Flexcomm0Rst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 reset control"]
    #[inline(always)]
    pub fn set_flexcomm0_rst(&mut self, val: super::vals::Flexcomm0Rst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXCOMM1 reset control"]
    #[inline(always)]
    pub const fn flexcomm1_rst(&self) -> super::vals::Flexcomm1Rst {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Flexcomm1Rst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 reset control"]
    #[inline(always)]
    pub fn set_flexcomm1_rst(&mut self, val: super::vals::Flexcomm1Rst) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXCOMM2 reset control"]
    #[inline(always)]
    pub const fn flexcomm2_rst(&self) -> super::vals::Flexcomm2Rst {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Flexcomm2Rst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 reset control"]
    #[inline(always)]
    pub fn set_flexcomm2_rst(&mut self, val: super::vals::Flexcomm2Rst) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXCOMM3 reset control"]
    #[inline(always)]
    pub const fn flexcomm3_rst(&self) -> super::vals::Flexcomm3Rst {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Flexcomm3Rst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 reset control"]
    #[inline(always)]
    pub fn set_flexcomm3_rst(&mut self, val: super::vals::Flexcomm3Rst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCOMM4 reset control"]
    #[inline(always)]
    pub const fn flexcomm4_rst(&self) -> super::vals::Flexcomm4Rst {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Flexcomm4Rst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 reset control"]
    #[inline(always)]
    pub fn set_flexcomm4_rst(&mut self, val: super::vals::Flexcomm4Rst) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXCOMM5 reset control"]
    #[inline(always)]
    pub const fn flexcomm5_rst(&self) -> super::vals::Flexcomm5Rst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Flexcomm5Rst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 reset control"]
    #[inline(always)]
    pub fn set_flexcomm5_rst(&mut self, val: super::vals::Flexcomm5Rst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FLEXCOMM6 reset control"]
    #[inline(always)]
    pub const fn flexcomm6_rst(&self) -> super::vals::Flexcomm6Rst {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Flexcomm6Rst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 reset control"]
    #[inline(always)]
    pub fn set_flexcomm6_rst(&mut self, val: super::vals::Flexcomm6Rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FLEXCOMM7 reset control"]
    #[inline(always)]
    pub const fn flexcomm7_rst(&self) -> super::vals::Flexcomm7Rst {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Flexcomm7Rst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 reset control"]
    #[inline(always)]
    pub fn set_flexcomm7_rst(&mut self, val: super::vals::Flexcomm7Rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "FLEXCOMM14 SPI reset control"]
    #[inline(always)]
    pub const fn flexcomm14_spi_rst(&self) -> super::vals::Flexcomm14SpiRst {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Flexcomm14SpiRst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 SPI reset control"]
    #[inline(always)]
    pub fn set_flexcomm14_spi_rst(&mut self, val: super::vals::Flexcomm14SpiRst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "FLEXCOMM15 I2C reset control"]
    #[inline(always)]
    pub const fn flexcomm15_i2c_rst(&self) -> super::vals::Flexcomm15I2cRst {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Flexcomm15I2cRst::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM15 I2C reset control"]
    #[inline(always)]
    pub fn set_flexcomm15_i2c_rst(&mut self, val: super::vals::Flexcomm15I2cRst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMIC0 reset control"]
    #[inline(always)]
    pub const fn dmic0_rst(&self) -> super::vals::Dmic0Rst {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmic0Rst::from_bits(val as u8)
    }
    #[doc = "DMIC0 reset control"]
    #[inline(always)]
    pub fn set_dmic0_rst(&mut self, val: super::vals::Dmic0Rst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "osevent timer reset control"]
    #[inline(always)]
    pub const fn osevt_timer_rst(&self) -> super::vals::OsevtTimerRst {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OsevtTimerRst::from_bits(val as u8)
    }
    #[doc = "osevent timer reset control"]
    #[inline(always)]
    pub fn set_osevt_timer_rst(&mut self, val: super::vals::OsevtTimerRst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Prstctl0 {
    #[inline(always)]
    fn default() -> Prstctl0 {
        Prstctl0(0)
    }
}
#[doc = "peripheral reset clear register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl0Clr(pub u32);
impl Prstctl0Clr {
    #[doc = "FLEXCOMM0 reset clear"]
    #[inline(always)]
    pub const fn flexcomm0_rst_clr(&self) -> super::vals::Flexcomm0RstClr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Flexcomm0RstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 reset clear"]
    #[inline(always)]
    pub fn set_flexcomm0_rst_clr(&mut self, val: super::vals::Flexcomm0RstClr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXCOMM1 reset clear"]
    #[inline(always)]
    pub const fn flexcomm1_rst_clr(&self) -> super::vals::Flexcomm1RstClr {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Flexcomm1RstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 reset clear"]
    #[inline(always)]
    pub fn set_flexcomm1_rst_clr(&mut self, val: super::vals::Flexcomm1RstClr) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXCOMM2 reset clear"]
    #[inline(always)]
    pub const fn flexcomm2_rst_clr(&self) -> super::vals::Flexcomm2RstClr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Flexcomm2RstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 reset clear"]
    #[inline(always)]
    pub fn set_flexcomm2_rst_clr(&mut self, val: super::vals::Flexcomm2RstClr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXCOMM3 reset clear"]
    #[inline(always)]
    pub const fn flexcomm3_rst_clr(&self) -> super::vals::Flexcomm3RstClr {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Flexcomm3RstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 reset clear"]
    #[inline(always)]
    pub fn set_flexcomm3_rst_clr(&mut self, val: super::vals::Flexcomm3RstClr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCOMM4 reset clear"]
    #[inline(always)]
    pub const fn flexcomm4_rst_clr(&self) -> super::vals::Flexcomm4RstClr {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Flexcomm4RstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 reset clear"]
    #[inline(always)]
    pub fn set_flexcomm4_rst_clr(&mut self, val: super::vals::Flexcomm4RstClr) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXCOMM5 reset clear"]
    #[inline(always)]
    pub const fn flexcomm5_rst_clr(&self) -> super::vals::Flexcomm5RstClr {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Flexcomm5RstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 reset clear"]
    #[inline(always)]
    pub fn set_flexcomm5_rst_clr(&mut self, val: super::vals::Flexcomm5RstClr) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FLEXCOMM6 reset clear"]
    #[inline(always)]
    pub const fn flexcomm6_rst_clr(&self) -> super::vals::Flexcomm6RstClr {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Flexcomm6RstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 reset clear"]
    #[inline(always)]
    pub fn set_flexcomm6_rst_clr(&mut self, val: super::vals::Flexcomm6RstClr) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FLEXCOMM7 reset clear"]
    #[inline(always)]
    pub const fn flexcomm7_rst_clr(&self) -> super::vals::Flexcomm7RstClr {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Flexcomm7RstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 reset clear"]
    #[inline(always)]
    pub fn set_flexcomm7_rst_clr(&mut self, val: super::vals::Flexcomm7RstClr) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "FLEXCOMM14 SPI reset clear"]
    #[inline(always)]
    pub const fn flexcomm14_spi_rst_clr(&self) -> super::vals::Flexcomm14SpiRstClr {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Flexcomm14SpiRstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 SPI reset clear"]
    #[inline(always)]
    pub fn set_flexcomm14_spi_rst_clr(&mut self, val: super::vals::Flexcomm14SpiRstClr) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "FLEXCOMM15 I2C reset clear"]
    #[inline(always)]
    pub const fn flexcomm15_i2c_rst_clr(&self) -> super::vals::Flexcomm15I2cRstClr {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Flexcomm15I2cRstClr::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM15 I2C reset clear"]
    #[inline(always)]
    pub fn set_flexcomm15_i2c_rst_clr(&mut self, val: super::vals::Flexcomm15I2cRstClr) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMIC0 reset clear"]
    #[inline(always)]
    pub const fn dmic0_rst_clr(&self) -> super::vals::Dmic0RstClr {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmic0RstClr::from_bits(val as u8)
    }
    #[doc = "DMIC0 reset clear"]
    #[inline(always)]
    pub fn set_dmic0_rst_clr(&mut self, val: super::vals::Dmic0RstClr) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "osevent timer reset clear"]
    #[inline(always)]
    pub const fn osevt_timer_rst_clr(&self) -> super::vals::OsevtTimerRstClr {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OsevtTimerRstClr::from_bits(val as u8)
    }
    #[doc = "osevent timer reset clear"]
    #[inline(always)]
    pub fn set_osevt_timer_rst_clr(&mut self, val: super::vals::OsevtTimerRstClr) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Prstctl0Clr {
    #[inline(always)]
    fn default() -> Prstctl0Clr {
        Prstctl0Clr(0)
    }
}
#[doc = "peripheral reset set register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl0Set(pub u32);
impl Prstctl0Set {
    #[doc = "FLEXCOMM0 reset set"]
    #[inline(always)]
    pub const fn flexcomm0_rst_set(&self) -> super::vals::Flexcomm0RstSet {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Flexcomm0RstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 reset set"]
    #[inline(always)]
    pub fn set_flexcomm0_rst_set(&mut self, val: super::vals::Flexcomm0RstSet) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXCOMM1 reset set"]
    #[inline(always)]
    pub const fn flexcomm1_rst_set(&self) -> super::vals::Flexcomm1RstSet {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Flexcomm1RstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 reset set"]
    #[inline(always)]
    pub fn set_flexcomm1_rst_set(&mut self, val: super::vals::Flexcomm1RstSet) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXCOMM2 reset set"]
    #[inline(always)]
    pub const fn flexcomm2_rst_set(&self) -> super::vals::Flexcomm2RstSet {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Flexcomm2RstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 reset set"]
    #[inline(always)]
    pub fn set_flexcomm2_rst_set(&mut self, val: super::vals::Flexcomm2RstSet) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXCOMM3 reset set"]
    #[inline(always)]
    pub const fn flexcomm3_rst_set(&self) -> super::vals::Flexcomm3RstSet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Flexcomm3RstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 reset set"]
    #[inline(always)]
    pub fn set_flexcomm3_rst_set(&mut self, val: super::vals::Flexcomm3RstSet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCOMM4 reset set"]
    #[inline(always)]
    pub const fn flexcomm4_rst_set(&self) -> super::vals::Flexcomm4RstSet {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Flexcomm4RstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 reset set"]
    #[inline(always)]
    pub fn set_flexcomm4_rst_set(&mut self, val: super::vals::Flexcomm4RstSet) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXCOMM5 reset set"]
    #[inline(always)]
    pub const fn flexcomm5_rst_set(&self) -> super::vals::Flexcomm5RstSet {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Flexcomm5RstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 reset set"]
    #[inline(always)]
    pub fn set_flexcomm5_rst_set(&mut self, val: super::vals::Flexcomm5RstSet) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FLEXCOMM6 reset set"]
    #[inline(always)]
    pub const fn flexcomm6_rst_set(&self) -> super::vals::Flexcomm6RstSet {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Flexcomm6RstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 reset set"]
    #[inline(always)]
    pub fn set_flexcomm6_rst_set(&mut self, val: super::vals::Flexcomm6RstSet) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FLEXCOMM7 reset set"]
    #[inline(always)]
    pub const fn flexcomm7_rst_set(&self) -> super::vals::Flexcomm7RstSet {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Flexcomm7RstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 reset set"]
    #[inline(always)]
    pub fn set_flexcomm7_rst_set(&mut self, val: super::vals::Flexcomm7RstSet) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "FLEXCOMM14 SPI reset set"]
    #[inline(always)]
    pub const fn flexcomm14_spi_rst_set(&self) -> super::vals::Flexcomm14SpiRstSet {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Flexcomm14SpiRstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 SPI reset set"]
    #[inline(always)]
    pub fn set_flexcomm14_spi_rst_set(&mut self, val: super::vals::Flexcomm14SpiRstSet) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "FLEXCOMM15 I2C reset set"]
    #[inline(always)]
    pub const fn flexcomm15_i2c_rst_set(&self) -> super::vals::Flexcomm15I2cRstSet {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Flexcomm15I2cRstSet::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM15 I2C reset set"]
    #[inline(always)]
    pub fn set_flexcomm15_i2c_rst_set(&mut self, val: super::vals::Flexcomm15I2cRstSet) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMIC0 reset set"]
    #[inline(always)]
    pub const fn dmic0_rst_set(&self) -> super::vals::Dmic0RstSet {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmic0RstSet::from_bits(val as u8)
    }
    #[doc = "DMIC0 reset set"]
    #[inline(always)]
    pub fn set_dmic0_rst_set(&mut self, val: super::vals::Dmic0RstSet) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "osevent timer reset set"]
    #[inline(always)]
    pub const fn osevt_timer_rst_set(&self) -> super::vals::OsevtTimerRstSet {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::OsevtTimerRstSet::from_bits(val as u8)
    }
    #[doc = "osevent timer reset set"]
    #[inline(always)]
    pub fn set_osevt_timer_rst_set(&mut self, val: super::vals::OsevtTimerRstSet) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Prstctl0Set {
    #[inline(always)]
    fn default() -> Prstctl0Set {
        Prstctl0Set(0)
    }
}
#[doc = "peripheral reset control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl1(pub u32);
impl Prstctl1 {
    #[doc = "HSGPIO0 reset control"]
    #[inline(always)]
    pub const fn hsgpio0_rst(&self) -> super::vals::Hsgpio0Rst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hsgpio0Rst::from_bits(val as u8)
    }
    #[doc = "HSGPIO0 reset control"]
    #[inline(always)]
    pub fn set_hsgpio0_rst(&mut self, val: super::vals::Hsgpio0Rst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "HSGPIO1 reset control"]
    #[inline(always)]
    pub const fn hsgpio1_rst(&self) -> super::vals::Hsgpio1Rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hsgpio1Rst::from_bits(val as u8)
    }
    #[doc = "HSGPIO1 reset control"]
    #[inline(always)]
    pub fn set_hsgpio1_rst(&mut self, val: super::vals::Hsgpio1Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSGPIO2 reset control"]
    #[inline(always)]
    pub const fn hsgpio2_rst(&self) -> super::vals::Hsgpio2Rst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Hsgpio2Rst::from_bits(val as u8)
    }
    #[doc = "HSGPIO2 reset control"]
    #[inline(always)]
    pub fn set_hsgpio2_rst(&mut self, val: super::vals::Hsgpio2Rst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "HSGPIO3 reset control"]
    #[inline(always)]
    pub const fn hsgpio3_rst(&self) -> super::vals::Hsgpio3Rst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hsgpio3Rst::from_bits(val as u8)
    }
    #[doc = "HSGPIO3 reset control"]
    #[inline(always)]
    pub fn set_hsgpio3_rst(&mut self, val: super::vals::Hsgpio3Rst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "HSGPIO4 reset control"]
    #[inline(always)]
    pub const fn hsgpio4_rst(&self) -> super::vals::Hsgpio4Rst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hsgpio4Rst::from_bits(val as u8)
    }
    #[doc = "HSGPIO4 reset control"]
    #[inline(always)]
    pub fn set_hsgpio4_rst(&mut self, val: super::vals::Hsgpio4Rst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HSGPIO5 reset control"]
    #[inline(always)]
    pub const fn hsgpio5_rst(&self) -> super::vals::Hsgpio5Rst {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hsgpio5Rst::from_bits(val as u8)
    }
    #[doc = "HSGPIO5 reset control"]
    #[inline(always)]
    pub fn set_hsgpio5_rst(&mut self, val: super::vals::Hsgpio5Rst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "HSGPIO6 reset control"]
    #[inline(always)]
    pub const fn hsgpio6_rst(&self) -> super::vals::Hsgpio6Rst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Hsgpio6Rst::from_bits(val as u8)
    }
    #[doc = "HSGPIO6 reset control"]
    #[inline(always)]
    pub fn set_hsgpio6_rst(&mut self, val: super::vals::Hsgpio6Rst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "HSGPIO7 reset control"]
    #[inline(always)]
    pub const fn hsgpio7_rst(&self) -> super::vals::Hsgpio7Rst {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Hsgpio7Rst::from_bits(val as u8)
    }
    #[doc = "HSGPIO7 reset control"]
    #[inline(always)]
    pub fn set_hsgpio7_rst(&mut self, val: super::vals::Hsgpio7Rst) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CRC reset control"]
    #[inline(always)]
    pub const fn crc_rst(&self) -> super::vals::CrcRst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CrcRst::from_bits(val as u8)
    }
    #[doc = "CRC reset control"]
    #[inline(always)]
    pub fn set_crc_rst(&mut self, val: super::vals::CrcRst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC0 reset control"]
    #[inline(always)]
    pub const fn dmac0_rst(&self) -> super::vals::Dmac0Rst {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0Rst::from_bits(val as u8)
    }
    #[doc = "DMAC0 reset control"]
    #[inline(always)]
    pub fn set_dmac0_rst(&mut self, val: super::vals::Dmac0Rst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC1 reset control"]
    #[inline(always)]
    pub const fn dmac1_rst(&self) -> super::vals::Dmac1Rst {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1Rst::from_bits(val as u8)
    }
    #[doc = "DMAC1 reset control"]
    #[inline(always)]
    pub fn set_dmac1_rst(&mut self, val: super::vals::Dmac1Rst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "MU reset control"]
    #[inline(always)]
    pub const fn mu_rst(&self) -> super::vals::MuRst {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MuRst::from_bits(val as u8)
    }
    #[doc = "MU reset control"]
    #[inline(always)]
    pub fn set_mu_rst(&mut self, val: super::vals::MuRst) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SEMA reset control"]
    #[inline(always)]
    pub const fn sema_rst(&self) -> super::vals::SemaRst {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SemaRst::from_bits(val as u8)
    }
    #[doc = "SEMA reset control"]
    #[inline(always)]
    pub fn set_sema_rst(&mut self, val: super::vals::SemaRst) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "FREQME reset control"]
    #[inline(always)]
    pub const fn freqme_rst(&self) -> super::vals::FreqmeRst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmeRst::from_bits(val as u8)
    }
    #[doc = "FREQME reset control"]
    #[inline(always)]
    pub fn set_freqme_rst(&mut self, val: super::vals::FreqmeRst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Prstctl1 {
    #[inline(always)]
    fn default() -> Prstctl1 {
        Prstctl1(0)
    }
}
#[doc = "peripheral reset clear register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl1Clr(pub u32);
impl Prstctl1Clr {
    #[doc = "HSGPIO0 reset clear"]
    #[inline(always)]
    pub const fn hsgpio0_rst_clr(&self) -> super::vals::Hsgpio0RstClr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hsgpio0RstClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO0 reset clear"]
    #[inline(always)]
    pub fn set_hsgpio0_rst_clr(&mut self, val: super::vals::Hsgpio0RstClr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "HSGPIO1 reset clear"]
    #[inline(always)]
    pub const fn hsgpio1_rst_clr(&self) -> super::vals::Hsgpio1RstClr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hsgpio1RstClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO1 reset clear"]
    #[inline(always)]
    pub fn set_hsgpio1_rst_clr(&mut self, val: super::vals::Hsgpio1RstClr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSGPIO2 reset clear"]
    #[inline(always)]
    pub const fn hsgpio2_rst_clr(&self) -> super::vals::Hsgpio2RstClr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Hsgpio2RstClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO2 reset clear"]
    #[inline(always)]
    pub fn set_hsgpio2_rst_clr(&mut self, val: super::vals::Hsgpio2RstClr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "HSGPIO3 reset clear"]
    #[inline(always)]
    pub const fn hsgpio3_rst_clr(&self) -> super::vals::Hsgpio3RstClr {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hsgpio3RstClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO3 reset clear"]
    #[inline(always)]
    pub fn set_hsgpio3_rst_clr(&mut self, val: super::vals::Hsgpio3RstClr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "HSGPIO4 reset clear"]
    #[inline(always)]
    pub const fn hsgpio4_rst_clr(&self) -> super::vals::Hsgpio4RstClr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hsgpio4RstClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO4 reset clear"]
    #[inline(always)]
    pub fn set_hsgpio4_rst_clr(&mut self, val: super::vals::Hsgpio4RstClr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HSGPIO5 reset clear"]
    #[inline(always)]
    pub const fn hsgpio5_rst_clr(&self) -> super::vals::Hsgpio5RstClr {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hsgpio5RstClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO5 reset clear"]
    #[inline(always)]
    pub fn set_hsgpio5_rst_clr(&mut self, val: super::vals::Hsgpio5RstClr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "HSGPIO6 reset clear"]
    #[inline(always)]
    pub const fn hsgpio6_rst_clr(&self) -> super::vals::Hsgpio6RstClr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Hsgpio6RstClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO6 reset clear"]
    #[inline(always)]
    pub fn set_hsgpio6_rst_clr(&mut self, val: super::vals::Hsgpio6RstClr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "HSGPIO7 reset clear"]
    #[inline(always)]
    pub const fn hsgpio7_rst_clr(&self) -> super::vals::Hsgpio7RstClr {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Hsgpio7RstClr::from_bits(val as u8)
    }
    #[doc = "HSGPIO7 reset clear"]
    #[inline(always)]
    pub fn set_hsgpio7_rst_clr(&mut self, val: super::vals::Hsgpio7RstClr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CRC reset clear"]
    #[inline(always)]
    pub const fn crc_rst_clr(&self) -> super::vals::CrcRstClr {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CrcRstClr::from_bits(val as u8)
    }
    #[doc = "CRC reset clear"]
    #[inline(always)]
    pub fn set_crc_rst_clr(&mut self, val: super::vals::CrcRstClr) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC0 reset clear"]
    #[inline(always)]
    pub const fn dmac0_rst_clr(&self) -> super::vals::Dmac0RstClr {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0RstClr::from_bits(val as u8)
    }
    #[doc = "DMAC0 reset clear"]
    #[inline(always)]
    pub fn set_dmac0_rst_clr(&mut self, val: super::vals::Dmac0RstClr) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC1 reset clear"]
    #[inline(always)]
    pub const fn dmac1_rst_clr(&self) -> super::vals::Dmac1RstClr {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1RstClr::from_bits(val as u8)
    }
    #[doc = "DMAC1 reset clear"]
    #[inline(always)]
    pub fn set_dmac1_rst_clr(&mut self, val: super::vals::Dmac1RstClr) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "MU reset clear"]
    #[inline(always)]
    pub const fn mu_rst_clr(&self) -> super::vals::MuRstClr {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MuRstClr::from_bits(val as u8)
    }
    #[doc = "MU reset clear"]
    #[inline(always)]
    pub fn set_mu_rst_clr(&mut self, val: super::vals::MuRstClr) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SEMA reset clear"]
    #[inline(always)]
    pub const fn sema_rst_clr(&self) -> super::vals::SemaRstClr {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SemaRstClr::from_bits(val as u8)
    }
    #[doc = "SEMA reset clear"]
    #[inline(always)]
    pub fn set_sema_rst_clr(&mut self, val: super::vals::SemaRstClr) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "FREQME reset clear"]
    #[inline(always)]
    pub const fn freqme_rst_clr(&self) -> super::vals::FreqmeRstClr {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmeRstClr::from_bits(val as u8)
    }
    #[doc = "FREQME reset clear"]
    #[inline(always)]
    pub fn set_freqme_rst_clr(&mut self, val: super::vals::FreqmeRstClr) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Prstctl1Clr {
    #[inline(always)]
    fn default() -> Prstctl1Clr {
        Prstctl1Clr(0)
    }
}
#[doc = "peripheral reset set register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl1Set(pub u32);
impl Prstctl1Set {
    #[doc = "HSGPIO0 reset set"]
    #[inline(always)]
    pub const fn hsgpio0_rst_set(&self) -> super::vals::Hsgpio0RstSet {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hsgpio0RstSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO0 reset set"]
    #[inline(always)]
    pub fn set_hsgpio0_rst_set(&mut self, val: super::vals::Hsgpio0RstSet) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "HSGPIO1 reset set"]
    #[inline(always)]
    pub const fn hsgpio1_rst_set(&self) -> super::vals::Hsgpio1RstSet {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hsgpio1RstSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO1 reset set"]
    #[inline(always)]
    pub fn set_hsgpio1_rst_set(&mut self, val: super::vals::Hsgpio1RstSet) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "HSGPIO2 reset set"]
    #[inline(always)]
    pub const fn hsgpio2_rst_set(&self) -> super::vals::Hsgpio2RstSet {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Hsgpio2RstSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO2 reset set"]
    #[inline(always)]
    pub fn set_hsgpio2_rst_set(&mut self, val: super::vals::Hsgpio2RstSet) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "HSGPIO3 reset set"]
    #[inline(always)]
    pub const fn hsgpio3_rst_set(&self) -> super::vals::Hsgpio3RstSet {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hsgpio3RstSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO3 reset set"]
    #[inline(always)]
    pub fn set_hsgpio3_rst_set(&mut self, val: super::vals::Hsgpio3RstSet) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "HSGPIO4 reset set"]
    #[inline(always)]
    pub const fn hsgpio4_rst_set(&self) -> super::vals::Hsgpio4RstSet {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hsgpio4RstSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO4 reset set"]
    #[inline(always)]
    pub fn set_hsgpio4_rst_set(&mut self, val: super::vals::Hsgpio4RstSet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HSGPIO5 reset set"]
    #[inline(always)]
    pub const fn hsgpio5_rst_set(&self) -> super::vals::Hsgpio5RstSet {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hsgpio5RstSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO5 reset set"]
    #[inline(always)]
    pub fn set_hsgpio5_rst_set(&mut self, val: super::vals::Hsgpio5RstSet) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "HSGPIO6 reset set"]
    #[inline(always)]
    pub const fn hsgpio6_rst_set(&self) -> super::vals::Hsgpio6RstSet {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Hsgpio6RstSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO6 reset set"]
    #[inline(always)]
    pub fn set_hsgpio6_rst_set(&mut self, val: super::vals::Hsgpio6RstSet) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "HSGPIO7 reset set"]
    #[inline(always)]
    pub const fn hsgpio7_rst_set(&self) -> super::vals::Hsgpio7RstSet {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Hsgpio7RstSet::from_bits(val as u8)
    }
    #[doc = "HSGPIO7 reset set"]
    #[inline(always)]
    pub fn set_hsgpio7_rst_set(&mut self, val: super::vals::Hsgpio7RstSet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "CRC reset set"]
    #[inline(always)]
    pub const fn crc_rst_set(&self) -> super::vals::CrcRstSet {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CrcRstSet::from_bits(val as u8)
    }
    #[doc = "CRC reset set"]
    #[inline(always)]
    pub fn set_crc_rst_set(&mut self, val: super::vals::CrcRstSet) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC0 reset set"]
    #[inline(always)]
    pub const fn dmac0_rst_set(&self) -> super::vals::Dmac0RstSet {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0RstSet::from_bits(val as u8)
    }
    #[doc = "DMAC0 reset set"]
    #[inline(always)]
    pub fn set_dmac0_rst_set(&mut self, val: super::vals::Dmac0RstSet) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC1 reset set"]
    #[inline(always)]
    pub const fn dmac1_rst_set(&self) -> super::vals::Dmac1RstSet {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1RstSet::from_bits(val as u8)
    }
    #[doc = "DMAC1 reset set"]
    #[inline(always)]
    pub fn set_dmac1_rst_set(&mut self, val: super::vals::Dmac1RstSet) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "MU reset set"]
    #[inline(always)]
    pub const fn mu_rst_set(&self) -> super::vals::MuRstSet {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::MuRstSet::from_bits(val as u8)
    }
    #[doc = "MU reset set"]
    #[inline(always)]
    pub fn set_mu_rst_set(&mut self, val: super::vals::MuRstSet) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SEMA reset set"]
    #[inline(always)]
    pub const fn sema_rst_set(&self) -> super::vals::SemaRstSet {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SemaRstSet::from_bits(val as u8)
    }
    #[doc = "SEMA reset set"]
    #[inline(always)]
    pub fn set_sema_rst_set(&mut self, val: super::vals::SemaRstSet) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "FREQME reset set"]
    #[inline(always)]
    pub const fn freqme_rst_set(&self) -> super::vals::FreqmeRstSet {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmeRstSet::from_bits(val as u8)
    }
    #[doc = "FREQME reset set"]
    #[inline(always)]
    pub fn set_freqme_rst_set(&mut self, val: super::vals::FreqmeRstSet) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Prstctl1Set {
    #[inline(always)]
    fn default() -> Prstctl1Set {
        Prstctl1Set(0)
    }
}
#[doc = "peripheral reset control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl2(pub u32);
impl Prstctl2 {
    #[doc = "CT32BIT0 reset control"]
    #[inline(always)]
    pub const fn ct32bit0_rst(&self) -> super::vals::Ct32bit0Rst {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ct32bit0Rst::from_bits(val as u8)
    }
    #[doc = "CT32BIT0 reset control"]
    #[inline(always)]
    pub fn set_ct32bit0_rst(&mut self, val: super::vals::Ct32bit0Rst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CT32BIT1 reset control"]
    #[inline(always)]
    pub const fn ct32bit1_rst(&self) -> super::vals::Ct32bit1Rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ct32bit1Rst::from_bits(val as u8)
    }
    #[doc = "CT32BIT1 reset control"]
    #[inline(always)]
    pub fn set_ct32bit1_rst(&mut self, val: super::vals::Ct32bit1Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CT32BIT2 reset control"]
    #[inline(always)]
    pub const fn ct32bit2_rst(&self) -> super::vals::Ct32bit2Rst {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ct32bit2Rst::from_bits(val as u8)
    }
    #[doc = "CT32BIT2 reset control"]
    #[inline(always)]
    pub fn set_ct32bit2_rst(&mut self, val: super::vals::Ct32bit2Rst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CT32BIT3 reset control"]
    #[inline(always)]
    pub const fn ct32bit3_rst(&self) -> super::vals::Ct32bit3Rst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ct32bit3Rst::from_bits(val as u8)
    }
    #[doc = "CT32BIT3 reset control"]
    #[inline(always)]
    pub fn set_ct32bit3_rst(&mut self, val: super::vals::Ct32bit3Rst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "CT32BIT4 reset control"]
    #[inline(always)]
    pub const fn ct32bit4_rst(&self) -> super::vals::Ct32bit4Rst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ct32bit4Rst::from_bits(val as u8)
    }
    #[doc = "CT32BIT4 reset control"]
    #[inline(always)]
    pub fn set_ct32bit4_rst(&mut self, val: super::vals::Ct32bit4Rst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "MRT0 reset control"]
    #[inline(always)]
    pub const fn mrt0_rst(&self) -> super::vals::Mrt0Rst {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mrt0Rst::from_bits(val as u8)
    }
    #[doc = "MRT0 reset control"]
    #[inline(always)]
    pub fn set_mrt0_rst(&mut self, val: super::vals::Mrt0Rst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT1 reset control"]
    #[inline(always)]
    pub const fn wwdt1_rst(&self) -> super::vals::Wwdt1Rst {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wwdt1Rst::from_bits(val as u8)
    }
    #[doc = "WWDT1 reset control"]
    #[inline(always)]
    pub fn set_wwdt1_rst(&mut self, val: super::vals::Wwdt1Rst) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "I3C0 reset control"]
    #[inline(always)]
    pub const fn i3c0_rst(&self) -> super::vals::I3c0Rst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::I3c0Rst::from_bits(val as u8)
    }
    #[doc = "I3C0 reset control"]
    #[inline(always)]
    pub fn set_i3c0_rst(&mut self, val: super::vals::I3c0Rst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIOINTCTL reset control"]
    #[inline(always)]
    pub const fn gpiointctl_rst(&self) -> super::vals::GpiointctlRst {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::GpiointctlRst::from_bits(val as u8)
    }
    #[doc = "GPIOINTCTL reset control"]
    #[inline(always)]
    pub fn set_gpiointctl_rst(&mut self, val: super::vals::GpiointctlRst) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "PMC reset control"]
    #[inline(always)]
    pub const fn pimctl_rst(&self) -> super::vals::PimctlRst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PimctlRst::from_bits(val as u8)
    }
    #[doc = "PMC reset control"]
    #[inline(always)]
    pub fn set_pimctl_rst(&mut self, val: super::vals::PimctlRst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Prstctl2 {
    #[inline(always)]
    fn default() -> Prstctl2 {
        Prstctl2(0)
    }
}
#[doc = "peripheral reset clear register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl2Clr(pub u32);
impl Prstctl2Clr {
    #[doc = "CT32BIT0 reset clear"]
    #[inline(always)]
    pub const fn ct32bit0_rst_clr(&self) -> super::vals::Ct32bit0RstClr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ct32bit0RstClr::from_bits(val as u8)
    }
    #[doc = "CT32BIT0 reset clear"]
    #[inline(always)]
    pub fn set_ct32bit0_rst_clr(&mut self, val: super::vals::Ct32bit0RstClr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CT32BIT1 reset clear"]
    #[inline(always)]
    pub const fn ct32bit1_rst_clr(&self) -> super::vals::Ct32bit1RstClr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ct32bit1RstClr::from_bits(val as u8)
    }
    #[doc = "CT32BIT1 reset clear"]
    #[inline(always)]
    pub fn set_ct32bit1_rst_clr(&mut self, val: super::vals::Ct32bit1RstClr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CT32BIT2 reset clear"]
    #[inline(always)]
    pub const fn ct32bit2_rst_clr(&self) -> super::vals::Ct32bit2RstClr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ct32bit2RstClr::from_bits(val as u8)
    }
    #[doc = "CT32BIT2 reset clear"]
    #[inline(always)]
    pub fn set_ct32bit2_rst_clr(&mut self, val: super::vals::Ct32bit2RstClr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CT32BIT3 reset clear"]
    #[inline(always)]
    pub const fn ct32bit3_rst_clr(&self) -> super::vals::Ct32bit3RstClr {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ct32bit3RstClr::from_bits(val as u8)
    }
    #[doc = "CT32BIT3 reset clear"]
    #[inline(always)]
    pub fn set_ct32bit3_rst_clr(&mut self, val: super::vals::Ct32bit3RstClr) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "CT32BIT4 reset clear"]
    #[inline(always)]
    pub const fn ct32bit4_rst_clr(&self) -> super::vals::Ct32bit4RstClr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ct32bit4RstClr::from_bits(val as u8)
    }
    #[doc = "CT32BIT4 reset clear"]
    #[inline(always)]
    pub fn set_ct32bit4_rst_clr(&mut self, val: super::vals::Ct32bit4RstClr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "MRT0 reset clear"]
    #[inline(always)]
    pub const fn mrt0_rst_clr(&self) -> super::vals::Mrt0RstClr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mrt0RstClr::from_bits(val as u8)
    }
    #[doc = "MRT0 reset clear"]
    #[inline(always)]
    pub fn set_mrt0_rst_clr(&mut self, val: super::vals::Mrt0RstClr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT1 reset clear"]
    #[inline(always)]
    pub const fn wwdt1_rst_clr(&self) -> super::vals::Wwdt1RstClr {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wwdt1RstClr::from_bits(val as u8)
    }
    #[doc = "WWDT1 reset clear"]
    #[inline(always)]
    pub fn set_wwdt1_rst_clr(&mut self, val: super::vals::Wwdt1RstClr) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "I3C0 reset clear"]
    #[inline(always)]
    pub const fn i3c0_rst_clr(&self) -> super::vals::I3c0RstClr {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::I3c0RstClr::from_bits(val as u8)
    }
    #[doc = "I3C0 reset clear"]
    #[inline(always)]
    pub fn set_i3c0_rst_clr(&mut self, val: super::vals::I3c0RstClr) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIOINTCTL reset clear"]
    #[inline(always)]
    pub const fn gpiointctl_rst_clr(&self) -> super::vals::GpiointctlRstClr {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::GpiointctlRstClr::from_bits(val as u8)
    }
    #[doc = "GPIOINTCTL reset clear"]
    #[inline(always)]
    pub fn set_gpiointctl_rst_clr(&mut self, val: super::vals::GpiointctlRstClr) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "PMC reset clear"]
    #[inline(always)]
    pub const fn pimctl_rst_clr(&self) -> super::vals::PimctlRstClr {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PimctlRstClr::from_bits(val as u8)
    }
    #[doc = "PMC reset clear"]
    #[inline(always)]
    pub fn set_pimctl_rst_clr(&mut self, val: super::vals::PimctlRstClr) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Prstctl2Clr {
    #[inline(always)]
    fn default() -> Prstctl2Clr {
        Prstctl2Clr(0)
    }
}
#[doc = "peripheral reset set register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prstctl2Set(pub u32);
impl Prstctl2Set {
    #[doc = "CT32BIT0 reset set"]
    #[inline(always)]
    pub const fn ct32bit0_rst_set(&self) -> super::vals::Ct32bit0RstSet {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ct32bit0RstSet::from_bits(val as u8)
    }
    #[doc = "CT32BIT0 reset set"]
    #[inline(always)]
    pub fn set_ct32bit0_rst_set(&mut self, val: super::vals::Ct32bit0RstSet) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CT32BIT1 reset set"]
    #[inline(always)]
    pub const fn ct32bit1_rst_set(&self) -> super::vals::Ct32bit1RstSet {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ct32bit1RstSet::from_bits(val as u8)
    }
    #[doc = "CT32BIT1 reset set"]
    #[inline(always)]
    pub fn set_ct32bit1_rst_set(&mut self, val: super::vals::Ct32bit1RstSet) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CT32BIT2 reset set"]
    #[inline(always)]
    pub const fn ct32bit2_rst_set(&self) -> super::vals::Ct32bit2RstSet {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ct32bit2RstSet::from_bits(val as u8)
    }
    #[doc = "CT32BIT2 reset set"]
    #[inline(always)]
    pub fn set_ct32bit2_rst_set(&mut self, val: super::vals::Ct32bit2RstSet) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CT32BIT3 reset set"]
    #[inline(always)]
    pub const fn ct32bit3_rst_set(&self) -> super::vals::Ct32bit3RstSet {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ct32bit3RstSet::from_bits(val as u8)
    }
    #[doc = "CT32BIT3 reset set"]
    #[inline(always)]
    pub fn set_ct32bit3_rst_set(&mut self, val: super::vals::Ct32bit3RstSet) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "CT32BIT4 reset set"]
    #[inline(always)]
    pub const fn ct32bit4_rst_set(&self) -> super::vals::Ct32bit4RstSet {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ct32bit4RstSet::from_bits(val as u8)
    }
    #[doc = "CT32BIT4 reset set"]
    #[inline(always)]
    pub fn set_ct32bit4_rst_set(&mut self, val: super::vals::Ct32bit4RstSet) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "MRT0 reset set"]
    #[inline(always)]
    pub const fn mrt0_rst_set(&self) -> super::vals::Mrt0RstSet {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mrt0RstSet::from_bits(val as u8)
    }
    #[doc = "MRT0 reset set"]
    #[inline(always)]
    pub fn set_mrt0_rst_set(&mut self, val: super::vals::Mrt0RstSet) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "WWDT1 reset set"]
    #[inline(always)]
    pub const fn wwdt1_rst_set(&self) -> super::vals::Wwdt1RstSet {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wwdt1RstSet::from_bits(val as u8)
    }
    #[doc = "WWDT1 reset set"]
    #[inline(always)]
    pub fn set_wwdt1_rst_set(&mut self, val: super::vals::Wwdt1RstSet) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "I3C0 reset set"]
    #[inline(always)]
    pub const fn i3c0_rst_set(&self) -> super::vals::I3c0RstSet {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::I3c0RstSet::from_bits(val as u8)
    }
    #[doc = "I3C0 reset set"]
    #[inline(always)]
    pub fn set_i3c0_rst_set(&mut self, val: super::vals::I3c0RstSet) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "GPIOINTCTL reset set"]
    #[inline(always)]
    pub const fn gpiointctl_rst_set(&self) -> super::vals::GpiointctlRstSet {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::GpiointctlRstSet::from_bits(val as u8)
    }
    #[doc = "GPIOINTCTL reset set"]
    #[inline(always)]
    pub fn set_gpiointctl_rst_set(&mut self, val: super::vals::GpiointctlRstSet) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "PMC reset set"]
    #[inline(always)]
    pub const fn pimctl_rst_set(&self) -> super::vals::PimctlRstSet {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PimctlRstSet::from_bits(val as u8)
    }
    #[doc = "PMC reset set"]
    #[inline(always)]
    pub fn set_pimctl_rst_set(&mut self, val: super::vals::PimctlRstSet) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Prstctl2Set {
    #[inline(always)]
    fn default() -> Prstctl2Set {
        Prstctl2Set(0)
    }
}
#[doc = "system reset status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysrststat(pub u32);
impl Sysrststat {
    #[doc = "VDD POR Event Detected:"]
    #[inline(always)]
    pub const fn vdd_por(&self) -> super::vals::VddPor {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::VddPor::from_bits(val as u8)
    }
    #[doc = "VDD POR Event Detected:"]
    #[inline(always)]
    pub fn set_vdd_por(&mut self, val: super::vals::VddPor) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PAD RESET Event Detected:"]
    #[inline(always)]
    pub const fn pad_reset(&self) -> super::vals::PadReset {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PadReset::from_bits(val as u8)
    }
    #[doc = "PAD RESET Event Detected:"]
    #[inline(always)]
    pub fn set_pad_reset(&mut self, val: super::vals::PadReset) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "ARM RESET Event Detected:"]
    #[inline(always)]
    pub const fn arm_apd_reset(&self) -> super::vals::ArmApdReset {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ArmApdReset::from_bits(val as u8)
    }
    #[doc = "ARM RESET Event Detected:"]
    #[inline(always)]
    pub fn set_arm_apd_reset(&mut self, val: super::vals::ArmApdReset) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "WDT0 RESET Event Detected:"]
    #[inline(always)]
    pub const fn wdt0_reset(&self) -> super::vals::Wdt0Reset {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Wdt0Reset::from_bits(val as u8)
    }
    #[doc = "WDT0 RESET Event Detected:"]
    #[inline(always)]
    pub fn set_wdt0_reset(&mut self, val: super::vals::Wdt0Reset) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "WDT1 RESET Event Detected:"]
    #[inline(always)]
    pub const fn wdt1_reset(&self) -> super::vals::Wdt1Reset {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wdt1Reset::from_bits(val as u8)
    }
    #[doc = "WDT1 RESET Event Detected:"]
    #[inline(always)]
    pub fn set_wdt1_reset(&mut self, val: super::vals::Wdt1Reset) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Sysrststat {
    #[inline(always)]
    fn default() -> Sysrststat {
        Sysrststat(0)
    }
}
