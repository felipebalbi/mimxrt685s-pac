#[doc = "MPU Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enables the MPU."]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "Enables the MPU."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the operation of MPU during HardFault and NMI handlers"]
    #[inline(always)]
    pub const fn hfnmiena(&self) -> super::vals::Hfnmiena {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hfnmiena::from_bits(val as u8)
    }
    #[doc = "Enables the operation of MPU during HardFault and NMI handlers"]
    #[inline(always)]
    pub fn set_hfnmiena(&mut self, val: super::vals::Hfnmiena) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn privdefena(&self) -> super::vals::Privdefena {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Privdefena::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn set_privdefena(&mut self, val: super::vals::Privdefena) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "MPU Memory Attribute Indirection Registers 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mair0(pub u32);
impl Mair0 {
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 0."]
    #[inline(always)]
    pub const fn attr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 0."]
    #[inline(always)]
    pub fn set_attr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 1."]
    #[inline(always)]
    pub const fn attr1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 1."]
    #[inline(always)]
    pub fn set_attr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 2."]
    #[inline(always)]
    pub const fn attr2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 2."]
    #[inline(always)]
    pub fn set_attr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 3."]
    #[inline(always)]
    pub const fn attr3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 3."]
    #[inline(always)]
    pub fn set_attr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mair0 {
    #[inline(always)]
    fn default() -> Mair0 {
        Mair0(0)
    }
}
#[doc = "MPU Memory Attribute Indirection Registers 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mair1(pub u32);
impl Mair1 {
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 4."]
    #[inline(always)]
    pub const fn attr4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 4."]
    #[inline(always)]
    pub fn set_attr4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 5."]
    #[inline(always)]
    pub const fn attr5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 5."]
    #[inline(always)]
    pub fn set_attr5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 6."]
    #[inline(always)]
    pub const fn attr6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 6."]
    #[inline(always)]
    pub fn set_attr6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 7."]
    #[inline(always)]
    pub const fn attr7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Memory attribute encoding for MPU regions with an AttrIndex of 7."]
    #[inline(always)]
    pub fn set_attr7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mair1 {
    #[inline(always)]
    fn default() -> Mair1 {
        Mair1(0)
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbar(pub u32);
impl Rbar {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn xn(&self) -> super::vals::RbarXn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RbarXn::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub fn set_xn(&mut self, val: super::vals::RbarXn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RbarAp {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RbarAp::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub fn set_ap(&mut self, val: super::vals::RbarAp) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn sh(&self) -> super::vals::RbarSh {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RbarSh::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub fn set_sh(&mut self, val: super::vals::RbarSh) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Rbar {
    #[inline(always)]
    fn default() -> Rbar {
        Rbar(0)
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbarA1(pub u32);
impl RbarA1 {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn xn(&self) -> super::vals::RbarA1Xn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RbarA1Xn::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub fn set_xn(&mut self, val: super::vals::RbarA1Xn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RbarA1Ap {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RbarA1Ap::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub fn set_ap(&mut self, val: super::vals::RbarA1Ap) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn sh(&self) -> super::vals::RbarA1Sh {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RbarA1Sh::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub fn set_sh(&mut self, val: super::vals::RbarA1Sh) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RbarA1 {
    #[inline(always)]
    fn default() -> RbarA1 {
        RbarA1(0)
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbarA2(pub u32);
impl RbarA2 {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn xn(&self) -> super::vals::RbarA2Xn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RbarA2Xn::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub fn set_xn(&mut self, val: super::vals::RbarA2Xn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RbarA2Ap {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RbarA2Ap::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub fn set_ap(&mut self, val: super::vals::RbarA2Ap) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn sh(&self) -> super::vals::RbarA2Sh {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RbarA2Sh::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub fn set_sh(&mut self, val: super::vals::RbarA2Sh) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RbarA2 {
    #[inline(always)]
    fn default() -> RbarA2 {
        RbarA2(0)
    }
}
#[doc = "MPU Region Base Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbarA3(pub u32);
impl RbarA3 {
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub const fn xn(&self) -> super::vals::RbarA3Xn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RbarA3Xn::from_bits(val as u8)
    }
    #[doc = "The XN bit is an Execute Never bit, that indicates whether the processor can execute instructions from the region."]
    #[inline(always)]
    pub fn set_xn(&mut self, val: super::vals::RbarA3Xn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub const fn ap(&self) -> super::vals::RbarA3Ap {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::RbarA3Ap::from_bits(val as u8)
    }
    #[doc = "The AP\\[2:0\\] bits indicate the access and privilege properties of the region."]
    #[inline(always)]
    pub fn set_ap(&mut self, val: super::vals::RbarA3Ap) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub const fn sh(&self) -> super::vals::RbarA3Sh {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::RbarA3Sh::from_bits(val as u8)
    }
    #[doc = "For Normal memory regions, the S bit indicates whether the region is shareable. For Strongly-ordered and Device memory, the S bit is ignored."]
    #[inline(always)]
    pub fn set_sh(&mut self, val: super::vals::RbarA3Sh) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub const fn base(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Contains bits\\[31:5\\] of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against."]
    #[inline(always)]
    pub fn set_base(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RbarA3 {
    #[inline(always)]
    fn default() -> RbarA3 {
        RbarA3(0)
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rlar(pub u32);
impl Rlar {
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn en(&self) -> super::vals::RlarEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RlarEn::from_bits(val as u8)
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::RlarEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn attrindx(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub fn set_attrindx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Rlar {
    #[inline(always)]
    fn default() -> Rlar {
        Rlar(0)
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RlarA1(pub u32);
impl RlarA1 {
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn en(&self) -> super::vals::RlarA1En {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RlarA1En::from_bits(val as u8)
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::RlarA1En) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn attrindx(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub fn set_attrindx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RlarA1 {
    #[inline(always)]
    fn default() -> RlarA1 {
        RlarA1(0)
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RlarA2(pub u32);
impl RlarA2 {
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn en(&self) -> super::vals::RlarA2En {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RlarA2En::from_bits(val as u8)
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::RlarA2En) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn attrindx(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub fn set_attrindx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RlarA2 {
    #[inline(always)]
    fn default() -> RlarA2 {
        RlarA2(0)
    }
}
#[doc = "MPU Region Limit Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RlarA3(pub u32);
impl RlarA3 {
    #[doc = "Enables this region."]
    #[inline(always)]
    pub const fn en(&self) -> super::vals::RlarA3En {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RlarA3En::from_bits(val as u8)
    }
    #[doc = "Enables this region."]
    #[inline(always)]
    pub fn set_en(&mut self, val: super::vals::RlarA3En) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub const fn attrindx(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Attribute index. Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields."]
    #[inline(always)]
    pub fn set_attrindx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address. Contains bits\\[31:5\\] of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against."]
    #[inline(always)]
    pub fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for RlarA3 {
    #[inline(always)]
    fn default() -> RlarA3 {
        RlarA3(0)
    }
}
#[doc = "MPU Region Number Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rnr(pub u32);
impl Rnr {
    #[doc = "Indicates the memory region accessed by MPU_RBAR and MPU_RASR."]
    #[inline(always)]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the memory region accessed by MPU_RBAR and MPU_RASR."]
    #[inline(always)]
    pub fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rnr {
    #[inline(always)]
    fn default() -> Rnr {
        Rnr(0)
    }
}
#[doc = "The MPU Type Register indicates how many regions the MPU support. Software can use it to determine if the processor implements an MPU."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type(pub u32);
impl Type {
    #[doc = "Indicates support for separate instruction and data address maps. RAZ. Armv8-M only supports a unified MPU"]
    #[inline(always)]
    pub const fn separate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates support for separate instruction and data address maps. RAZ. Armv8-M only supports a unified MPU"]
    #[inline(always)]
    pub fn set_separate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Number of regions supported by the MPU. If this field reads-as-zero the processor does not implement an MPU."]
    #[inline(always)]
    pub const fn dregion(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of regions supported by the MPU. If this field reads-as-zero the processor does not implement an MPU."]
    #[inline(always)]
    pub fn set_dregion(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Type {
    #[inline(always)]
    fn default() -> Type {
        Type(0)
    }
}
