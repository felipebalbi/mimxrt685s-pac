#[doc = "Policy Select"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Polsel(pub u32);
impl Polsel {
    #[doc = "Policy Select for Region 0"]
    #[inline(always)]
    pub const fn reg0_policy(&self) -> super::vals::Reg0Policy {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Reg0Policy::from_bits(val as u8)
    }
    #[doc = "Policy Select for Region 0"]
    #[inline(always)]
    pub fn set_reg0_policy(&mut self, val: super::vals::Reg0Policy) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Policy Select for Region 0"]
    #[inline(always)]
    pub const fn reg1_policy(&self) -> super::vals::Reg1Policy {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Reg1Policy::from_bits(val as u8)
    }
    #[doc = "Policy Select for Region 0"]
    #[inline(always)]
    pub fn set_reg1_policy(&mut self, val: super::vals::Reg1Policy) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Policy Select for Region 0"]
    #[inline(always)]
    pub const fn reg02_policy(&self) -> super::vals::Reg02Policy {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Reg02Policy::from_bits(val as u8)
    }
    #[doc = "Policy Select for Region 0"]
    #[inline(always)]
    pub fn set_reg02_policy(&mut self, val: super::vals::Reg02Policy) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Polsel {
    #[inline(always)]
    fn default() -> Polsel {
        Polsel(0)
    }
}
#[doc = "Region 0 Top Boundary"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg0Top(pub u32);
impl Reg0Top {
    #[doc = "Upper limit of Region 0"]
    #[inline(always)]
    pub const fn reg0_top(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Upper limit of Region 0"]
    #[inline(always)]
    pub fn set_reg0_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 10usize)) | (((val as u32) & 0x0001_ffff) << 10usize);
    }
}
impl Default for Reg0Top {
    #[inline(always)]
    fn default() -> Reg0Top {
        Reg0Top(0)
    }
}
#[doc = "Region 1 Top Boundary"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg1Top(pub u32);
impl Reg1Top {
    #[doc = "Upper limit of Region 1"]
    #[inline(always)]
    pub const fn reg1_top(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "Upper limit of Region 1"]
    #[inline(always)]
    pub fn set_reg1_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 10usize)) | (((val as u32) & 0x0001_ffff) << 10usize);
    }
}
impl Default for Reg1Top {
    #[inline(always)]
    fn default() -> Reg1Top {
        Reg1Top(0)
    }
}
