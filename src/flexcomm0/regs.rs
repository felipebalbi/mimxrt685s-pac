#[doc = "Peripheral identification register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pid(pub u32);
impl Pid {
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pid {
    #[inline(always)]
    fn default() -> Pid {
        Pid(0)
    }
}
#[doc = "Peripheral Select and Flexcomm ID register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pselid(pub u32);
impl Pselid {
    #[doc = "Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub const fn persel(&self) -> super::vals::Persel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Persel::from_bits(val as u8)
    }
    #[doc = "Peripheral Select. This field is writable by software."]
    #[inline(always)]
    pub fn set_persel(&mut self, val: super::vals::Persel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub const fn lock(&self) -> super::vals::Lock {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lock::from_bits(val as u8)
    }
    #[doc = "Lock the peripheral select. This field is writable by software."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: super::vals::Lock) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "USART present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn usartpresent(&self) -> super::vals::Usartpresent {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Usartpresent::from_bits(val as u8)
    }
    #[doc = "USART present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn set_usartpresent(&mut self, val: super::vals::Usartpresent) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SPI present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn spipresent(&self) -> super::vals::Spipresent {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Spipresent::from_bits(val as u8)
    }
    #[doc = "SPI present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn set_spipresent(&mut self, val: super::vals::Spipresent) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "I2C present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn i2cpresent(&self) -> super::vals::I2cpresent {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::I2cpresent::from_bits(val as u8)
    }
    #[doc = "I2C present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn set_i2cpresent(&mut self, val: super::vals::I2cpresent) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "I 2S present indicator. This field is Read-only."]
    #[inline(always)]
    pub const fn i2spresent(&self) -> super::vals::I2spresent {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::I2spresent::from_bits(val as u8)
    }
    #[doc = "I 2S present indicator. This field is Read-only."]
    #[inline(always)]
    pub fn set_i2spresent(&mut self, val: super::vals::I2spresent) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Flexcomm ID."]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Flexcomm ID."]
    #[inline(always)]
    pub fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Pselid {
    #[inline(always)]
    fn default() -> Pselid {
        Pselid(0)
    }
}
