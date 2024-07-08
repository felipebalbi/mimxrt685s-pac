#[doc = "ADC Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "ADC trigger priority control"]
    #[inline(always)]
    pub const fn tprictrl(&self) -> super::vals::Tprictrl {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tprictrl::from_bits(val as u8)
    }
    #[doc = "ADC trigger priority control"]
    #[inline(always)]
    pub fn set_tprictrl(&mut self, val: super::vals::Tprictrl) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Power Configuration Select"]
    #[inline(always)]
    pub const fn pwrsel(&self) -> super::vals::Pwrsel {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Pwrsel::from_bits(val as u8)
    }
    #[doc = "Power Configuration Select"]
    #[inline(always)]
    pub fn set_pwrsel(&mut self, val: super::vals::Pwrsel) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Voltage Reference Selection"]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Refsel::from_bits(val as u8)
    }
    #[doc = "Voltage Reference Selection"]
    #[inline(always)]
    pub fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Power Up Delay"]
    #[inline(always)]
    pub const fn pudly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Power Up Delay"]
    #[inline(always)]
    pub fn set_pudly(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "ADC Analog Pre-Enable"]
    #[inline(always)]
    pub const fn pwren(&self) -> super::vals::Pwren {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Pwren::from_bits(val as u8)
    }
    #[doc = "ADC Analog Pre-Enable"]
    #[inline(always)]
    pub fn set_pwren(&mut self, val: super::vals::Pwren) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh1(pub u32);
impl Cmdh1 {
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh1Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh1Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub fn set_cmpen(&mut self, val: super::vals::Cmdh1Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh1Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh1Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh1Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh1Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh1Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh1Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh1Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh1Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh1Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh1Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh1Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh1Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh1Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh1Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh1Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh1 {
    #[inline(always)]
    fn default() -> Cmdh1 {
        Cmdh1(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh10(pub u32);
impl Cmdh10 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh10Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh10Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh10Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh10Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh10Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh10Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh10Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh10Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh10Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh10Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh10Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh10Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh10Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh10Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh10Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh10 {
    #[inline(always)]
    fn default() -> Cmdh10 {
        Cmdh10(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh11(pub u32);
impl Cmdh11 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh11Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh11Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh11Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh11Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh11Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh11Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh11Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh11Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh11Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh11Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh11Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh11Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh11Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh11Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh11Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh11 {
    #[inline(always)]
    fn default() -> Cmdh11 {
        Cmdh11(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh12(pub u32);
impl Cmdh12 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh12Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh12Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh12Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh12Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh12Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh12Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh12Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh12Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh12Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh12Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh12Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh12Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh12Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh12Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh12Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh12 {
    #[inline(always)]
    fn default() -> Cmdh12 {
        Cmdh12(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh13(pub u32);
impl Cmdh13 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh13Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh13Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh13Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh13Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh13Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh13Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh13Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh13Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh13Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh13Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh13Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh13Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh13Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh13Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh13Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh13 {
    #[inline(always)]
    fn default() -> Cmdh13 {
        Cmdh13(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh14(pub u32);
impl Cmdh14 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh14Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh14Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh14Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh14Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh14Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh14Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh14Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh14Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh14Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh14Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh14Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh14Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh14Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh14Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh14Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh14 {
    #[inline(always)]
    fn default() -> Cmdh14 {
        Cmdh14(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh15(pub u32);
impl Cmdh15 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh15Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh15Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh15Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh15Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh15Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh15Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh15Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh15Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh15Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh15Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh15Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh15Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh15Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh15Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh15Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh15 {
    #[inline(always)]
    fn default() -> Cmdh15 {
        Cmdh15(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh2(pub u32);
impl Cmdh2 {
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh2Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh2Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub fn set_cmpen(&mut self, val: super::vals::Cmdh2Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh2Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh2Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh2Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh2Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh2Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh2Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh2Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh2Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh2Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh2Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh2Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh2Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh2Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh2Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh2Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh2 {
    #[inline(always)]
    fn default() -> Cmdh2 {
        Cmdh2(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh3(pub u32);
impl Cmdh3 {
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh3Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh3Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub fn set_cmpen(&mut self, val: super::vals::Cmdh3Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh3Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh3Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh3Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh3Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh3Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh3Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh3Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh3Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh3Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh3Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh3Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh3Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh3Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh3Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh3Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh3 {
    #[inline(always)]
    fn default() -> Cmdh3 {
        Cmdh3(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh4(pub u32);
impl Cmdh4 {
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub const fn cmpen(&self) -> super::vals::Cmdh4Cmpen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cmdh4Cmpen::from_bits(val as u8)
    }
    #[doc = "Compare Function Enable"]
    #[inline(always)]
    pub fn set_cmpen(&mut self, val: super::vals::Cmdh4Cmpen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh4Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh4Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh4Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh4Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh4Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh4Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh4Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh4Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh4Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh4Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh4Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh4Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh4Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh4Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh4Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh4 {
    #[inline(always)]
    fn default() -> Cmdh4 {
        Cmdh4(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh5(pub u32);
impl Cmdh5 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh5Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh5Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh5Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh5Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh5Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh5Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh5Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh5Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh5Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh5Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh5Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh5Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh5Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh5Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh5Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh5 {
    #[inline(always)]
    fn default() -> Cmdh5 {
        Cmdh5(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh6(pub u32);
impl Cmdh6 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh6Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh6Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh6Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh6Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh6Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh6Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh6Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh6Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh6Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh6Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh6Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh6Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh6Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh6Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh6Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh6 {
    #[inline(always)]
    fn default() -> Cmdh6 {
        Cmdh6(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh7(pub u32);
impl Cmdh7 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh7Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh7Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh7Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh7Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh7Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh7Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh7Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh7Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh7Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh7Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh7Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh7Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh7Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh7Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh7Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh7 {
    #[inline(always)]
    fn default() -> Cmdh7 {
        Cmdh7(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh8(pub u32);
impl Cmdh8 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh8Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh8Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh8Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh8Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh8Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh8Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh8Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh8Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh8Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh8Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh8Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh8Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh8Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh8Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh8Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh8 {
    #[inline(always)]
    fn default() -> Cmdh8 {
        Cmdh8(0)
    }
}
#[doc = "ADC Command High Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdh9(pub u32);
impl Cmdh9 {
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub const fn lwi(&self) -> super::vals::Cmdh9Lwi {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cmdh9Lwi::from_bits(val as u8)
    }
    #[doc = "Loop with Increment"]
    #[inline(always)]
    pub fn set_lwi(&mut self, val: super::vals::Cmdh9Lwi) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub const fn sts(&self) -> super::vals::Cmdh9Sts {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cmdh9Sts::from_bits(val as u8)
    }
    #[doc = "Sample Time Select"]
    #[inline(always)]
    pub fn set_sts(&mut self, val: super::vals::Cmdh9Sts) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub const fn avgs(&self) -> super::vals::Cmdh9Avgs {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Cmdh9Avgs::from_bits(val as u8)
    }
    #[doc = "Hardware Average Select"]
    #[inline(always)]
    pub fn set_avgs(&mut self, val: super::vals::Cmdh9Avgs) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Cmdh9Loop {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cmdh9Loop::from_bits(val as u8)
    }
    #[doc = "Loop Count Select"]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Cmdh9Loop) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub const fn next(&self) -> super::vals::Cmdh9Next {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdh9Next::from_bits(val as u8)
    }
    #[doc = "Next Command Select"]
    #[inline(always)]
    pub fn set_next(&mut self, val: super::vals::Cmdh9Next) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Cmdh9 {
    #[inline(always)]
    fn default() -> Cmdh9 {
        Cmdh9(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl1(pub u32);
impl Cmdl1 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl1Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl1Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl1Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl1Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl1Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl1Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl1Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl1Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl1Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl1Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl1Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl1Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl1 {
    #[inline(always)]
    fn default() -> Cmdl1 {
        Cmdl1(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl10(pub u32);
impl Cmdl10 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl10Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl10Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl10Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl10Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl10Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl10Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl10Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl10Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl10Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl10Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl10Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl10Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl10 {
    #[inline(always)]
    fn default() -> Cmdl10 {
        Cmdl10(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl11(pub u32);
impl Cmdl11 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl11Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl11Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl11Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl11Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl11Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl11Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl11Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl11Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl11Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl11Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl11Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl11Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl11 {
    #[inline(always)]
    fn default() -> Cmdl11 {
        Cmdl11(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl12(pub u32);
impl Cmdl12 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl12Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl12Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl12Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl12Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl12Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl12Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl12Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl12Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl12Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl12Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl12Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl12Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl12 {
    #[inline(always)]
    fn default() -> Cmdl12 {
        Cmdl12(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl13(pub u32);
impl Cmdl13 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl13Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl13Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl13Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl13Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl13Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl13Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl13Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl13Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl13Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl13Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl13Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl13Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl13 {
    #[inline(always)]
    fn default() -> Cmdl13 {
        Cmdl13(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl14(pub u32);
impl Cmdl14 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl14Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl14Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl14Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl14Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl14Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl14Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl14Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl14Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl14Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl14Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl14Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl14Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl14 {
    #[inline(always)]
    fn default() -> Cmdl14 {
        Cmdl14(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl15(pub u32);
impl Cmdl15 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl15Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl15Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl15Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl15Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl15Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl15Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl15Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl15Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl15Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl15Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl15Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl15Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl15 {
    #[inline(always)]
    fn default() -> Cmdl15 {
        Cmdl15(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl2(pub u32);
impl Cmdl2 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl2Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl2Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl2Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl2Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl2Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl2Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl2Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl2Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl2Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl2Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl2Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl2Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl2 {
    #[inline(always)]
    fn default() -> Cmdl2 {
        Cmdl2(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl3(pub u32);
impl Cmdl3 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl3Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl3Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl3Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl3Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl3Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl3Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl3Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl3Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl3Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl3Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl3Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl3Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl3 {
    #[inline(always)]
    fn default() -> Cmdl3 {
        Cmdl3(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl4(pub u32);
impl Cmdl4 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl4Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl4Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl4Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl4Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl4Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl4Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl4Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl4Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl4Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl4Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl4Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl4Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl4 {
    #[inline(always)]
    fn default() -> Cmdl4 {
        Cmdl4(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl5(pub u32);
impl Cmdl5 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl5Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl5Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl5Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl5Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl5Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl5Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl5Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl5Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl5Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl5Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl5Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl5Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl5 {
    #[inline(always)]
    fn default() -> Cmdl5 {
        Cmdl5(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl6(pub u32);
impl Cmdl6 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl6Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl6Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl6Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl6Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl6Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl6Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl6Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl6Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl6Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl6Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl6Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl6Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl6 {
    #[inline(always)]
    fn default() -> Cmdl6 {
        Cmdl6(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl7(pub u32);
impl Cmdl7 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl7Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl7Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl7Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl7Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl7Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl7Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl7Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl7Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl7Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl7Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl7Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl7Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl7 {
    #[inline(always)]
    fn default() -> Cmdl7 {
        Cmdl7(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl8(pub u32);
impl Cmdl8 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl8Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl8Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl8Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl8Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl8Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl8Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl8Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl8Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl8Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl8Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl8Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl8Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl8 {
    #[inline(always)]
    fn default() -> Cmdl8 {
        Cmdl8(0)
    }
}
#[doc = "ADC Command Low Buffer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdl9(pub u32);
impl Cmdl9 {
    #[doc = "Input channel select"]
    #[inline(always)]
    pub const fn adch(&self) -> super::vals::Cmdl9Adch {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmdl9Adch::from_bits(val as u8)
    }
    #[doc = "Input channel select"]
    #[inline(always)]
    pub fn set_adch(&mut self, val: super::vals::Cmdl9Adch) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub const fn absel(&self) -> super::vals::Cmdl9Absel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cmdl9Absel::from_bits(val as u8)
    }
    #[doc = "A-side vs. B-side Select"]
    #[inline(always)]
    pub fn set_absel(&mut self, val: super::vals::Cmdl9Absel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub const fn diff(&self) -> super::vals::Cmdl9Diff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cmdl9Diff::from_bits(val as u8)
    }
    #[doc = "Differential Mode Enable"]
    #[inline(always)]
    pub fn set_diff(&mut self, val: super::vals::Cmdl9Diff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub const fn cscale(&self) -> super::vals::Cmdl9Cscale {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Cmdl9Cscale::from_bits(val as u8)
    }
    #[doc = "Channel Scale"]
    #[inline(always)]
    pub fn set_cscale(&mut self, val: super::vals::Cmdl9Cscale) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
}
impl Default for Cmdl9 {
    #[inline(always)]
    fn default() -> Cmdl9 {
        Cmdl9(0)
    }
}
#[doc = "ADC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "ADC Enable"]
    #[inline(always)]
    pub const fn adcen(&self) -> super::vals::Adcen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Adcen::from_bits(val as u8)
    }
    #[doc = "ADC Enable"]
    #[inline(always)]
    pub fn set_adcen(&mut self, val: super::vals::Adcen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn rst(&self) -> super::vals::Rst {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn set_rst(&mut self, val: super::vals::Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn dozen(&self) -> super::vals::Dozen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub fn set_dozen(&mut self, val: super::vals::Dozen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reset FIFO"]
    #[inline(always)]
    pub const fn rstfifo(&self) -> super::vals::Rstfifo {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rstfifo::from_bits(val as u8)
    }
    #[doc = "Reset FIFO"]
    #[inline(always)]
    pub fn set_rstfifo(&mut self, val: super::vals::Rstfifo) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Compare Value Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cv(pub u32);
impl Cv {
    #[doc = "Compare Value Low."]
    #[inline(always)]
    pub const fn cvl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value Low."]
    #[inline(always)]
    pub fn set_cvl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Compare Value High."]
    #[inline(always)]
    pub const fn cvh(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Compare Value High."]
    #[inline(always)]
    pub fn set_cvh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cv {
    #[inline(always)]
    fn default() -> Cv {
        Cv(0)
    }
}
#[doc = "DMA Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct De(pub u32);
impl De {
    #[doc = "FIFO Watermark DMA Enable"]
    #[inline(always)]
    pub const fn fwmde(&self) -> super::vals::Fwmde {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fwmde::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark DMA Enable"]
    #[inline(always)]
    pub fn set_fwmde(&mut self, val: super::vals::Fwmde) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for De {
    #[inline(always)]
    fn default() -> De {
        De(0)
    }
}
#[doc = "ADC FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl(pub u32);
impl Fctrl {
    #[doc = "Result FIFO counter"]
    #[inline(always)]
    pub const fn fcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Result FIFO counter"]
    #[inline(always)]
    pub fn set_fcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Watermark level selection"]
    #[inline(always)]
    pub const fn fwmark(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Watermark level selection"]
    #[inline(always)]
    pub fn set_fwmark(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fctrl {
    #[inline(always)]
    fn default() -> Fctrl {
        Fctrl(0)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ie(pub u32);
impl Ie {
    #[doc = "FIFO Watermark Interrupt Enable"]
    #[inline(always)]
    pub const fn fwmie(&self) -> super::vals::Fwmie {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fwmie::from_bits(val as u8)
    }
    #[doc = "FIFO Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn set_fwmie(&mut self, val: super::vals::Fwmie) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn fofie(&self) -> super::vals::Fofie {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fofie::from_bits(val as u8)
    }
    #[doc = "Result FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn set_fofie(&mut self, val: super::vals::Fofie) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Ie {
    #[inline(always)]
    fn default() -> Ie {
        Ie(0)
    }
}
#[doc = "Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Trigger Number"]
    #[inline(always)]
    pub const fn trig_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Number"]
    #[inline(always)]
    pub fn set_trig_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Result FIFO Depth"]
    #[inline(always)]
    pub const fn fifosize(&self) -> super::vals::Fifosize {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Fifosize::from_bits(val as u8)
    }
    #[doc = "Result FIFO Depth"]
    #[inline(always)]
    pub fn set_fifosize(&mut self, val: super::vals::Fifosize) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Compare Value Number"]
    #[inline(always)]
    pub const fn cv_num(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Compare Value Number"]
    #[inline(always)]
    pub fn set_cv_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Command Buffer Number"]
    #[inline(always)]
    pub const fn cmd_num(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Command Buffer Number"]
    #[inline(always)]
    pub fn set_cmd_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
#[doc = "ADC Pause Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pause(pub u32);
impl Pause {
    #[doc = "Pause Delay"]
    #[inline(always)]
    pub const fn pausedly(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Pause Delay"]
    #[inline(always)]
    pub fn set_pausedly(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "PAUSE Option Enable"]
    #[inline(always)]
    pub const fn pauseen(&self) -> super::vals::Pauseen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pauseen::from_bits(val as u8)
    }
    #[doc = "PAUSE Option Enable"]
    #[inline(always)]
    pub fn set_pauseen(&mut self, val: super::vals::Pauseen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pause {
    #[inline(always)]
    fn default() -> Pause {
        Pause(0)
    }
}
#[doc = "ADC Data Result FIFO Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resfifo(pub u32);
impl Resfifo {
    #[doc = "Data result"]
    #[inline(always)]
    pub const fn d(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data result"]
    #[inline(always)]
    pub fn set_d(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub const fn tsrc(&self) -> super::vals::Tsrc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Tsrc::from_bits(val as u8)
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub fn set_tsrc(&mut self, val: super::vals::Tsrc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Loop count value"]
    #[inline(always)]
    pub const fn loopcnt(&self) -> super::vals::Loopcnt {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Loopcnt::from_bits(val as u8)
    }
    #[doc = "Loop count value"]
    #[inline(always)]
    pub fn set_loopcnt(&mut self, val: super::vals::Loopcnt) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Command Buffer Source"]
    #[inline(always)]
    pub const fn cmdsrc(&self) -> super::vals::Cmdsrc {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdsrc::from_bits(val as u8)
    }
    #[doc = "Command Buffer Source"]
    #[inline(always)]
    pub fn set_cmdsrc(&mut self, val: super::vals::Cmdsrc) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "FIFO entry is valid"]
    #[inline(always)]
    pub const fn valid(&self) -> super::vals::Valid {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Valid::from_bits(val as u8)
    }
    #[doc = "FIFO entry is valid"]
    #[inline(always)]
    pub fn set_valid(&mut self, val: super::vals::Valid) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Resfifo {
    #[inline(always)]
    fn default() -> Resfifo {
        Resfifo(0)
    }
}
#[doc = "ADC Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Result FIFO Ready Flag"]
    #[inline(always)]
    pub const fn rdy(&self) -> super::vals::Rdy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rdy::from_bits(val as u8)
    }
    #[doc = "Result FIFO Ready Flag"]
    #[inline(always)]
    pub fn set_rdy(&mut self, val: super::vals::Rdy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Result FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn fof(&self) -> super::vals::Fof {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Fof::from_bits(val as u8)
    }
    #[doc = "Result FIFO Overflow Flag"]
    #[inline(always)]
    pub fn set_fof(&mut self, val: super::vals::Fof) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Trigger Active"]
    #[inline(always)]
    pub const fn trgact(&self) -> super::vals::Trgact {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Trgact::from_bits(val as u8)
    }
    #[doc = "Trigger Active"]
    #[inline(always)]
    pub fn set_trgact(&mut self, val: super::vals::Trgact) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Command Active"]
    #[inline(always)]
    pub const fn cmdact(&self) -> super::vals::Cmdact {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Cmdact::from_bits(val as u8)
    }
    #[doc = "Command Active"]
    #[inline(always)]
    pub fn set_cmdact(&mut self, val: super::vals::Cmdact) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
#[doc = "Software Trigger Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swtrig(pub u32);
impl Swtrig {
    #[doc = "Software trigger 0 event"]
    #[inline(always)]
    pub const fn swt0(&self) -> super::vals::Swt0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swt0::from_bits(val as u8)
    }
    #[doc = "Software trigger 0 event"]
    #[inline(always)]
    pub fn set_swt0(&mut self, val: super::vals::Swt0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software trigger 1 event"]
    #[inline(always)]
    pub const fn swt1(&self) -> super::vals::Swt1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Swt1::from_bits(val as u8)
    }
    #[doc = "Software trigger 1 event"]
    #[inline(always)]
    pub fn set_swt1(&mut self, val: super::vals::Swt1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Software trigger 2 event"]
    #[inline(always)]
    pub const fn swt2(&self) -> super::vals::Swt2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Swt2::from_bits(val as u8)
    }
    #[doc = "Software trigger 2 event"]
    #[inline(always)]
    pub fn set_swt2(&mut self, val: super::vals::Swt2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Software trigger 3 event"]
    #[inline(always)]
    pub const fn swt3(&self) -> super::vals::Swt3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Swt3::from_bits(val as u8)
    }
    #[doc = "Software trigger 3 event"]
    #[inline(always)]
    pub fn set_swt3(&mut self, val: super::vals::Swt3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Software trigger 4 event"]
    #[inline(always)]
    pub const fn swt4(&self) -> super::vals::Swt4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Swt4::from_bits(val as u8)
    }
    #[doc = "Software trigger 4 event"]
    #[inline(always)]
    pub fn set_swt4(&mut self, val: super::vals::Swt4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Software trigger 5 event"]
    #[inline(always)]
    pub const fn swt5(&self) -> super::vals::Swt5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Swt5::from_bits(val as u8)
    }
    #[doc = "Software trigger 5 event"]
    #[inline(always)]
    pub fn set_swt5(&mut self, val: super::vals::Swt5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Software trigger 6 event"]
    #[inline(always)]
    pub const fn swt6(&self) -> super::vals::Swt6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Swt6::from_bits(val as u8)
    }
    #[doc = "Software trigger 6 event"]
    #[inline(always)]
    pub fn set_swt6(&mut self, val: super::vals::Swt6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Software trigger 7 event"]
    #[inline(always)]
    pub const fn swt7(&self) -> super::vals::Swt7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Swt7::from_bits(val as u8)
    }
    #[doc = "Software trigger 7 event"]
    #[inline(always)]
    pub fn set_swt7(&mut self, val: super::vals::Swt7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Software trigger 8 event"]
    #[inline(always)]
    pub const fn swt8(&self) -> super::vals::Swt8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Swt8::from_bits(val as u8)
    }
    #[doc = "Software trigger 8 event"]
    #[inline(always)]
    pub fn set_swt8(&mut self, val: super::vals::Swt8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Software trigger 9 event"]
    #[inline(always)]
    pub const fn swt9(&self) -> super::vals::Swt9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Swt9::from_bits(val as u8)
    }
    #[doc = "Software trigger 9 event"]
    #[inline(always)]
    pub fn set_swt9(&mut self, val: super::vals::Swt9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Software trigger 10 event"]
    #[inline(always)]
    pub const fn swt10(&self) -> super::vals::Swt10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Swt10::from_bits(val as u8)
    }
    #[doc = "Software trigger 10 event"]
    #[inline(always)]
    pub fn set_swt10(&mut self, val: super::vals::Swt10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software trigger 11 event"]
    #[inline(always)]
    pub const fn swt11(&self) -> super::vals::Swt11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Swt11::from_bits(val as u8)
    }
    #[doc = "Software trigger 11 event"]
    #[inline(always)]
    pub fn set_swt11(&mut self, val: super::vals::Swt11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Software trigger 12 event"]
    #[inline(always)]
    pub const fn swt12(&self) -> super::vals::Swt12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Swt12::from_bits(val as u8)
    }
    #[doc = "Software trigger 12 event"]
    #[inline(always)]
    pub fn set_swt12(&mut self, val: super::vals::Swt12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Software trigger 13 event"]
    #[inline(always)]
    pub const fn swt13(&self) -> super::vals::Swt13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Swt13::from_bits(val as u8)
    }
    #[doc = "Software trigger 13 event"]
    #[inline(always)]
    pub fn set_swt13(&mut self, val: super::vals::Swt13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Software trigger 14 event"]
    #[inline(always)]
    pub const fn swt14(&self) -> super::vals::Swt14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Swt14::from_bits(val as u8)
    }
    #[doc = "Software trigger 14 event"]
    #[inline(always)]
    pub fn set_swt14(&mut self, val: super::vals::Swt14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Software trigger 15 event"]
    #[inline(always)]
    pub const fn swt15(&self) -> super::vals::Swt15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Swt15::from_bits(val as u8)
    }
    #[doc = "Software trigger 15 event"]
    #[inline(always)]
    pub fn set_swt15(&mut self, val: super::vals::Swt15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Swtrig {
    #[inline(always)]
    fn default() -> Swtrig {
        Swtrig(0)
    }
}
#[doc = "Trigger Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctrl(pub u32);
impl Tctrl {
    #[doc = "Trigger enable"]
    #[inline(always)]
    pub const fn hten(&self) -> super::vals::Hten {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hten::from_bits(val as u8)
    }
    #[doc = "Trigger enable"]
    #[inline(always)]
    pub fn set_hten(&mut self, val: super::vals::Hten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger priority setting"]
    #[inline(always)]
    pub const fn tpri(&self) -> super::vals::Tpri {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Tpri::from_bits(val as u8)
    }
    #[doc = "Trigger priority setting"]
    #[inline(always)]
    pub fn set_tpri(&mut self, val: super::vals::Tpri) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Trigger delay select"]
    #[inline(always)]
    pub const fn tdly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Trigger delay select"]
    #[inline(always)]
    pub fn set_tdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Trigger command select"]
    #[inline(always)]
    pub const fn tcmd(&self) -> super::vals::Tcmd {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Tcmd::from_bits(val as u8)
    }
    #[doc = "Trigger command select"]
    #[inline(always)]
    pub fn set_tcmd(&mut self, val: super::vals::Tcmd) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Tctrl {
    #[inline(always)]
    fn default() -> Tctrl {
        Tctrl(0)
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Resolution"]
    #[inline(always)]
    pub const fn res(&self) -> super::vals::Res {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Res::from_bits(val as u8)
    }
    #[doc = "Resolution"]
    #[inline(always)]
    pub fn set_res(&mut self, val: super::vals::Res) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Differential Supported"]
    #[inline(always)]
    pub const fn diffen(&self) -> super::vals::Diffen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Diffen::from_bits(val as u8)
    }
    #[doc = "Differential Supported"]
    #[inline(always)]
    pub fn set_diffen(&mut self, val: super::vals::Diffen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Multi Vref Implemented"]
    #[inline(always)]
    pub const fn mvi(&self) -> super::vals::Mvi {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mvi::from_bits(val as u8)
    }
    #[doc = "Multi Vref Implemented"]
    #[inline(always)]
    pub fn set_mvi(&mut self, val: super::vals::Mvi) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Scale Width"]
    #[inline(always)]
    pub const fn csw(&self) -> super::vals::Csw {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Csw::from_bits(val as u8)
    }
    #[doc = "Channel Scale Width"]
    #[inline(always)]
    pub fn set_csw(&mut self, val: super::vals::Csw) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented"]
    #[inline(always)]
    pub const fn vr1rngi(&self) -> super::vals::Vr1rngi {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Vr1rngi::from_bits(val as u8)
    }
    #[doc = "Voltage Reference 1 Range Control Bit Implemented"]
    #[inline(always)]
    pub fn set_vr1rngi(&mut self, val: super::vals::Vr1rngi) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Internal ADC Clock implemented"]
    #[inline(always)]
    pub const fn iadcki(&self) -> super::vals::Iadcki {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Iadcki::from_bits(val as u8)
    }
    #[doc = "Internal ADC Clock implemented"]
    #[inline(always)]
    pub fn set_iadcki(&mut self, val: super::vals::Iadcki) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Calibration Offset Function Implemented"]
    #[inline(always)]
    pub const fn calofsi(&self) -> super::vals::Calofsi {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Calofsi::from_bits(val as u8)
    }
    #[doc = "Calibration Offset Function Implemented"]
    #[inline(always)]
    pub fn set_calofsi(&mut self, val: super::vals::Calofsi) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
