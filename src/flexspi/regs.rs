#[doc = "AHB Bus Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbcr(pub u32);
impl Ahbcr {
    #[doc = "Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline(always)]
    pub const fn aparen(&self) -> super::vals::Aparen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Aparen::from_bits(val as u8)
    }
    #[doc = "Parallel mode enabled for AHB triggered Command (both read and write) ."]
    #[inline(always)]
    pub fn set_aparen(&mut self, val: super::vals::Aparen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable AHB bus cachable read access support."]
    #[inline(always)]
    pub const fn cachableen(&self) -> super::vals::Cachableen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cachableen::from_bits(val as u8)
    }
    #[doc = "Enable AHB bus cachable read access support."]
    #[inline(always)]
    pub fn set_cachableen(&mut self, val: super::vals::Cachableen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline(always)]
    pub const fn bufferableen(&self) -> super::vals::Bufferableen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bufferableen::from_bits(val as u8)
    }
    #[doc = "Enable AHB bus bufferable write access support. This field affects the last beat of AHB write access, refer for more details about AHB bufferable write."]
    #[inline(always)]
    pub fn set_bufferableen(&mut self, val: super::vals::Bufferableen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline(always)]
    pub const fn readaddropt(&self) -> super::vals::Readaddropt {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Readaddropt::from_bits(val as u8)
    }
    #[doc = "AHB Read Address option bit. This option bit is intend to remove AHB burst start address alignment limitation."]
    #[inline(always)]
    pub fn set_readaddropt(&mut self, val: super::vals::Readaddropt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for Ahbcr {
    #[inline(always)]
    fn default() -> Ahbcr {
        Ahbcr(0)
    }
}
#[doc = "AHB RX Buffer 0 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf0cr0(pub u32);
impl Ahbrxbuf0cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf0cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf0cr0 {
        Ahbrxbuf0cr0(0)
    }
}
#[doc = "AHB RX Buffer 1 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf1cr0(pub u32);
impl Ahbrxbuf1cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf1cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf1cr0 {
        Ahbrxbuf1cr0(0)
    }
}
#[doc = "AHB RX Buffer 2 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf2cr0(pub u32);
impl Ahbrxbuf2cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf2cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf2cr0 {
        Ahbrxbuf2cr0(0)
    }
}
#[doc = "AHB RX Buffer 3 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf3cr0(pub u32);
impl Ahbrxbuf3cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf3cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf3cr0 {
        Ahbrxbuf3cr0(0)
    }
}
#[doc = "AHB RX Buffer 4 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf4cr0(pub u32);
impl Ahbrxbuf4cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf4cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf4cr0 {
        Ahbrxbuf4cr0(0)
    }
}
#[doc = "AHB RX Buffer 5 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf5cr0(pub u32);
impl Ahbrxbuf5cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf5cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf5cr0 {
        Ahbrxbuf5cr0(0)
    }
}
#[doc = "AHB RX Buffer 6 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf6cr0(pub u32);
impl Ahbrxbuf6cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf6cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf6cr0 {
        Ahbrxbuf6cr0(0)
    }
}
#[doc = "AHB RX Buffer 7 Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf7cr0(pub u32);
impl Ahbrxbuf7cr0 {
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub const fn bufsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "AHB RX Buffer Size in 64 bits."]
    #[inline(always)]
    pub fn set_bufsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This AHB RX Buffer is assigned according to AHB Master with ID (MSTR_ID)."]
    #[inline(always)]
    pub fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "This priority for AHB Master Read which this AHB RX Buffer is assigned. 7 is the highest priority, 0 the lowest."]
    #[inline(always)]
    pub fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub const fn prefetchen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Prefetch Enable for current AHB RX Buffer corresponding Master."]
    #[inline(always)]
    pub fn set_prefetchen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf7cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf7cr0 {
        Ahbrxbuf7cr0(0)
    }
}
#[doc = "AHB Suspend Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbspndsts(pub u32);
impl Ahbspndsts {
    #[doc = "Indicates if an AHB read prefetch command sequence has been suspended."]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if an AHB read prefetch command sequence has been suspended."]
    #[inline(always)]
    pub fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB RX BUF ID for suspended command sequence."]
    #[inline(always)]
    pub const fn bufid(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "AHB RX BUF ID for suspended command sequence."]
    #[inline(always)]
    pub fn set_bufid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Left Data size for suspended command sequence (in byte)."]
    #[inline(always)]
    pub const fn datlft(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Left Data size for suspended command sequence (in byte)."]
    #[inline(always)]
    pub fn set_datlft(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ahbspndsts {
    #[inline(always)]
    fn default() -> Ahbspndsts {
        Ahbspndsts(0)
    }
}
#[doc = "DLL Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dllcr(pub u32);
impl Dllcr {
    #[doc = "DLL calibration enable."]
    #[inline(always)]
    pub const fn dllen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DLL calibration enable."]
    #[inline(always)]
    pub fn set_dllen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[inline(always)]
    pub const fn dllreset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software could force a reset on DLL by setting this field to 0x1. This will cause the DLL to lose lock and re-calibrate to detect an ref_clock half period phase shift. The reset action is edge triggered, so software need to clear this bit after set this bit (no delay limitation)."]
    #[inline(always)]
    pub fn set_dllreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
    #[inline(always)]
    pub const fn slvdlytarget(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "The delay target for slave delay line is: ((SLVDLYTARGET+1) * 1/32 * clock cycle of reference clock (serial root clock). If serial root clock is >= 100 MHz, DLLEN set to 0x1, OVRDEN set to =0x0, then SLVDLYTARGET setting of 0xF is recommended."]
    #[inline(always)]
    pub fn set_slvdlytarget(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    pub const fn ovrden(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave clock delay line delay cell number selection override enable."]
    #[inline(always)]
    pub fn set_ovrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    pub const fn ovrdval(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Slave clock delay line delay cell number selection override value."]
    #[inline(always)]
    pub fn set_ovrdval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
}
impl Default for Dllcr {
    #[inline(always)]
    fn default() -> Dllcr {
        Dllcr(0)
    }
}
#[doc = "Data Learn Pattern Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlpr(pub u32);
impl Dlpr {
    #[doc = "Data Learning Pattern."]
    #[inline(always)]
    pub const fn dlp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Learning Pattern."]
    #[inline(always)]
    pub fn set_dlp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dlpr {
    #[inline(always)]
    fn default() -> Dlpr {
        Dlpr(0)
    }
}
#[doc = "Flash Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha1cr0(pub u32);
impl Flsha1cr0 {
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Flsha1cr0 {
    #[inline(always)]
    fn default() -> Flsha1cr0 {
        Flsha1cr0(0)
    }
}
#[doc = "Flash Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha2cr0(pub u32);
impl Flsha2cr0 {
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Flsha2cr0 {
    #[inline(always)]
    fn default() -> Flsha2cr0 {
        Flsha2cr0(0)
    }
}
#[doc = "Flash Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb1cr0(pub u32);
impl Flshb1cr0 {
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Flshb1cr0 {
    #[inline(always)]
    fn default() -> Flshb1cr0 {
        Flshb1cr0(0)
    }
}
#[doc = "Flash Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb2cr0(pub u32);
impl Flshb2cr0 {
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KByte."]
    #[inline(always)]
    pub fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Flshb2cr0 {
    #[inline(always)]
    fn default() -> Flshb2cr0 {
        Flshb2cr0(0)
    }
}
#[doc = "Flash Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr1(pub u32);
impl Flshcr1 {
    #[doc = "Serial Flash CS setup time."]
    #[inline(always)]
    pub const fn tcss(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS setup time."]
    #[inline(always)]
    pub fn set_tcss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Serial Flash CS Hold time."]
    #[inline(always)]
    pub const fn tcsh(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Hold time."]
    #[inline(always)]
    pub fn set_tcsh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Word Addressable."]
    #[inline(always)]
    pub const fn wa(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Word Addressable."]
    #[inline(always)]
    pub fn set_wa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Column Address Size."]
    #[inline(always)]
    pub const fn cas(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Column Address Size."]
    #[inline(always)]
    pub fn set_cas(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "CS interval unit"]
    #[inline(always)]
    pub const fn csintervalunit(&self) -> super::vals::Csintervalunit {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Csintervalunit::from_bits(val as u8)
    }
    #[doc = "CS interval unit"]
    #[inline(always)]
    pub fn set_csintervalunit(&mut self, val: super::vals::Csintervalunit) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline(always)]
    pub const fn csinterval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "This field is used to set the minimum interval between flash device Chip selection deassertion and flash device Chip selection assertion. If external flash has a limitation on the interval between command sequences, this field should be set accordingly. If there is no limitation, set this field with value 0x0."]
    #[inline(always)]
    pub fn set_csinterval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Flshcr1 {
    #[inline(always)]
    fn default() -> Flshcr1 {
        Flshcr1(0)
    }
}
#[doc = "Flash Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr2(pub u32);
impl Flshcr2 {
    #[doc = "Sequence Index for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub const fn ardseqid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn set_ardseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Sequence Number for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub const fn ardseqnum(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn set_ardseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Sequence Index for AHB Write triggered Command."]
    #[inline(always)]
    pub const fn awrseqid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Write triggered Command."]
    #[inline(always)]
    pub fn set_awrseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Sequence Number for AHB Write triggered Command."]
    #[inline(always)]
    pub const fn awrseqnum(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Write triggered Command."]
    #[inline(always)]
    pub fn set_awrseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline(always)]
    pub const fn awrwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline(always)]
    pub fn set_awrwait(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "AWRWAIT unit"]
    #[inline(always)]
    pub const fn awrwaitunit(&self) -> super::vals::Awrwaitunit {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Awrwaitunit::from_bits(val as u8)
    }
    #[doc = "AWRWAIT unit"]
    #[inline(always)]
    pub fn set_awrwaitunit(&mut self, val: super::vals::Awrwaitunit) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline(always)]
    pub const fn clrinstrptr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline(always)]
    pub fn set_clrinstrptr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Flshcr2 {
    #[inline(always)]
    fn default() -> Flshcr2 {
        Flshcr2(0)
    }
}
#[doc = "Flash Control Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr4(pub u32);
impl Flshcr4 {
    #[doc = "Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline(always)]
    pub const fn wmopt1(&self) -> super::vals::Wmopt1 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Wmopt1::from_bits(val as u8)
    }
    #[doc = "Write mask option bit 1. This option bit could be used to remove AHB write burst start address alignment limitation."]
    #[inline(always)]
    pub fn set_wmopt1(&mut self, val: super::vals::Wmopt1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline(always)]
    pub const fn wmena(&self) -> super::vals::Wmena {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wmena::from_bits(val as u8)
    }
    #[doc = "Write mask enable bit for flash device on port A. When write mask function is needed for memory device on port A, this bit must be set."]
    #[inline(always)]
    pub fn set_wmena(&mut self, val: super::vals::Wmena) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline(always)]
    pub const fn wmenb(&self) -> super::vals::Wmenb {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wmenb::from_bits(val as u8)
    }
    #[doc = "Write mask enable bit for flash device on port B. When write mask function is needed for memory device on port B, this bit must be set."]
    #[inline(always)]
    pub fn set_wmenb(&mut self, val: super::vals::Wmenb) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Flshcr4 {
    #[inline(always)]
    fn default() -> Flshcr4 {
        Flshcr4(0)
    }
}
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    pub const fn ipcmddoneen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Execution finished interrupt enable."]
    #[inline(always)]
    pub fn set_ipcmddoneen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub const fn ipcmdgeen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn set_ipcmdgeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub const fn ahbcmdgeen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt enable."]
    #[inline(always)]
    pub fn set_ahbcmdgeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub const fn ipcmderren(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn set_ipcmderren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub const fn ahbcmderren(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt enable."]
    #[inline(always)]
    pub fn set_ahbcmderren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    pub const fn iprxwaen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IP RX FIFO WaterMark available interrupt enable."]
    #[inline(always)]
    pub fn set_iprxwaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    pub const fn iptxween(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IP TX FIFO WaterMark empty interrupt enable."]
    #[inline(always)]
    pub fn set_iptxween(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Learning failed interrupt enable."]
    #[inline(always)]
    pub const fn datalearnfailen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data Learning failed interrupt enable."]
    #[inline(always)]
    pub fn set_datalearnfailen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    pub const fn sckstopbyrden(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt enable."]
    #[inline(always)]
    pub fn set_sckstopbyrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    pub const fn sckstopbywren(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt enable."]
    #[inline(always)]
    pub fn set_sckstopbywren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub const fn ahbbustimeouten(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn set_ahbbustimeouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub const fn seqtimeouten(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence execution timeout interrupt enable.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn set_seqtimeouten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[inline(always)]
    pub const fn ipcmddone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Execution finished interrupt. This interrupt is also generated when there is IPCMDGE or IPCMDERR interrupt generated."]
    #[inline(always)]
    pub fn set_ipcmddone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub const fn ipcmdge(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn set_ipcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub const fn ahbcmdge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB triggered Command Sequences Grant Timeout interrupt."]
    #[inline(always)]
    pub fn set_ahbcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub const fn ipcmderr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IP triggered Command Sequences Error Detected interrupt. When an error detected for IP command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn set_ipcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub const fn ahbcmderr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AHB triggered Command Sequences Error Detected interrupt. When an error detected for AHB command, this command will be ignored and not executed at all."]
    #[inline(always)]
    pub fn set_ahbcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IP RX FIFO watermark available interrupt."]
    #[inline(always)]
    pub const fn iprxwa(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IP RX FIFO watermark available interrupt."]
    #[inline(always)]
    pub fn set_iprxwa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IP TX FIFO watermark empty interrupt."]
    #[inline(always)]
    pub const fn iptxwe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IP TX FIFO watermark empty interrupt."]
    #[inline(always)]
    pub fn set_iptxwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Learning failed interrupt."]
    #[inline(always)]
    pub const fn datalearnfail(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data Learning failed interrupt."]
    #[inline(always)]
    pub fn set_datalearnfail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[inline(always)]
    pub const fn sckstopbyrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK is stopped during command sequence because Async RX FIFO full interrupt."]
    #[inline(always)]
    pub fn set_sckstopbyrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[inline(always)]
    pub const fn sckstopbywr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK is stopped during command sequence because Async TX FIFO empty interrupt."]
    #[inline(always)]
    pub fn set_sckstopbywr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub const fn ahbbustimeout(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Bus timeout interrupt.Refer Interrupts chapter for more details."]
    #[inline(always)]
    pub fn set_ahbbustimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence execution timeout interrupt."]
    #[inline(always)]
    pub const fn seqtimeout(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence execution timeout interrupt."]
    #[inline(always)]
    pub fn set_seqtimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "IP Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcmd(pub u32);
impl Ipcmd {
    #[doc = "Setting this bit will trigger an IP Command."]
    #[inline(always)]
    pub const fn trg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit will trigger an IP Command."]
    #[inline(always)]
    pub fn set_trg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ipcmd {
    #[inline(always)]
    fn default() -> Ipcmd {
        Ipcmd(0)
    }
}
#[doc = "IP Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr0(pub u32);
impl Ipcr0 {
    #[doc = "Serial Flash Address for IP command."]
    #[inline(always)]
    pub const fn sfar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Serial Flash Address for IP command."]
    #[inline(always)]
    pub fn set_sfar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipcr0 {
    #[inline(always)]
    fn default() -> Ipcr0 {
        Ipcr0(0)
    }
}
#[doc = "IP Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr1(pub u32);
impl Ipcr1 {
    #[doc = "Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline(always)]
    pub const fn idatsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Flash Read/Program Data Size (in Bytes) for IP command."]
    #[inline(always)]
    pub fn set_idatsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub const fn iseqid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub fn set_iseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    pub const fn iseqnum(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    pub fn set_iseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Parallel mode Enabled for IP command."]
    #[inline(always)]
    pub const fn iparen(&self) -> super::vals::Iparen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Iparen::from_bits(val as u8)
    }
    #[doc = "Parallel mode Enabled for IP command."]
    #[inline(always)]
    pub fn set_iparen(&mut self, val: super::vals::Iparen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ipcr1 {
    #[inline(always)]
    fn default() -> Ipcr1 {
        Ipcr1(0)
    }
}
#[doc = "IP RX FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfcr(pub u32);
impl Iprxfcr {
    #[doc = "Clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    pub const fn clriprxf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear all valid data entries in IP RX FIFO."]
    #[inline(always)]
    pub fn set_clriprxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP RX FIFO reading by DMA enabled."]
    #[inline(always)]
    pub const fn rxdmaen(&self) -> super::vals::Rxdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rxdmaen::from_bits(val as u8)
    }
    #[doc = "IP RX FIFO reading by DMA enabled."]
    #[inline(always)]
    pub fn set_rxdmaen(&mut self, val: super::vals::Rxdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Watermark level is (RXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub const fn rxwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Watermark level is (RXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn set_rxwmrk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
}
impl Default for Iprxfcr {
    #[inline(always)]
    fn default() -> Iprxfcr {
        Iprxfcr(0)
    }
}
#[doc = "IP RX FIFO Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfsts(pub u32);
impl Iprxfsts {
    #[doc = "Fill level of IP RX FIFO."]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill level of IP RX FIFO."]
    #[inline(always)]
    pub fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Total Read Data Counter: RDCNTR * 64 Bits."]
    #[inline(always)]
    pub const fn rdcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Total Read Data Counter: RDCNTR * 64 Bits."]
    #[inline(always)]
    pub fn set_rdcntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Iprxfsts {
    #[inline(always)]
    fn default() -> Iprxfsts {
        Iprxfsts(0)
    }
}
#[doc = "IP TX FIFO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfcr(pub u32);
impl Iptxfcr {
    #[doc = "Clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    pub const fn clriptxf(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear all valid data entries in IP TX FIFO."]
    #[inline(always)]
    pub fn set_clriptxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP TX FIFO filling by DMA enabled."]
    #[inline(always)]
    pub const fn txdmaen(&self) -> super::vals::Txdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Txdmaen::from_bits(val as u8)
    }
    #[doc = "IP TX FIFO filling by DMA enabled."]
    #[inline(always)]
    pub fn set_txdmaen(&mut self, val: super::vals::Txdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub const fn txwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "Watermark level is (TXWMRK+1)*64 Bits."]
    #[inline(always)]
    pub fn set_txwmrk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
}
impl Default for Iptxfcr {
    #[inline(always)]
    fn default() -> Iptxfcr {
        Iptxfcr(0)
    }
}
#[doc = "IP TX FIFO Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfsts(pub u32);
impl Iptxfsts {
    #[doc = "Fill level of IP TX FIFO."]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill level of IP TX FIFO."]
    #[inline(always)]
    pub fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Total Write Data Counter: WRCNTR * 64 Bits."]
    #[inline(always)]
    pub const fn wrcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Total Write Data Counter: WRCNTR * 64 Bits."]
    #[inline(always)]
    pub fn set_wrcntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Iptxfsts {
    #[inline(always)]
    fn default() -> Iptxfsts {
        Iptxfsts(0)
    }
}
#[doc = "LUT x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lut(pub u32);
impl Lut {
    #[doc = "OPERAND0"]
    #[inline(always)]
    pub const fn operand0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OPERAND0"]
    #[inline(always)]
    pub fn set_operand0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "NUM_PADS0"]
    #[inline(always)]
    pub const fn num_pads0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "NUM_PADS0"]
    #[inline(always)]
    pub fn set_num_pads0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "OPCODE"]
    #[inline(always)]
    pub const fn opcode0(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "OPCODE"]
    #[inline(always)]
    pub fn set_opcode0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "OPERAND1"]
    #[inline(always)]
    pub const fn operand1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "OPERAND1"]
    #[inline(always)]
    pub fn set_operand1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "NUM_PADS1"]
    #[inline(always)]
    pub const fn num_pads1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "NUM_PADS1"]
    #[inline(always)]
    pub fn set_num_pads1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "OPCODE1"]
    #[inline(always)]
    pub const fn opcode1(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "OPCODE1"]
    #[inline(always)]
    pub fn set_opcode1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Lut {
    #[inline(always)]
    fn default() -> Lut {
        Lut(0)
    }
}
#[doc = "LUT Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutcr(pub u32);
impl Lutcr {
    #[doc = "Lock LUT"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock LUT"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Unlock LUT"]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock LUT"]
    #[inline(always)]
    pub fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Lutcr {
    #[inline(always)]
    fn default() -> Lutcr {
        Lutcr(0)
    }
}
#[doc = "LUT Key Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutkey(pub u32);
impl Lutkey {
    #[doc = "The Key to lock or unlock LUT."]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The Key to lock or unlock LUT."]
    #[inline(always)]
    pub fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lutkey {
    #[inline(always)]
    fn default() -> Lutkey {
        Lutkey(0)
    }
}
#[doc = "Module Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr0(pub u32);
impl Mcr0 {
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn swreset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn set_swreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Module Disable"]
    #[inline(always)]
    pub const fn mdis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Module Disable"]
    #[inline(always)]
    pub fn set_mdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Sample Clock source selection for Flash Reading"]
    #[inline(always)]
    pub const fn rxclksrc(&self) -> super::vals::Rxclksrc {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Rxclksrc::from_bits(val as u8)
    }
    #[doc = "Sample Clock source selection for Flash Reading"]
    #[inline(always)]
    pub fn set_rxclksrc(&mut self, val: super::vals::Rxclksrc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "The serial root clock could be divided inside FlexSPI wrapper. Refer Clocks chapter for more details on clocking."]
    #[inline(always)]
    pub const fn serclkdiv(&self) -> super::vals::Serclkdiv {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Serclkdiv::from_bits(val as u8)
    }
    #[doc = "The serial root clock could be divided inside FlexSPI wrapper. Refer Clocks chapter for more details on clocking."]
    #[inline(always)]
    pub fn set_serclkdiv(&mut self, val: super::vals::Serclkdiv) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Half Speed Serial Flash access Enable."]
    #[inline(always)]
    pub const fn hsen(&self) -> super::vals::Hsen {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Hsen::from_bits(val as u8)
    }
    #[doc = "Half Speed Serial Flash access Enable."]
    #[inline(always)]
    pub fn set_hsen(&mut self, val: super::vals::Hsen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Doze mode enable bit"]
    #[inline(always)]
    pub const fn dozeen(&self) -> super::vals::Dozeen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dozeen::from_bits(val as u8)
    }
    #[doc = "Doze mode enable bit"]
    #[inline(always)]
    pub fn set_dozeen(&mut self, val: super::vals::Dozeen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is used to force SCLK output free-running. For FPGA applications, external device may use SCLK as reference clock to its internal PLL. If SCLK free-running is enabled, data sampling with loopback clock from SCLK pad is not supported (MCR0\\[RXCLKSRC\\]=2)."]
    #[inline(always)]
    pub const fn sckfreerunen(&self) -> super::vals::Sckfreerunen {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Sckfreerunen::from_bits(val as u8)
    }
    #[doc = "This bit is used to force SCLK output free-running. For FPGA applications, external device may use SCLK as reference clock to its internal PLL. If SCLK free-running is enabled, data sampling with loopback clock from SCLK pad is not supported (MCR0\\[RXCLKSRC\\]=2)."]
    #[inline(always)]
    pub fn set_sckfreerunen(&mut self, val: super::vals::Sckfreerunen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "This bit is used to enable/disable data learning feature. When data learning is disabled, the sampling clock phase 0 is always used for RX data sampling even if LEARN instruction is correctly executed."]
    #[inline(always)]
    pub const fn learnen(&self) -> super::vals::Learnen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Learnen::from_bits(val as u8)
    }
    #[doc = "This bit is used to enable/disable data learning feature. When data learning is disabled, the sampling clock phase 0 is always used for RX data sampling even if LEARN instruction is correctly executed."]
    #[inline(always)]
    pub fn set_learnen(&mut self, val: super::vals::Learnen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Time out wait cycle for IP command grant."]
    #[inline(always)]
    pub const fn ipgrantwait(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Time out wait cycle for IP command grant."]
    #[inline(always)]
    pub fn set_ipgrantwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Timeout wait cycle for AHB command grant."]
    #[inline(always)]
    pub const fn ahbgrantwait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Timeout wait cycle for AHB command grant."]
    #[inline(always)]
    pub fn set_ahbgrantwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mcr0 {
    #[inline(always)]
    fn default() -> Mcr0 {
        Mcr0(0)
    }
}
#[doc = "Module Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr1(pub u32);
impl Mcr1 {
    #[doc = "AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[inline(always)]
    pub const fn ahbbuswait(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AHB Read/Write access to Serial Flash Memory space will timeout if not data received from Flash or data not transmitted after AHBBUSWAIT * 1024 ahb clock cycles, AHB Bus will get an error response"]
    #[inline(always)]
    pub fn set_ahbbuswait(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[inline(always)]
    pub const fn seqwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Command Sequence Execution will timeout and abort after SEQWAIT * 1024 Serial Root Clock cycles"]
    #[inline(always)]
    pub fn set_seqwait(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mcr1 {
    #[inline(always)]
    fn default() -> Mcr1 {
        Mcr1(0)
    }
}
#[doc = "Module Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr2(pub u32);
impl Mcr2 {
    #[doc = "This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline(always)]
    pub const fn clrahbbufopt(&self) -> super::vals::Clrahbbufopt {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Clrahbbufopt::from_bits(val as u8)
    }
    #[doc = "This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline(always)]
    pub fn set_clrahbbufopt(&mut self, val: super::vals::Clrahbbufopt) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    pub const fn clrlearnphase(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "The sampling clock phase selection will be reset to phase 0 when this bit is written with 0x1. This bit will be auto-cleared immediately."]
    #[inline(always)]
    pub fn set_clrlearnphase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline(always)]
    pub const fn samedeviceen(&self) -> super::vals::Samedeviceen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Samedeviceen::from_bits(val as u8)
    }
    #[doc = "All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline(always)]
    pub fn set_samedeviceen(&mut self, val: super::vals::Samedeviceen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\] should be set."]
    #[inline(always)]
    pub const fn sckbdiffopt(&self) -> super::vals::Sckbdiffopt {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Sckbdiffopt::from_bits(val as u8)
    }
    #[doc = "B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\] should be set."]
    #[inline(always)]
    pub fn set_sckbdiffopt(&mut self, val: super::vals::Sckbdiffopt) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    pub const fn resumewait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    pub fn set_resumewait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mcr2 {
    #[inline(always)]
    fn default() -> Mcr2 {
        Mcr2(0)
    }
}
#[doc = "IP RX FIFO Data Register x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfdr(pub u32);
impl Rfdr {
    #[doc = "RX Data"]
    #[inline(always)]
    pub const fn rxdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RX Data"]
    #[inline(always)]
    pub fn set_rxdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rfdr {
    #[inline(always)]
    fn default() -> Rfdr {
        Rfdr(0)
    }
}
#[doc = "Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts0(pub u32);
impl Sts0 {
    #[doc = "This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
    #[inline(always)]
    pub const fn seqidle(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit indicates the state machine in SEQ_CTL is idle and there is command sequence executing on FlexSPI interface."]
    #[inline(always)]
    pub fn set_seqidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
    #[inline(always)]
    pub const fn arbidle(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This status bit indicates the state machine in ARB_CTL is busy and there is command sequence granted by arbitrator and not finished yet on FlexSPI interface. When ARB_CTL state (ARBIDLE=0x1) is idle, there will be no transaction on FlexSPI interface also (SEQIDLE=0x1). So this bit should be polled to wait for FlexSPI controller become idle instead of SEQIDLE."]
    #[inline(always)]
    pub fn set_arbidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
    #[inline(always)]
    pub const fn arbcmdsrc(&self) -> super::vals::Arbcmdsrc {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Arbcmdsrc::from_bits(val as u8)
    }
    #[doc = "This status field indicates the trigger source of current command sequence granted by arbitrator. This field value is meaningless when ARB_CTL is not busy (STS0\\[ARBIDLE\\]=0x1)."]
    #[inline(always)]
    pub fn set_arbcmdsrc(&mut self, val: super::vals::Arbcmdsrc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Indicate the sampling clock phase selection on Port A after Data Learning."]
    #[inline(always)]
    pub const fn datalearnphasea(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the sampling clock phase selection on Port A after Data Learning."]
    #[inline(always)]
    pub fn set_datalearnphasea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicate the sampling clock phase selection on Port B after Data Learning."]
    #[inline(always)]
    pub const fn datalearnphaseb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicate the sampling clock phase selection on Port B after Data Learning."]
    #[inline(always)]
    pub fn set_datalearnphaseb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Sts0 {
    #[inline(always)]
    fn default() -> Sts0 {
        Sts0(0)
    }
}
#[doc = "Status Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts1(pub u32);
impl Sts1 {
    #[doc = "Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub const fn ahbcmderrid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the sequence index when an AHB command error is detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn set_ahbcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub const fn ahbcmderrcode(&self) -> super::vals::Ahbcmderrcode {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Ahbcmderrcode::from_bits(val as u8)
    }
    #[doc = "Indicates the Error Code when AHB command Error detected. This field will be cleared when INTR\\[AHBCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn set_ahbcmderrcode(&mut self, val: super::vals::Ahbcmderrcode) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Indicates the sequence Index when IP command error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub const fn ipcmderrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the sequence Index when IP command error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn set_ipcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub const fn ipcmderrcode(&self) -> super::vals::Ipcmderrcode {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Ipcmderrcode::from_bits(val as u8)
    }
    #[doc = "Indicates the Error Code when IP command Error detected. This field will be cleared when INTR\\[IPCMDERR\\] is write-1-clear(w1c)."]
    #[inline(always)]
    pub fn set_ipcmderrcode(&mut self, val: super::vals::Ipcmderrcode) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Sts1 {
    #[inline(always)]
    fn default() -> Sts1 {
        Sts1(0)
    }
}
#[doc = "Status Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts2(pub u32);
impl Sts2 {
    #[doc = "Flash A sample clock slave delay line locked."]
    #[inline(always)]
    pub const fn aslvlock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flash A sample clock slave delay line locked."]
    #[inline(always)]
    pub fn set_aslvlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash A sample clock reference delay line locked."]
    #[inline(always)]
    pub const fn areflock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flash A sample clock reference delay line locked."]
    #[inline(always)]
    pub fn set_areflock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash A sample clock slave delay line delay cell number selection ."]
    #[inline(always)]
    pub const fn aslvsel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A sample clock slave delay line delay cell number selection ."]
    #[inline(always)]
    pub fn set_aslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Flash A sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub const fn arefsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn set_arefsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Flash B sample clock slave delay line locked."]
    #[inline(always)]
    pub const fn bslvlock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Flash B sample clock slave delay line locked."]
    #[inline(always)]
    pub fn set_bslvlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Flash B sample clock reference delay line locked."]
    #[inline(always)]
    pub const fn breflock(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Flash B sample clock reference delay line locked."]
    #[inline(always)]
    pub fn set_breflock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Flash B sample clock slave delay line delay cell number selection."]
    #[inline(always)]
    pub const fn bslvsel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B sample clock slave delay line delay cell number selection."]
    #[inline(always)]
    pub fn set_bslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "Flash B sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub const fn brefsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B sample clock reference delay line delay cell number selection."]
    #[inline(always)]
    pub fn set_brefsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Sts2 {
    #[inline(always)]
    fn default() -> Sts2 {
        Sts2(0)
    }
}
#[doc = "IP TX FIFO Data Register x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfdr(pub u32);
impl Tfdr {
    #[doc = "TX Data"]
    #[inline(always)]
    pub const fn txdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TX Data"]
    #[inline(always)]
    pub fn set_txdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tfdr {
    #[inline(always)]
    fn default() -> Tfdr {
        Tfdr(0)
    }
}
