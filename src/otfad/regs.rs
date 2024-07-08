#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Force Logically Disabled Mode"]
    #[inline(always)]
    pub const fn fldm(&self) -> super::vals::Fldm {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Fldm::from_bits(val as u8)
    }
    #[doc = "Force Logically Disabled Mode"]
    #[inline(always)]
    pub fn set_fldm(&mut self, val: super::vals::Fldm) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Restricted Register Access Enable"]
    #[inline(always)]
    pub const fn rrae(&self) -> super::vals::Rrae {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Rrae::from_bits(val as u8)
    }
    #[doc = "Restricted Register Access Enable"]
    #[inline(always)]
    pub fn set_rrae(&mut self, val: super::vals::Rrae) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Global OTFAD Enable"]
    #[inline(always)]
    pub const fn ge(&self) -> super::vals::Ge {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Ge::from_bits(val as u8)
    }
    #[doc = "Global OTFAD Enable"]
    #[inline(always)]
    pub fn set_ge(&mut self, val: super::vals::Ge) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
#[doc = "AES Counter Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtxCtr(pub u32);
impl CtxCtr {
    #[doc = "AES Counter"]
    #[inline(always)]
    pub const fn ctr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES Counter"]
    #[inline(always)]
    pub fn set_ctr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CtxCtr {
    #[inline(always)]
    fn default() -> CtxCtr {
        CtxCtr(0)
    }
}
#[doc = "AES Key Word"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtxKey(pub u32);
impl CtxKey {
    #[doc = "AES Key"]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AES Key"]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CtxKey {
    #[inline(always)]
    fn default() -> CtxKey {
        CtxKey(0)
    }
}
#[doc = "AES Region Descriptor Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtxRgdW0(pub u32);
impl CtxRgdW0 {
    #[doc = "Start Address"]
    #[inline(always)]
    pub const fn srtaddr(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Start Address"]
    #[inline(always)]
    pub fn set_srtaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for CtxRgdW0 {
    #[inline(always)]
    fn default() -> CtxRgdW0 {
        CtxRgdW0(0)
    }
}
#[doc = "AES Region Descriptor Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtxRgdW1(pub u32);
impl CtxRgdW1 {
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn vld(&self) -> super::vals::Vld {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Vld::from_bits(val as u8)
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn set_vld(&mut self, val: super::vals::Vld) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AES Decryption Enable."]
    #[inline(always)]
    pub const fn ade(&self) -> super::vals::Ade {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ade::from_bits(val as u8)
    }
    #[doc = "AES Decryption Enable."]
    #[inline(always)]
    pub fn set_ade(&mut self, val: super::vals::Ade) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Read-Only"]
    #[inline(always)]
    pub const fn ro(&self) -> super::vals::Ro {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ro::from_bits(val as u8)
    }
    #[doc = "Read-Only"]
    #[inline(always)]
    pub fn set_ro(&mut self, val: super::vals::Ro) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "End Address"]
    #[inline(always)]
    pub const fn endaddr(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "End Address"]
    #[inline(always)]
    pub fn set_endaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 10usize)) | (((val as u32) & 0x003f_ffff) << 10usize);
    }
}
impl Default for CtxRgdW1 {
    #[inline(always)]
    fn default() -> CtxRgdW1 {
        CtxRgdW1(0)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "MDPC Present"]
    #[inline(always)]
    pub const fn mdpcp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "MDPC Present"]
    #[inline(always)]
    pub fn set_mdpcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Operating Mode"]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Operating Mode"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Number of Contexts"]
    #[inline(always)]
    pub const fn nctx(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Contexts"]
    #[inline(always)]
    pub fn set_nctx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Hardware Revision Level"]
    #[inline(always)]
    pub const fn hrl(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Hardware Revision Level"]
    #[inline(always)]
    pub fn set_hrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Restricted Register Access Mode"]
    #[inline(always)]
    pub const fn rram(&self) -> super::vals::Rram {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Rram::from_bits(val as u8)
    }
    #[doc = "Restricted Register Access Mode"]
    #[inline(always)]
    pub fn set_rram(&mut self, val: super::vals::Rram) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Global Enable Mode"]
    #[inline(always)]
    pub const fn gem(&self) -> super::vals::Gem {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Gem::from_bits(val as u8)
    }
    #[doc = "Global Enable Mode"]
    #[inline(always)]
    pub fn set_gem(&mut self, val: super::vals::Gem) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
