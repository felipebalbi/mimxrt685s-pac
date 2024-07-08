#[doc = "Semphores2 Gate n"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate(pub u8);
impl Gate {
    #[doc = "ate Finite State Machine. The hardware gate is maintained in a 16-state implementation"]
    #[inline(always)]
    pub const fn gtfsm(&self) -> super::vals::Gtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Gtfsm::from_bits(val as u8)
    }
    #[doc = "ate Finite State Machine. The hardware gate is maintained in a 16-state implementation"]
    #[inline(always)]
    pub fn set_gtfsm(&mut self, val: super::vals::Gtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate {
    #[inline(always)]
    fn default() -> Gate {
        Gate(0)
    }
}
#[doc = "Reset Gate Read"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtR(pub u16);
impl RstgtR {
    #[doc = "RSTGTN"]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RSTGTN"]
    #[inline(always)]
    pub fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "RSTGMS"]
    #[inline(always)]
    pub const fn rstgms(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "RSTGMS"]
    #[inline(always)]
    pub fn set_rstgms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "RSTGSM"]
    #[inline(always)]
    pub const fn rstgsm(&self) -> super::vals::Rstgsm {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Rstgsm::from_bits(val as u8)
    }
    #[doc = "RSTGSM"]
    #[inline(always)]
    pub fn set_rstgsm(&mut self, val: super::vals::Rstgsm) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "ROZ"]
    #[inline(always)]
    pub const fn roz(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "ROZ"]
    #[inline(always)]
    pub fn set_roz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u16) & 0x03) << 14usize);
    }
}
impl Default for RstgtR {
    #[inline(always)]
    fn default() -> RstgtR {
        RstgtR(0)
    }
}
#[doc = "Reset Gate Write"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtW(pub u16);
impl RstgtW {
    #[doc = "RSTGTN"]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RSTGTN"]
    #[inline(always)]
    pub fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "RSTGDP"]
    #[inline(always)]
    pub const fn rstgdp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RSTGDP"]
    #[inline(always)]
    pub fn set_rstgdp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for RstgtW {
    #[inline(always)]
    fn default() -> RstgtW {
        RstgtW(0)
    }
}
