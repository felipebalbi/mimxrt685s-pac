#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Fn"]
    #[inline(always)]
    pub const fn fn_(&self) -> super::vals::CrFn {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::CrFn::from_bits(val as u8)
    }
    #[doc = "Fn"]
    #[inline(always)]
    pub fn set_fn_(&mut self, val: super::vals::CrFn) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "MUR"]
    #[inline(always)]
    pub const fn mur(&self) -> super::vals::Mur {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Mur::from_bits(val as u8)
    }
    #[doc = "MUR"]
    #[inline(always)]
    pub fn set_mur(&mut self, val: super::vals::Mur) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "BRDIE"]
    #[inline(always)]
    pub const fn rdie(&self) -> super::vals::Rdie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Rdie::from_bits(val as u8)
    }
    #[doc = "BRDIE"]
    #[inline(always)]
    pub fn set_rdie(&mut self, val: super::vals::Rdie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RAIE"]
    #[inline(always)]
    pub const fn raie(&self) -> super::vals::Raie {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Raie::from_bits(val as u8)
    }
    #[doc = "RAIE"]
    #[inline(always)]
    pub fn set_raie(&mut self, val: super::vals::Raie) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GIRn"]
    #[inline(always)]
    pub const fn girn(&self) -> super::vals::Girn {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Girn::from_bits(val as u8)
    }
    #[doc = "GIRn"]
    #[inline(always)]
    pub fn set_girn(&mut self, val: super::vals::Girn) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "TIEn"]
    #[inline(always)]
    pub const fn tien(&self) -> super::vals::Tien {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Tien::from_bits(val as u8)
    }
    #[doc = "TIEn"]
    #[inline(always)]
    pub fn set_tien(&mut self, val: super::vals::Tien) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "RIEn"]
    #[inline(always)]
    pub const fn rien(&self) -> super::vals::Rien {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Rien::from_bits(val as u8)
    }
    #[doc = "RIEn"]
    #[inline(always)]
    pub fn set_rien(&mut self, val: super::vals::Rien) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "GIEn"]
    #[inline(always)]
    pub const fn gien(&self) -> super::vals::Gien {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Gien::from_bits(val as u8)
    }
    #[doc = "GIEn"]
    #[inline(always)]
    pub fn set_gien(&mut self, val: super::vals::Gien) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "Use Parameter register to determine the parameter settings of MUA."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Par(pub u32);
impl Par {
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn parameter(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn set_parameter(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Par {
    #[inline(always)]
    fn default() -> Par {
        Par(0)
    }
}
#[doc = "Receive Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rr(pub u32);
impl Rr {
    #[doc = "DATA"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DATA"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rr {
    #[inline(always)]
    fn default() -> Rr {
        Rr(0)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Fn"]
    #[inline(always)]
    pub const fn fn_(&self) -> super::vals::SrFn {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SrFn::from_bits(val as u8)
    }
    #[doc = "Fn"]
    #[inline(always)]
    pub fn set_fn_(&mut self, val: super::vals::SrFn) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "EP"]
    #[inline(always)]
    pub const fn ep(&self) -> super::vals::Ep {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ep::from_bits(val as u8)
    }
    #[doc = "EP"]
    #[inline(always)]
    pub fn set_ep(&mut self, val: super::vals::Ep) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "PM"]
    #[inline(always)]
    pub const fn pm(&self) -> super::vals::Pm {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Pm::from_bits(val as u8)
    }
    #[doc = "PM"]
    #[inline(always)]
    pub fn set_pm(&mut self, val: super::vals::Pm) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "RS"]
    #[inline(always)]
    pub const fn rs(&self) -> super::vals::Rs {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Rs::from_bits(val as u8)
    }
    #[doc = "RS"]
    #[inline(always)]
    pub fn set_rs(&mut self, val: super::vals::Rs) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FUP"]
    #[inline(always)]
    pub const fn fup(&self) -> super::vals::Fup {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fup::from_bits(val as u8)
    }
    #[doc = "FUP"]
    #[inline(always)]
    pub fn set_fup(&mut self, val: super::vals::Fup) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "BRDIP"]
    #[inline(always)]
    pub const fn rdip(&self) -> super::vals::Rdip {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rdip::from_bits(val as u8)
    }
    #[doc = "BRDIP"]
    #[inline(always)]
    pub fn set_rdip(&mut self, val: super::vals::Rdip) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "RAIP"]
    #[inline(always)]
    pub const fn raip(&self) -> super::vals::Raip {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Raip::from_bits(val as u8)
    }
    #[doc = "RAIP"]
    #[inline(always)]
    pub fn set_raip(&mut self, val: super::vals::Raip) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "TEn"]
    #[inline(always)]
    pub const fn ten(&self) -> super::vals::Ten {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Ten::from_bits(val as u8)
    }
    #[doc = "TEn"]
    #[inline(always)]
    pub fn set_ten(&mut self, val: super::vals::Ten) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "RFn"]
    #[inline(always)]
    pub const fn rfn(&self) -> super::vals::Rfn {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Rfn::from_bits(val as u8)
    }
    #[doc = "RFn"]
    #[inline(always)]
    pub fn set_rfn(&mut self, val: super::vals::Rfn) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "GIPn"]
    #[inline(always)]
    pub const fn gipn(&self) -> super::vals::Gipn {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Gipn::from_bits(val as u8)
    }
    #[doc = "GIPn"]
    #[inline(always)]
    pub fn set_gipn(&mut self, val: super::vals::Gipn) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "Transmit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr(pub u32);
impl Tr {
    #[doc = "DATA"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "DATA"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tr {
    #[inline(always)]
    fn default() -> Tr {
        Tr(0)
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ver(pub u32);
impl Ver {
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
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
impl Default for Ver {
    #[inline(always)]
    fn default() -> Ver {
        Ver(0)
    }
}
