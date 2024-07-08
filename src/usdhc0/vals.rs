#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12WrChkbusyEn {
    #[doc = "Do not check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_0 = 0x0,
    #[doc = "Check busy after auto CMD12 for write data packet"]
    AC12_WR_CHKBUSY_EN_1 = 0x01,
}
impl Ac12WrChkbusyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12WrChkbusyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12WrChkbusyEn {
    #[inline(always)]
    fn from(val: u8) -> Ac12WrChkbusyEn {
        Ac12WrChkbusyEn::from_bits(val)
    }
}
impl From<Ac12WrChkbusyEn> for u8 {
    #[inline(always)]
    fn from(val: Ac12WrChkbusyEn) -> u8 {
        Ac12WrChkbusyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12ce {
    #[doc = "No CRC error"]
    AC12CE_0 = 0x0,
    #[doc = "CRC Error Met in Auto CMD12/23 Response"]
    AC12CE_1 = 0x01,
}
impl Ac12ce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ce {
    #[inline(always)]
    fn from(val: u8) -> Ac12ce {
        Ac12ce::from_bits(val)
    }
}
impl From<Ac12ce> for u8 {
    #[inline(always)]
    fn from(val: Ac12ce) -> u8 {
        Ac12ce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12e {
    #[doc = "No Error"]
    AC12E_0 = 0x0,
    #[doc = "Error"]
    AC12E_1 = 0x01,
}
impl Ac12e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12e {
    #[inline(always)]
    fn from(val: u8) -> Ac12e {
        Ac12e::from_bits(val)
    }
}
impl From<Ac12e> for u8 {
    #[inline(always)]
    fn from(val: Ac12e) -> u8 {
        Ac12e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12ebe {
    #[doc = "No error"]
    AC12EBE_0 = 0x0,
    #[doc = "End Bit Error Generated"]
    AC12EBE_1 = 0x01,
}
impl Ac12ebe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ebe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ebe {
    #[inline(always)]
    fn from(val: u8) -> Ac12ebe {
        Ac12ebe::from_bits(val)
    }
}
impl From<Ac12ebe> for u8 {
    #[inline(always)]
    fn from(val: Ac12ebe) -> u8 {
        Ac12ebe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12eien {
    #[doc = "Masked"]
    AC12EIEN_0 = 0x0,
    #[doc = "Enabled"]
    AC12EIEN_1 = 0x01,
}
impl Ac12eien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12eien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12eien {
    #[inline(always)]
    fn from(val: u8) -> Ac12eien {
        Ac12eien::from_bits(val)
    }
}
impl From<Ac12eien> for u8 {
    #[inline(always)]
    fn from(val: Ac12eien) -> u8 {
        Ac12eien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12en {
    #[doc = "Disable"]
    AC12EN_0 = 0x0,
    #[doc = "Enable"]
    AC12EN_1 = 0x01,
}
impl Ac12en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12en {
    #[inline(always)]
    fn from(val: u8) -> Ac12en {
        Ac12en::from_bits(val)
    }
}
impl From<Ac12en> for u8 {
    #[inline(always)]
    fn from(val: Ac12en) -> u8 {
        Ac12en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12esen {
    #[doc = "Masked"]
    AC12ESEN_0 = 0x0,
    #[doc = "Enabled"]
    AC12ESEN_1 = 0x01,
}
impl Ac12esen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12esen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12esen {
    #[inline(always)]
    fn from(val: u8) -> Ac12esen {
        Ac12esen::from_bits(val)
    }
}
impl From<Ac12esen> for u8 {
    #[inline(always)]
    fn from(val: Ac12esen) -> u8 {
        Ac12esen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12ie {
    #[doc = "No error"]
    AC12IE_0 = 0x0,
    #[doc = "Error, the CMD index in response is not CMD12/23"]
    AC12IE_1 = 0x01,
}
impl Ac12ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ie {
    #[inline(always)]
    fn from(val: u8) -> Ac12ie {
        Ac12ie::from_bits(val)
    }
}
impl From<Ac12ie> for u8 {
    #[inline(always)]
    fn from(val: Ac12ie) -> u8 {
        Ac12ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12ne {
    #[doc = "Executed"]
    AC12NE_0 = 0x0,
    #[doc = "Not executed"]
    AC12NE_1 = 0x01,
}
impl Ac12ne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ne {
    #[inline(always)]
    fn from(val: u8) -> Ac12ne {
        Ac12ne::from_bits(val)
    }
}
impl From<Ac12ne> for u8 {
    #[inline(always)]
    fn from(val: Ac12ne) -> u8 {
        Ac12ne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ac12toe {
    #[doc = "No error"]
    AC12TOE_0 = 0x0,
    #[doc = "Time out"]
    AC12TOE_1 = 0x01,
}
impl Ac12toe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12toe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12toe {
    #[inline(always)]
    fn from(val: u8) -> Ac12toe {
        Ac12toe::from_bits(val)
    }
}
impl From<Ac12toe> for u8 {
    #[inline(always)]
    fn from(val: Ac12toe) -> u8 {
        Ac12toe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Acmd23Argu2En {
    #[doc = "Disable"]
    ACMD23_ARGU2_EN_0 = 0x0,
    #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enable."]
    ACMD23_ARGU2_EN_1 = 0x01,
}
impl Acmd23Argu2En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmd23Argu2En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmd23Argu2En {
    #[inline(always)]
    fn from(val: u8) -> Acmd23Argu2En {
        Acmd23Argu2En::from_bits(val)
    }
}
impl From<Acmd23Argu2En> for u8 {
    #[inline(always)]
    fn from(val: Acmd23Argu2En) -> u8 {
        Acmd23Argu2En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Admadce {
    #[doc = "No Error"]
    ADMADCE_0 = 0x0,
    #[doc = "Error"]
    ADMADCE_1 = 0x01,
}
impl Admadce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admadce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admadce {
    #[inline(always)]
    fn from(val: u8) -> Admadce {
        Admadce::from_bits(val)
    }
}
impl From<Admadce> for u8 {
    #[inline(always)]
    fn from(val: Admadce) -> u8 {
        Admadce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Admalme {
    #[doc = "No Error"]
    ADMALME_0 = 0x0,
    #[doc = "Error"]
    ADMALME_1 = 0x01,
}
impl Admalme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admalme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admalme {
    #[inline(always)]
    fn from(val: u8) -> Admalme {
        Admalme::from_bits(val)
    }
}
impl From<Admalme> for u8 {
    #[inline(always)]
    fn from(val: Admalme) -> u8 {
        Admalme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Admas {
    #[doc = "Advanced DMA Not supported"]
    ADMAS_0 = 0x0,
    #[doc = "Advanced DMA Supported"]
    ADMAS_1 = 0x01,
}
impl Admas {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admas {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admas {
    #[inline(always)]
    fn from(val: u8) -> Admas {
        Admas::from_bits(val)
    }
}
impl From<Admas> for u8 {
    #[inline(always)]
    fn from(val: Admas) -> u8 {
        Admas::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum AutoTuneEn {
    #[doc = "Disable auto tuning"]
    AUTO_TUNE_EN_0 = 0x0,
    #[doc = "Enable auto tuning"]
    AUTO_TUNE_EN_1 = 0x01,
}
impl AutoTuneEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoTuneEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoTuneEn {
    #[inline(always)]
    fn from(val: u8) -> AutoTuneEn {
        AutoTuneEn::from_bits(val)
    }
}
impl From<AutoTuneEn> for u8 {
    #[inline(always)]
    fn from(val: AutoTuneEn) -> u8 {
        AutoTuneEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Autocmd12ErrStatusSmpClkSel {
    #[doc = "Fixed clock is used to sample data"]
    SMP_CLK_SEL_0 = 0x0,
    #[doc = "Tuned clock is used to sample data"]
    SMP_CLK_SEL_1 = 0x01,
}
impl Autocmd12ErrStatusSmpClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autocmd12ErrStatusSmpClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autocmd12ErrStatusSmpClkSel {
    #[inline(always)]
    fn from(val: u8) -> Autocmd12ErrStatusSmpClkSel {
        Autocmd12ErrStatusSmpClkSel::from_bits(val)
    }
}
impl From<Autocmd12ErrStatusSmpClkSel> for u8 {
    #[inline(always)]
    fn from(val: Autocmd12ErrStatusSmpClkSel) -> u8 {
        Autocmd12ErrStatusSmpClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bcen {
    #[doc = "Disable"]
    BCEN_0 = 0x0,
    #[doc = "Enable"]
    BCEN_1 = 0x01,
}
impl Bcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bcen {
    #[inline(always)]
    fn from(val: u8) -> Bcen {
        Bcen::from_bits(val)
    }
}
impl From<Bcen> for u8 {
    #[inline(always)]
    fn from(val: Bcen) -> u8 {
        Bcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bge {
    #[doc = "No block gap event"]
    BGE_0 = 0x0,
    #[doc = "Transaction stopped at block gap"]
    BGE_1 = 0x01,
}
impl Bge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bge {
    #[inline(always)]
    fn from(val: u8) -> Bge {
        Bge::from_bits(val)
    }
}
impl From<Bge> for u8 {
    #[inline(always)]
    fn from(val: Bge) -> u8 {
        Bge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bgeien {
    #[doc = "Masked"]
    BGEIEN_0 = 0x0,
    #[doc = "Enabled"]
    BGEIEN_1 = 0x01,
}
impl Bgeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bgeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bgeien {
    #[inline(always)]
    fn from(val: u8) -> Bgeien {
        Bgeien::from_bits(val)
    }
}
impl From<Bgeien> for u8 {
    #[inline(always)]
    fn from(val: Bgeien) -> u8 {
        Bgeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bgesen {
    #[doc = "Masked"]
    BGESEN_0 = 0x0,
    #[doc = "Enabled"]
    BGESEN_1 = 0x01,
}
impl Bgesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bgesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bgesen {
    #[inline(always)]
    fn from(val: u8) -> Bgesen {
        Bgesen::from_bits(val)
    }
}
impl From<Bgesen> for u8 {
    #[inline(always)]
    fn from(val: Bgesen) -> u8 {
        Bgesen::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Blkcnt(pub u16);
impl Blkcnt {
    #[doc = "Stop Count"]
    pub const BLKCNT_0: Self = Self(0x0);
    #[doc = "1 block"]
    pub const BLKCNT_1: Self = Self(0x01);
    #[doc = "2 blocks"]
    pub const BLKCNT_2: Self = Self(0x02);
    #[doc = "65535 blocks"]
    pub const BLKCNT_65535: Self = Self(0xffff);
}
impl Blkcnt {
    pub const fn from_bits(val: u16) -> Blkcnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Blkcnt {
    #[inline(always)]
    fn from(val: u16) -> Blkcnt {
        Blkcnt::from_bits(val)
    }
}
impl From<Blkcnt> for u16 {
    #[inline(always)]
    fn from(val: Blkcnt) -> u16 {
        Blkcnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Blksize(pub u16);
impl Blksize {
    #[doc = "No data transfer"]
    pub const BLKSIZE_0: Self = Self(0x0);
    #[doc = "1 Byte"]
    pub const BLKSIZE_1: Self = Self(0x01);
    #[doc = "2 Bytes"]
    pub const BLKSIZE_2: Self = Self(0x02);
    #[doc = "3 Bytes"]
    pub const BLKSIZE_3: Self = Self(0x03);
    #[doc = "4 Bytes"]
    pub const BLKSIZE_4: Self = Self(0x04);
    #[doc = "511 Bytes"]
    pub const BLKSIZE_511: Self = Self(0x01ff);
    #[doc = "512 Bytes"]
    pub const BLKSIZE_512: Self = Self(0x0200);
    #[doc = "2048 Bytes"]
    pub const BLKSIZE_2048: Self = Self(0x0800);
    #[doc = "4096 Bytes"]
    pub const BLKSIZE_4096: Self = Self(0x1000);
}
impl Blksize {
    pub const fn from_bits(val: u16) -> Blksize {
        Self(val & 0x1fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Blksize {
    #[inline(always)]
    fn from(val: u16) -> Blksize {
        Blksize::from_bits(val)
    }
}
impl From<Blksize> for u16 {
    #[inline(always)]
    fn from(val: Blksize) -> u16 {
        Blksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum BootAck {
    #[doc = "No ack"]
    BOOT_ACK_0 = 0x0,
    #[doc = "Ack"]
    BOOT_ACK_1 = 0x01,
}
impl BootAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootAck {
    #[inline(always)]
    fn from(val: u8) -> BootAck {
        BootAck::from_bits(val)
    }
}
impl From<BootAck> for u8 {
    #[inline(always)]
    fn from(val: BootAck) -> u8 {
        BootAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum BootEn {
    #[doc = "Fast boot disable"]
    BOOT_EN_0 = 0x0,
    #[doc = "Fast boot enable"]
    BOOT_EN_1 = 0x01,
}
impl BootEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootEn {
    #[inline(always)]
    fn from(val: u8) -> BootEn {
        BootEn::from_bits(val)
    }
}
impl From<BootEn> for u8 {
    #[inline(always)]
    fn from(val: BootEn) -> u8 {
        BootEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum BootMode {
    #[doc = "Normal boot"]
    BOOT_MODE_0 = 0x0,
    #[doc = "Alternative boot"]
    BOOT_MODE_1 = 0x01,
}
impl BootMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootMode {
    #[inline(always)]
    fn from(val: u8) -> BootMode {
        BootMode::from_bits(val)
    }
}
impl From<BootMode> for u8 {
    #[inline(always)]
    fn from(val: BootMode) -> u8 {
        BootMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bren {
    #[doc = "Read disable"]
    BREN_0 = 0x0,
    #[doc = "Read enable"]
    BREN_1 = 0x01,
}
impl Bren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bren {
    #[inline(always)]
    fn from(val: u8) -> Bren {
        Bren::from_bits(val)
    }
}
impl From<Bren> for u8 {
    #[inline(always)]
    fn from(val: Bren) -> u8 {
        Bren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Brr {
    #[doc = "Not ready to read buffer"]
    BRR_0 = 0x0,
    #[doc = "Ready to read buffer"]
    BRR_1 = 0x01,
}
impl Brr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brr {
    #[inline(always)]
    fn from(val: u8) -> Brr {
        Brr::from_bits(val)
    }
}
impl From<Brr> for u8 {
    #[inline(always)]
    fn from(val: Brr) -> u8 {
        Brr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Brrien {
    #[doc = "Masked"]
    BRRIEN_0 = 0x0,
    #[doc = "Enabled"]
    BRRIEN_1 = 0x01,
}
impl Brrien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brrien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brrien {
    #[inline(always)]
    fn from(val: u8) -> Brrien {
        Brrien::from_bits(val)
    }
}
impl From<Brrien> for u8 {
    #[inline(always)]
    fn from(val: Brrien) -> u8 {
        Brrien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Brrsen {
    #[doc = "Masked"]
    BRRSEN_0 = 0x0,
    #[doc = "Enabled"]
    BRRSEN_1 = 0x01,
}
impl Brrsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brrsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brrsen {
    #[inline(always)]
    fn from(val: u8) -> Brrsen {
        Brrsen::from_bits(val)
    }
}
impl From<Brrsen> for u8 {
    #[inline(always)]
    fn from(val: Brrsen) -> u8 {
        Brrsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum BurstLenEn {
    _RESERVED_0 = 0x0,
    #[doc = "Burst length is enabled for INCR"]
    BURST_LEN_EN_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl BurstLenEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BurstLenEn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BurstLenEn {
    #[inline(always)]
    fn from(val: u8) -> BurstLenEn {
        BurstLenEn::from_bits(val)
    }
}
impl From<BurstLenEn> for u8 {
    #[inline(always)]
    fn from(val: BurstLenEn) -> u8 {
        BurstLenEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bwen {
    #[doc = "Write disable"]
    BWEN_0 = 0x0,
    #[doc = "Write enable"]
    BWEN_1 = 0x01,
}
impl Bwen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwen {
    #[inline(always)]
    fn from(val: u8) -> Bwen {
        Bwen::from_bits(val)
    }
}
impl From<Bwen> for u8 {
    #[inline(always)]
    fn from(val: Bwen) -> u8 {
        Bwen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bwr {
    #[doc = "Not ready to write buffer"]
    BWR_0 = 0x0,
    #[doc = "Ready to write buffer:"]
    BWR_1 = 0x01,
}
impl Bwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwr {
    #[inline(always)]
    fn from(val: u8) -> Bwr {
        Bwr::from_bits(val)
    }
}
impl From<Bwr> for u8 {
    #[inline(always)]
    fn from(val: Bwr) -> u8 {
        Bwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bwrien {
    #[doc = "Masked"]
    BWRIEN_0 = 0x0,
    #[doc = "Enabled"]
    BWRIEN_1 = 0x01,
}
impl Bwrien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwrien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwrien {
    #[inline(always)]
    fn from(val: u8) -> Bwrien {
        Bwrien::from_bits(val)
    }
}
impl From<Bwrien> for u8 {
    #[inline(always)]
    fn from(val: Bwrien) -> u8 {
        Bwrien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bwrsen {
    #[doc = "Masked"]
    BWRSEN_0 = 0x0,
    #[doc = "Enabled"]
    BWRSEN_1 = 0x01,
}
impl Bwrsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwrsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwrsen {
    #[inline(always)]
    fn from(val: u8) -> Bwrsen {
        Bwrsen::from_bits(val)
    }
}
impl From<Bwrsen> for u8 {
    #[inline(always)]
    fn from(val: Bwrsen) -> u8 {
        Bwrsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CardIntD3Test {
    #[doc = "Check the card interrupt only when DATA3 is high."]
    CARD_INT_D3_TEST_0 = 0x0,
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    CARD_INT_D3_TEST_1 = 0x01,
}
impl CardIntD3Test {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CardIntD3Test {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CardIntD3Test {
    #[inline(always)]
    fn from(val: u8) -> CardIntD3Test {
        CardIntD3Test::from_bits(val)
    }
}
impl From<CardIntD3Test> for u8 {
    #[inline(always)]
    fn from(val: CardIntD3Test) -> u8 {
        CardIntD3Test::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cc {
    #[doc = "Command not complete"]
    CC_0 = 0x0,
    #[doc = "Command complete"]
    CC_1 = 0x01,
}
impl Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc {
    #[inline(always)]
    fn from(val: u8) -> Cc {
        Cc::from_bits(val)
    }
}
impl From<Cc> for u8 {
    #[inline(always)]
    fn from(val: Cc) -> u8 {
        Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cccen {
    #[doc = "Disable"]
    CCCEN_0 = 0x0,
    #[doc = "Enable"]
    CCCEN_1 = 0x01,
}
impl Cccen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cccen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cccen {
    #[inline(always)]
    fn from(val: u8) -> Cccen {
        Cccen::from_bits(val)
    }
}
impl From<Cccen> for u8 {
    #[inline(always)]
    fn from(val: Cccen) -> u8 {
        Cccen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cce {
    #[doc = "No Error"]
    CCE_0 = 0x0,
    #[doc = "CRC Error Generated."]
    CCE_1 = 0x01,
}
impl Cce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cce {
    #[inline(always)]
    fn from(val: u8) -> Cce {
        Cce::from_bits(val)
    }
}
impl From<Cce> for u8 {
    #[inline(always)]
    fn from(val: Cce) -> u8 {
        Cce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cceien {
    #[doc = "Masked"]
    CCEIEN_0 = 0x0,
    #[doc = "Enabled"]
    CCEIEN_1 = 0x01,
}
impl Cceien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cceien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cceien {
    #[inline(always)]
    fn from(val: u8) -> Cceien {
        Cceien::from_bits(val)
    }
}
impl From<Cceien> for u8 {
    #[inline(always)]
    fn from(val: Cceien) -> u8 {
        Cceien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ccesen {
    #[doc = "Masked"]
    CCESEN_0 = 0x0,
    #[doc = "Enabled"]
    CCESEN_1 = 0x01,
}
impl Ccesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccesen {
    #[inline(always)]
    fn from(val: u8) -> Ccesen {
        Ccesen::from_bits(val)
    }
}
impl From<Ccesen> for u8 {
    #[inline(always)]
    fn from(val: Ccesen) -> u8 {
        Ccesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ccien {
    #[doc = "Masked"]
    CCIEN_0 = 0x0,
    #[doc = "Enabled"]
    CCIEN_1 = 0x01,
}
impl Ccien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccien {
    #[inline(always)]
    fn from(val: u8) -> Ccien {
        Ccien::from_bits(val)
    }
}
impl From<Ccien> for u8 {
    #[inline(always)]
    fn from(val: Ccien) -> u8 {
        Ccien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ccsen {
    #[doc = "Masked"]
    CCSEN_0 = 0x0,
    #[doc = "Enabled"]
    CCSEN_1 = 0x01,
}
impl Ccsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccsen {
    #[inline(always)]
    fn from(val: u8) -> Ccsen {
        Ccsen::from_bits(val)
    }
}
impl From<Ccsen> for u8 {
    #[inline(always)]
    fn from(val: Ccsen) -> u8 {
        Ccsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cdihb {
    #[doc = "Can issue command which uses the DATA line"]
    CDIHB_0 = 0x0,
    #[doc = "Cannot issue command which uses the DATA line"]
    CDIHB_1 = 0x01,
}
impl Cdihb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdihb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdihb {
    #[inline(always)]
    fn from(val: u8) -> Cdihb {
        Cdihb::from_bits(val)
    }
}
impl From<Cdihb> for u8 {
    #[inline(always)]
    fn from(val: Cdihb) -> u8 {
        Cdihb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cdpl {
    #[doc = "No card present (CD_B = 1)"]
    CDPL_0 = 0x0,
    #[doc = "Card present (CD_B = 0)"]
    CDPL_1 = 0x01,
}
impl Cdpl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdpl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdpl {
    #[inline(always)]
    fn from(val: u8) -> Cdpl {
        Cdpl::from_bits(val)
    }
}
impl From<Cdpl> for u8 {
    #[inline(always)]
    fn from(val: Cdpl) -> u8 {
        Cdpl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cdss {
    #[doc = "Card Detection Level is selected (for normal purpose)."]
    CDSS_0 = 0x0,
    #[doc = "Card Detection Test Level is selected (for test purpose)."]
    CDSS_1 = 0x01,
}
impl Cdss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdss {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdss {
    #[inline(always)]
    fn from(val: u8) -> Cdss {
        Cdss::from_bits(val)
    }
}
impl From<Cdss> for u8 {
    #[inline(always)]
    fn from(val: Cdss) -> u8 {
        Cdss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cdtl {
    #[doc = "Card Detect Test Level is 0, no card inserted"]
    CDTL_0 = 0x0,
    #[doc = "Card Detect Test Level is 1, card inserted"]
    CDTL_1 = 0x01,
}
impl Cdtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdtl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdtl {
    #[inline(always)]
    fn from(val: u8) -> Cdtl {
        Cdtl::from_bits(val)
    }
}
impl From<Cdtl> for u8 {
    #[inline(always)]
    fn from(val: Cdtl) -> u8 {
        Cdtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cebe {
    #[doc = "No Error"]
    CEBE_0 = 0x0,
    #[doc = "End Bit Error Generated"]
    CEBE_1 = 0x01,
}
impl Cebe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cebe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cebe {
    #[inline(always)]
    fn from(val: u8) -> Cebe {
        Cebe::from_bits(val)
    }
}
impl From<Cebe> for u8 {
    #[inline(always)]
    fn from(val: Cebe) -> u8 {
        Cebe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cebeien {
    #[doc = "Masked"]
    CEBEIEN_0 = 0x0,
    #[doc = "Enabled"]
    CEBEIEN_1 = 0x01,
}
impl Cebeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cebeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cebeien {
    #[inline(always)]
    fn from(val: u8) -> Cebeien {
        Cebeien::from_bits(val)
    }
}
impl From<Cebeien> for u8 {
    #[inline(always)]
    fn from(val: Cebeien) -> u8 {
        Cebeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cebesen {
    #[doc = "Masked"]
    CEBESEN_0 = 0x0,
    #[doc = "Enabled"]
    CEBESEN_1 = 0x01,
}
impl Cebesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cebesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cebesen {
    #[inline(always)]
    fn from(val: u8) -> Cebesen {
        Cebesen::from_bits(val)
    }
}
impl From<Cebesen> for u8 {
    #[inline(always)]
    fn from(val: Cebesen) -> u8 {
        Cebesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cicen {
    #[doc = "Disable"]
    CICEN_0 = 0x0,
    #[doc = "Enable"]
    CICEN_1 = 0x01,
}
impl Cicen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cicen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cicen {
    #[inline(always)]
    fn from(val: u8) -> Cicen {
        Cicen::from_bits(val)
    }
}
impl From<Cicen> for u8 {
    #[inline(always)]
    fn from(val: Cicen) -> u8 {
        Cicen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cie {
    #[doc = "No Error"]
    CIE_0 = 0x0,
    #[doc = "Error"]
    CIE_1 = 0x01,
}
impl Cie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cie {
    #[inline(always)]
    fn from(val: u8) -> Cie {
        Cie::from_bits(val)
    }
}
impl From<Cie> for u8 {
    #[inline(always)]
    fn from(val: Cie) -> u8 {
        Cie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cieien {
    #[doc = "Masked"]
    CIEIEN_0 = 0x0,
    #[doc = "Enabled"]
    CIEIEN_1 = 0x01,
}
impl Cieien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cieien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cieien {
    #[inline(always)]
    fn from(val: u8) -> Cieien {
        Cieien::from_bits(val)
    }
}
impl From<Cieien> for u8 {
    #[inline(always)]
    fn from(val: Cieien) -> u8 {
        Cieien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ciesen {
    #[doc = "Masked"]
    CIESEN_0 = 0x0,
    #[doc = "Enabled"]
    CIESEN_1 = 0x01,
}
impl Ciesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ciesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ciesen {
    #[inline(always)]
    fn from(val: u8) -> Ciesen {
        Ciesen::from_bits(val)
    }
}
impl From<Ciesen> for u8 {
    #[inline(always)]
    fn from(val: Ciesen) -> u8 {
        Ciesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cihb {
    #[doc = "Can issue command using only CMD line"]
    CIHB_0 = 0x0,
    #[doc = "Cannot issue command"]
    CIHB_1 = 0x01,
}
impl Cihb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cihb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cihb {
    #[inline(always)]
    fn from(val: u8) -> Cihb {
        Cihb::from_bits(val)
    }
}
impl From<Cihb> for u8 {
    #[inline(always)]
    fn from(val: Cihb) -> u8 {
        Cihb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cins {
    #[doc = "Card state unstable or removed"]
    CINS_0 = 0x0,
    #[doc = "Card inserted"]
    CINS_1 = 0x01,
}
impl Cins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cins {
    #[inline(always)]
    fn from(val: u8) -> Cins {
        Cins::from_bits(val)
    }
}
impl From<Cins> for u8 {
    #[inline(always)]
    fn from(val: Cins) -> u8 {
        Cins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cinsien {
    #[doc = "Masked"]
    CINSIEN_0 = 0x0,
    #[doc = "Enabled"]
    CINSIEN_1 = 0x01,
}
impl Cinsien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinsien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinsien {
    #[inline(always)]
    fn from(val: u8) -> Cinsien {
        Cinsien::from_bits(val)
    }
}
impl From<Cinsien> for u8 {
    #[inline(always)]
    fn from(val: Cinsien) -> u8 {
        Cinsien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cinssen {
    #[doc = "Masked"]
    CINSSEN_0 = 0x0,
    #[doc = "Enabled"]
    CINSSEN_1 = 0x01,
}
impl Cinssen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinssen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinssen {
    #[inline(always)]
    fn from(val: u8) -> Cinssen {
        Cinssen::from_bits(val)
    }
}
impl From<Cinssen> for u8 {
    #[inline(always)]
    fn from(val: Cinssen) -> u8 {
        Cinssen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cinst {
    #[doc = "Power on Reset or No Card"]
    CINST_0 = 0x0,
    #[doc = "Card Inserted"]
    CINST_1 = 0x01,
}
impl Cinst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinst {
    #[inline(always)]
    fn from(val: u8) -> Cinst {
        Cinst::from_bits(val)
    }
}
impl From<Cinst> for u8 {
    #[inline(always)]
    fn from(val: Cinst) -> u8 {
        Cinst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cint {
    #[doc = "No Card Interrupt"]
    CINT_0 = 0x0,
    #[doc = "Generate Card Interrupt"]
    CINT_1 = 0x01,
}
impl Cint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cint {
    #[inline(always)]
    fn from(val: u8) -> Cint {
        Cint::from_bits(val)
    }
}
impl From<Cint> for u8 {
    #[inline(always)]
    fn from(val: Cint) -> u8 {
        Cint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cintien {
    #[doc = "Masked"]
    CINTIEN_0 = 0x0,
    #[doc = "Enabled"]
    CINTIEN_1 = 0x01,
}
impl Cintien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cintien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cintien {
    #[inline(always)]
    fn from(val: u8) -> Cintien {
        Cintien::from_bits(val)
    }
}
impl From<Cintien> for u8 {
    #[inline(always)]
    fn from(val: Cintien) -> u8 {
        Cintien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cintsen {
    #[doc = "Masked"]
    CINTSEN_0 = 0x0,
    #[doc = "Enabled"]
    CINTSEN_1 = 0x01,
}
impl Cintsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cintsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cintsen {
    #[inline(always)]
    fn from(val: u8) -> Cintsen {
        Cintsen::from_bits(val)
    }
}
impl From<Cintsen> for u8 {
    #[inline(always)]
    fn from(val: Cintsen) -> u8 {
        Cintsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CmdByteEn {
    #[doc = "Disable"]
    CMD_BYTE_EN_0 = 0x0,
    #[doc = "Enable"]
    CMD_BYTE_EN_1 = 0x01,
}
impl CmdByteEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdByteEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdByteEn {
    #[inline(always)]
    fn from(val: u8) -> CmdByteEn {
        CmdByteEn::from_bits(val)
    }
}
impl From<CmdByteEn> for u8 {
    #[inline(always)]
    fn from(val: CmdByteEn) -> u8 {
        CmdByteEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdtyp {
    #[doc = "Normal Other commands"]
    CMDTYP_0 = 0x0,
    #[doc = "Suspend CMD52 for writing Bus Suspend in CCCR"]
    CMDTYP_1 = 0x01,
    #[doc = "Resume CMD52 for writing Function Select in CCCR"]
    CMDTYP_2 = 0x02,
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR"]
    CMDTYP_3 = 0x03,
}
impl Cmdtyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdtyp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdtyp {
    #[inline(always)]
    fn from(val: u8) -> Cmdtyp {
        Cmdtyp::from_bits(val)
    }
}
impl From<Cmdtyp> for u8 {
    #[inline(always)]
    fn from(val: Cmdtyp) -> u8 {
        Cmdtyp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cnibac12e {
    #[doc = "No error"]
    CNIBAC12E_0 = 0x0,
    #[doc = "Not Issued"]
    CNIBAC12E_1 = 0x01,
}
impl Cnibac12e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cnibac12e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cnibac12e {
    #[inline(always)]
    fn from(val: u8) -> Cnibac12e {
        Cnibac12e::from_bits(val)
    }
}
impl From<Cnibac12e> for u8 {
    #[inline(always)]
    fn from(val: Cnibac12e) -> u8 {
        Cnibac12e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CrcChkDis {
    #[doc = "Check CRC16 for every read data packet and check CRC bits for every write data packet"]
    CRC_CHK_DIS_0 = 0x0,
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC bits check for every write data packet"]
    CRC_CHK_DIS_1 = 0x01,
}
impl CrcChkDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcChkDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcChkDis {
    #[inline(always)]
    fn from(val: u8) -> CrcChkDis {
        CrcChkDis::from_bits(val)
    }
}
impl From<CrcChkDis> for u8 {
    #[inline(always)]
    fn from(val: CrcChkDis) -> u8 {
        CrcChkDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Creq {
    #[doc = "No effect"]
    CREQ_0 = 0x0,
    #[doc = "Restart"]
    CREQ_1 = 0x01,
}
impl Creq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Creq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Creq {
    #[inline(always)]
    fn from(val: u8) -> Creq {
        Creq::from_bits(val)
    }
}
impl From<Creq> for u8 {
    #[inline(always)]
    fn from(val: Creq) -> u8 {
        Creq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crm {
    #[doc = "Card state unstable or inserted"]
    CRM_0 = 0x0,
    #[doc = "Card removed"]
    CRM_1 = 0x01,
}
impl Crm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crm {
    #[inline(always)]
    fn from(val: u8) -> Crm {
        Crm::from_bits(val)
    }
}
impl From<Crm> for u8 {
    #[inline(always)]
    fn from(val: Crm) -> u8 {
        Crm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crmien {
    #[doc = "Masked"]
    CRMIEN_0 = 0x0,
    #[doc = "Enabled"]
    CRMIEN_1 = 0x01,
}
impl Crmien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crmien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crmien {
    #[inline(always)]
    fn from(val: u8) -> Crmien {
        Crmien::from_bits(val)
    }
}
impl From<Crmien> for u8 {
    #[inline(always)]
    fn from(val: Crmien) -> u8 {
        Crmien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Crmsen {
    #[doc = "Masked"]
    CRMSEN_0 = 0x0,
    #[doc = "Enabled"]
    CRMSEN_1 = 0x01,
}
impl Crmsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crmsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crmsen {
    #[inline(always)]
    fn from(val: u8) -> Crmsen {
        Crmsen::from_bits(val)
    }
}
impl From<Crmsen> for u8 {
    #[inline(always)]
    fn from(val: Crmsen) -> u8 {
        Crmsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctoe {
    #[doc = "No Error"]
    CTOE_0 = 0x0,
    #[doc = "Time out"]
    CTOE_1 = 0x01,
}
impl Ctoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctoe {
    #[inline(always)]
    fn from(val: u8) -> Ctoe {
        Ctoe::from_bits(val)
    }
}
impl From<Ctoe> for u8 {
    #[inline(always)]
    fn from(val: Ctoe) -> u8 {
        Ctoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctoeien {
    #[doc = "Masked"]
    CTOEIEN_0 = 0x0,
    #[doc = "Enabled"]
    CTOEIEN_1 = 0x01,
}
impl Ctoeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctoeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctoeien {
    #[inline(always)]
    fn from(val: u8) -> Ctoeien {
        Ctoeien::from_bits(val)
    }
}
impl From<Ctoeien> for u8 {
    #[inline(always)]
    fn from(val: Ctoeien) -> u8 {
        Ctoeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ctoesen {
    #[doc = "Masked"]
    CTOESEN_0 = 0x0,
    #[doc = "Enabled"]
    CTOESEN_1 = 0x01,
}
impl Ctoesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctoesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctoesen {
    #[inline(always)]
    fn from(val: u8) -> Ctoesen {
        Ctoesen::from_bits(val)
    }
}
impl From<Ctoesen> for u8 {
    #[inline(always)]
    fn from(val: Ctoesen) -> u8 {
        Ctoesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum D3cd {
    #[doc = "DATA3 does not monitor Card Insertion"]
    D3CD_0 = 0x0,
    #[doc = "DATA3 as Card Detection Pin"]
    D3CD_1 = 0x01,
}
impl D3cd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D3cd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D3cd {
    #[inline(always)]
    fn from(val: u8) -> D3cd {
        D3cd::from_bits(val)
    }
}
impl From<D3cd> for u8 {
    #[inline(always)]
    fn from(val: D3cd) -> u8 {
        D3cd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dce {
    #[doc = "No Error"]
    DCE_0 = 0x0,
    #[doc = "Error"]
    DCE_1 = 0x01,
}
impl Dce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dce {
    #[inline(always)]
    fn from(val: u8) -> Dce {
        Dce::from_bits(val)
    }
}
impl From<Dce> for u8 {
    #[inline(always)]
    fn from(val: Dce) -> u8 {
        Dce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dceien {
    #[doc = "Masked"]
    DCEIEN_0 = 0x0,
    #[doc = "Enabled"]
    DCEIEN_1 = 0x01,
}
impl Dceien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dceien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dceien {
    #[inline(always)]
    fn from(val: u8) -> Dceien {
        Dceien::from_bits(val)
    }
}
impl From<Dceien> for u8 {
    #[inline(always)]
    fn from(val: Dceien) -> u8 {
        Dceien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dcesen {
    #[doc = "Masked"]
    DCESEN_0 = 0x0,
    #[doc = "Enabled"]
    DCESEN_1 = 0x01,
}
impl Dcesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcesen {
    #[inline(always)]
    fn from(val: u8) -> Dcesen {
        Dcesen::from_bits(val)
    }
}
impl From<Dcesen> for u8 {
    #[inline(always)]
    fn from(val: Dcesen) -> u8 {
        Dcesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Debe {
    #[doc = "No Error"]
    DEBE_0 = 0x0,
    #[doc = "Error"]
    DEBE_1 = 0x01,
}
impl Debe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debe {
    #[inline(always)]
    fn from(val: u8) -> Debe {
        Debe::from_bits(val)
    }
}
impl From<Debe> for u8 {
    #[inline(always)]
    fn from(val: Debe) -> u8 {
        Debe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Debeien {
    #[doc = "Masked"]
    DEBEIEN_0 = 0x0,
    #[doc = "Enabled"]
    DEBEIEN_1 = 0x01,
}
impl Debeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debeien {
    #[inline(always)]
    fn from(val: u8) -> Debeien {
        Debeien::from_bits(val)
    }
}
impl From<Debeien> for u8 {
    #[inline(always)]
    fn from(val: Debeien) -> u8 {
        Debeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Debesen {
    #[doc = "Masked"]
    DEBESEN_0 = 0x0,
    #[doc = "Enabled"]
    DEBESEN_1 = 0x01,
}
impl Debesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debesen {
    #[inline(always)]
    fn from(val: u8) -> Debesen {
        Debesen::from_bits(val)
    }
}
impl From<Debesen> for u8 {
    #[inline(always)]
    fn from(val: Debesen) -> u8 {
        Debesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dint {
    #[doc = "No DMA Interrupt"]
    DINT_0 = 0x0,
    #[doc = "DMA Interrupt is generated"]
    DINT_1 = 0x01,
}
impl Dint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dint {
    #[inline(always)]
    fn from(val: u8) -> Dint {
        Dint::from_bits(val)
    }
}
impl From<Dint> for u8 {
    #[inline(always)]
    fn from(val: Dint) -> u8 {
        Dint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dintien {
    #[doc = "Masked"]
    DINTIEN_0 = 0x0,
    #[doc = "Enabled"]
    DINTIEN_1 = 0x01,
}
impl Dintien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dintien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dintien {
    #[inline(always)]
    fn from(val: u8) -> Dintien {
        Dintien::from_bits(val)
    }
}
impl From<Dintien> for u8 {
    #[inline(always)]
    fn from(val: Dintien) -> u8 {
        Dintien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dintsen {
    #[doc = "Masked"]
    DINTSEN_0 = 0x0,
    #[doc = "Enabled"]
    DINTSEN_1 = 0x01,
}
impl Dintsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dintsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dintsen {
    #[inline(always)]
    fn from(val: u8) -> Dintsen {
        Dintsen::from_bits(val)
    }
}
impl From<Dintsen> for u8 {
    #[inline(always)]
    fn from(val: Dintsen) -> u8 {
        Dintsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DisableTimeOut {
    #[doc = "Enable time out"]
    DISABLE_TIME_OUT_0 = 0x0,
    #[doc = "Disable time out"]
    DISABLE_TIME_OUT_1 = 0x01,
}
impl DisableTimeOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisableTimeOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisableTimeOut {
    #[inline(always)]
    fn from(val: u8) -> DisableTimeOut {
        DisableTimeOut::from_bits(val)
    }
}
impl From<DisableTimeOut> for u8 {
    #[inline(always)]
    fn from(val: DisableTimeOut) -> u8 {
        DisableTimeOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dla {
    #[doc = "DATA Line Inactive"]
    DLA_0 = 0x0,
    #[doc = "DATA Line Active"]
    DLA_1 = 0x01,
}
impl Dla {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dla {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dla {
    #[inline(always)]
    fn from(val: u8) -> Dla {
        Dla::from_bits(val)
    }
}
impl From<Dla> for u8 {
    #[inline(always)]
    fn from(val: Dla) -> u8 {
        Dla::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dlsl {
    #[doc = "Data 0 line signal level"]
    DATA0 = 0x0,
    #[doc = "Data 1 line signal level"]
    DATA1 = 0x01,
    #[doc = "Data 2 line signal level"]
    DATA2 = 0x02,
    #[doc = "Data 3 line signal level"]
    DATA3 = 0x03,
    #[doc = "Data 4 line signal level"]
    DATA4 = 0x04,
    #[doc = "Data 5 line signal level"]
    DATA5 = 0x05,
    #[doc = "Data 6 line signal level"]
    DATA6 = 0x06,
    #[doc = "Data 7 line signal level"]
    DATA7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl Dlsl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dlsl {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dlsl {
    #[inline(always)]
    fn from(val: u8) -> Dlsl {
        Dlsl::from_bits(val)
    }
}
impl From<Dlsl> for u8 {
    #[inline(always)]
    fn from(val: Dlsl) -> u8 {
        Dlsl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmae {
    #[doc = "No Error"]
    DMAE_0 = 0x0,
    #[doc = "Error"]
    DMAE_1 = 0x01,
}
impl Dmae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmae {
    #[inline(always)]
    fn from(val: u8) -> Dmae {
        Dmae::from_bits(val)
    }
}
impl From<Dmae> for u8 {
    #[inline(always)]
    fn from(val: Dmae) -> u8 {
        Dmae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmaeien {
    #[doc = "Masked"]
    DMAEIEN_0 = 0x0,
    #[doc = "Enable"]
    DMAEIEN_1 = 0x01,
}
impl Dmaeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaeien {
    #[inline(always)]
    fn from(val: u8) -> Dmaeien {
        Dmaeien::from_bits(val)
    }
}
impl From<Dmaeien> for u8 {
    #[inline(always)]
    fn from(val: Dmaeien) -> u8 {
        Dmaeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmaen {
    #[doc = "Disable"]
    DMAEN_0 = 0x0,
    #[doc = "Enable"]
    DMAEN_1 = 0x01,
}
impl Dmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaen {
    #[inline(always)]
    fn from(val: u8) -> Dmaen {
        Dmaen::from_bits(val)
    }
}
impl From<Dmaen> for u8 {
    #[inline(always)]
    fn from(val: Dmaen) -> u8 {
        Dmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmaesen {
    #[doc = "Masked"]
    DMAESEN_0 = 0x0,
    #[doc = "Enabled"]
    DMAESEN_1 = 0x01,
}
impl Dmaesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaesen {
    #[inline(always)]
    fn from(val: u8) -> Dmaesen {
        Dmaesen::from_bits(val)
    }
}
impl From<Dmaesen> for u8 {
    #[inline(always)]
    fn from(val: Dmaesen) -> u8 {
        Dmaesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmas {
    #[doc = "DMA not supported"]
    DMAS_0 = 0x0,
    #[doc = "DMA Supported"]
    DMAS_1 = 0x01,
}
impl Dmas {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmas {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmas {
    #[inline(always)]
    fn from(val: u8) -> Dmas {
        Dmas::from_bits(val)
    }
}
impl From<Dmas> for u8 {
    #[inline(always)]
    fn from(val: Dmas) -> u8 {
        Dmas::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmasel {
    #[doc = "No DMA or Simple DMA is selected"]
    DMASEL_0 = 0x0,
    #[doc = "ADMA1 is selected"]
    DMASEL_1 = 0x01,
    #[doc = "ADMA2 is selected"]
    DMASEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dmasel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmasel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmasel {
    #[inline(always)]
    fn from(val: u8) -> Dmasel {
        Dmasel::from_bits(val)
    }
}
impl From<Dmasel> for u8 {
    #[inline(always)]
    fn from(val: Dmasel) -> u8 {
        Dmasel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dpsel {
    #[doc = "No Data Present"]
    DPSEL_0 = 0x0,
    #[doc = "Data Present"]
    DPSEL_1 = 0x01,
}
impl Dpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpsel {
    #[inline(always)]
    fn from(val: u8) -> Dpsel {
        Dpsel::from_bits(val)
    }
}
impl From<Dpsel> for u8 {
    #[inline(always)]
    fn from(val: Dpsel) -> u8 {
        Dpsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dtdsel {
    #[doc = "Write (Host to Card)"]
    DTDSEL_0 = 0x0,
    #[doc = "Read (Card to Host)"]
    DTDSEL_1 = 0x01,
}
impl Dtdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtdsel {
    #[inline(always)]
    fn from(val: u8) -> Dtdsel {
        Dtdsel::from_bits(val)
    }
}
impl From<Dtdsel> for u8 {
    #[inline(always)]
    fn from(val: Dtdsel) -> u8 {
        Dtdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dtocv {
    #[doc = "SDCLK x 2 14"]
    DTOCV_0 = 0x0,
    #[doc = "SDCLK x 2 15"]
    DTOCV_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "SDCLK x 2 27"]
    DTOCV_13 = 0x0d,
    #[doc = "SDCLK x 2 28"]
    DTOCV_14 = 0x0e,
    #[doc = "SDCLK x 2 29"]
    DTOCV_15 = 0x0f,
}
impl Dtocv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtocv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtocv {
    #[inline(always)]
    fn from(val: u8) -> Dtocv {
        Dtocv::from_bits(val)
    }
}
impl From<Dtocv> for u8 {
    #[inline(always)]
    fn from(val: Dtocv) -> u8 {
        Dtocv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DtocvAck {
    #[doc = "SDCLK x 2^14"]
    DTOCV_ACK_0 = 0x0,
    #[doc = "SDCLK x 2^15"]
    DTOCV_ACK_1 = 0x01,
    #[doc = "SDCLK x 2^16"]
    DTOCV_ACK_2 = 0x02,
    #[doc = "SDCLK x 2^17"]
    DTOCV_ACK_3 = 0x03,
    #[doc = "SDCLK x 2^18"]
    DTOCV_ACK_4 = 0x04,
    #[doc = "SDCLK x 2^19"]
    DTOCV_ACK_5 = 0x05,
    #[doc = "SDCLK x 2^20"]
    DTOCV_ACK_6 = 0x06,
    #[doc = "SDCLK x 2^21"]
    DTOCV_ACK_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "SDCLK x 2^28"]
    DTOCV_ACK_14 = 0x0e,
    #[doc = "SDCLK x 2^29"]
    DTOCV_ACK_15 = 0x0f,
}
impl DtocvAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtocvAck {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtocvAck {
    #[inline(always)]
    fn from(val: u8) -> DtocvAck {
        DtocvAck::from_bits(val)
    }
}
impl From<DtocvAck> for u8 {
    #[inline(always)]
    fn from(val: DtocvAck) -> u8 {
        DtocvAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dtoe {
    #[doc = "No Error"]
    DTOE_0 = 0x0,
    #[doc = "Time out"]
    DTOE_1 = 0x01,
}
impl Dtoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtoe {
    #[inline(always)]
    fn from(val: u8) -> Dtoe {
        Dtoe::from_bits(val)
    }
}
impl From<Dtoe> for u8 {
    #[inline(always)]
    fn from(val: Dtoe) -> u8 {
        Dtoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dtoeien {
    #[doc = "Masked"]
    DTOEIEN_0 = 0x0,
    #[doc = "Enabled"]
    DTOEIEN_1 = 0x01,
}
impl Dtoeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtoeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtoeien {
    #[inline(always)]
    fn from(val: u8) -> Dtoeien {
        Dtoeien::from_bits(val)
    }
}
impl From<Dtoeien> for u8 {
    #[inline(always)]
    fn from(val: Dtoeien) -> u8 {
        Dtoeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dtoesen {
    #[doc = "Masked"]
    DTOESEN_0 = 0x0,
    #[doc = "Enabled"]
    DTOESEN_1 = 0x01,
}
impl Dtoesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtoesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtoesen {
    #[inline(always)]
    fn from(val: u8) -> Dtoesen {
        Dtoesen::from_bits(val)
    }
}
impl From<Dtoesen> for u8 {
    #[inline(always)]
    fn from(val: Dtoesen) -> u8 {
        Dtoesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dtw {
    #[doc = "1-bit mode"]
    DTW_0 = 0x0,
    #[doc = "4-bit mode"]
    DTW_1 = 0x01,
    #[doc = "8-bit mode"]
    DTW_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dtw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtw {
    #[inline(always)]
    fn from(val: u8) -> Dtw {
        Dtw::from_bits(val)
    }
}
impl From<Dtw> for u8 {
    #[inline(always)]
    fn from(val: Dtw) -> u8 {
        Dtw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dvs {
    #[doc = "Divide-by-1"]
    DVS_0 = 0x0,
    #[doc = "Divide-by-2"]
    DVS_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Divide-by-15"]
    DVS_14 = 0x0e,
    #[doc = "Divide-by-16"]
    DVS_15 = 0x0f,
}
impl Dvs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvs {
    #[inline(always)]
    fn from(val: u8) -> Dvs {
        Dvs::from_bits(val)
    }
}
impl From<Dvs> for u8 {
    #[inline(always)]
    fn from(val: Dvs) -> u8 {
        Dvs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Emode {
    #[doc = "Big Endian Mode"]
    EMODE_0 = 0x0,
    #[doc = "Half Word Big Endian Mode"]
    EMODE_1 = 0x01,
    #[doc = "Little Endian Mode"]
    EMODE_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Emode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emode {
    #[inline(always)]
    fn from(val: u8) -> Emode {
        Emode::from_bits(val)
    }
}
impl From<Emode> for u8 {
    #[inline(always)]
    fn from(val: Emode) -> u8 {
        Emode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExeTune {
    #[doc = "Not Tuned or Tuning Completed"]
    EXE_TUNE_0 = 0x0,
    #[doc = "Execute Tuning"]
    EXE_TUNE_1 = 0x01,
}
impl ExeTune {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExeTune {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExeTune {
    #[inline(always)]
    fn from(val: u8) -> ExeTune {
        ExeTune::from_bits(val)
    }
}
impl From<ExeTune> for u8 {
    #[inline(always)]
    fn from(val: ExeTune) -> u8 {
        ExeTune::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FbclkSel {
    #[doc = "Feedback clock comes from the loopback CLK"]
    FBCLK_SEL_0 = 0x0,
    #[doc = "Feedback clock comes from the ipp_card_clk_out"]
    FBCLK_SEL_1 = 0x01,
}
impl FbclkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FbclkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FbclkSel {
    #[inline(always)]
    fn from(val: u8) -> FbclkSel {
        FbclkSel::from_bits(val)
    }
}
impl From<FbclkSel> for u8 {
    #[inline(always)]
    fn from(val: FbclkSel) -> u8 {
        FbclkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FrcSdclkOn {
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    FRC_SDCLK_ON_0 = 0x0,
    #[doc = "Force CLK active."]
    FRC_SDCLK_ON_1 = 0x01,
}
impl FrcSdclkOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcSdclkOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcSdclkOn {
    #[inline(always)]
    fn from(val: u8) -> FrcSdclkOn {
        FrcSdclkOn::from_bits(val)
    }
}
impl From<FrcSdclkOn> for u8 {
    #[inline(always)]
    fn from(val: FrcSdclkOn) -> u8 {
        FrcSdclkOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hckoff {
    #[doc = "HCLK is active."]
    HCKOFF_0 = 0x0,
    #[doc = "HCLK is gated off."]
    HCKOFF_1 = 0x01,
}
impl Hckoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hckoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hckoff {
    #[inline(always)]
    fn from(val: u8) -> Hckoff {
        Hckoff::from_bits(val)
    }
}
impl From<Hckoff> for u8 {
    #[inline(always)]
    fn from(val: Hckoff) -> u8 {
        Hckoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hss {
    #[doc = "High Speed Not Supported"]
    HSS_0 = 0x0,
    #[doc = "High Speed Supported"]
    HSS_1 = 0x01,
}
impl Hss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hss {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hss {
    #[inline(always)]
    fn from(val: u8) -> Hss {
        Hss::from_bits(val)
    }
}
impl From<Hss> for u8 {
    #[inline(always)]
    fn from(val: Hss) -> u8 {
        Hss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Iabg {
    #[doc = "Disabled"]
    IABG_0 = 0x0,
    #[doc = "Enabled"]
    IABG_1 = 0x01,
}
impl Iabg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iabg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iabg {
    #[inline(always)]
    fn from(val: u8) -> Iabg {
        Iabg::from_bits(val)
    }
}
impl From<Iabg> for u8 {
    #[inline(always)]
    fn from(val: Iabg) -> u8 {
        Iabg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ipgoff {
    #[doc = "IPG_CLK is active."]
    IPGOFF_0 = 0x0,
    #[doc = "IPG_CLK is gated off."]
    IPGOFF_1 = 0x01,
}
impl Ipgoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipgoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipgoff {
    #[inline(always)]
    fn from(val: u8) -> Ipgoff {
        Ipgoff::from_bits(val)
    }
}
impl From<Ipgoff> for u8 {
    #[inline(always)]
    fn from(val: Ipgoff) -> u8 {
        Ipgoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mbl {
    #[doc = "512 bytes"]
    MBL_0 = 0x0,
    #[doc = "1024 bytes"]
    MBL_1 = 0x01,
    #[doc = "2048 bytes"]
    MBL_2 = 0x02,
    #[doc = "4096 bytes"]
    MBL_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbl {
    #[inline(always)]
    fn from(val: u8) -> Mbl {
        Mbl::from_bits(val)
    }
}
impl From<Mbl> for u8 {
    #[inline(always)]
    fn from(val: Mbl) -> u8 {
        Mbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MixCtrlSmpClkSel {
    #[doc = "Fixed clock is used to sample data / cmd"]
    SMP_CLK_SEL_0 = 0x0,
    #[doc = "Tuned clock is used to sample data / cmd"]
    SMP_CLK_SEL_1 = 0x01,
}
impl MixCtrlSmpClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlSmpClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlSmpClkSel {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlSmpClkSel {
        MixCtrlSmpClkSel::from_bits(val)
    }
}
impl From<MixCtrlSmpClkSel> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlSmpClkSel) -> u8 {
        MixCtrlSmpClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Msbsel {
    #[doc = "Single Block"]
    MSBSEL_0 = 0x0,
    #[doc = "Multiple Blocks"]
    MSBSEL_1 = 0x01,
}
impl Msbsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msbsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msbsel {
    #[inline(always)]
    fn from(val: u8) -> Msbsel {
        Msbsel::from_bits(val)
    }
}
impl From<Msbsel> for u8 {
    #[inline(always)]
    fn from(val: Msbsel) -> u8 {
        Msbsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum NonExactBlkRd {
    #[doc = "The block read is exact block read. Host driver doesn't need to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_0 = 0x0,
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    NON_EXACT_BLK_RD_1 = 0x01,
}
impl NonExactBlkRd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NonExactBlkRd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NonExactBlkRd {
    #[inline(always)]
    fn from(val: u8) -> NonExactBlkRd {
        NonExactBlkRd::from_bits(val)
    }
}
impl From<NonExactBlkRd> for u8 {
    #[inline(always)]
    fn from(val: NonExactBlkRd) -> u8 {
        NonExactBlkRd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Peroff {
    #[doc = "IPG_PERCLK is active."]
    PEROFF_0 = 0x0,
    #[doc = "IPG_PERCLK is gated off."]
    PEROFF_1 = 0x01,
}
impl Peroff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Peroff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Peroff {
    #[inline(always)]
    fn from(val: u8) -> Peroff {
        Peroff::from_bits(val)
    }
}
impl From<Peroff> for u8 {
    #[inline(always)]
    fn from(val: Peroff) -> u8 {
        Peroff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RdNo8clkEn {
    #[doc = "Disable S/W RD_DONE_NO_8CLK, uSHDC determines if 8 clocks are needed automatically."]
    RD_NO8CLK_EN_0 = 0x0,
    #[doc = "S/W RD_DONE_NO_8CLK is enabled."]
    RD_NO8CLK_EN_1 = 0x01,
}
impl RdNo8clkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RdNo8clkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RdNo8clkEn {
    #[inline(always)]
    fn from(val: u8) -> RdNo8clkEn {
        RdNo8clkEn::from_bits(val)
    }
}
impl From<RdNo8clkEn> for u8 {
    #[inline(always)]
    fn from(val: RdNo8clkEn) -> u8 {
        RdNo8clkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RetuningMode {
    #[doc = "Mode 1"]
    RETUNING_MODE_0 = 0x0,
    #[doc = "Mode 2"]
    RETUNING_MODE_1 = 0x01,
    #[doc = "Mode 3"]
    RETUNING_MODE_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RetuningMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RetuningMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RetuningMode {
    #[inline(always)]
    fn from(val: u8) -> RetuningMode {
        RetuningMode::from_bits(val)
    }
}
impl From<RetuningMode> for u8 {
    #[inline(always)]
    fn from(val: RetuningMode) -> u8 {
        RetuningMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rsptyp {
    #[doc = "No Response"]
    RSPTYP_0 = 0x0,
    #[doc = "Response Length 136"]
    RSPTYP_1 = 0x01,
    #[doc = "Response Length 48"]
    RSPTYP_2 = 0x02,
    #[doc = "Response Length 48, check Busy after response"]
    RSPTYP_3 = 0x03,
}
impl Rsptyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsptyp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsptyp {
    #[inline(always)]
    fn from(val: u8) -> Rsptyp {
        Rsptyp::from_bits(val)
    }
}
impl From<Rsptyp> for u8 {
    #[inline(always)]
    fn from(val: Rsptyp) -> u8 {
        Rsptyp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rsta {
    #[doc = "No Reset"]
    RSTA_0 = 0x0,
    #[doc = "Reset"]
    RSTA_1 = 0x01,
}
impl Rsta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsta {
    #[inline(always)]
    fn from(val: u8) -> Rsta {
        Rsta::from_bits(val)
    }
}
impl From<Rsta> for u8 {
    #[inline(always)]
    fn from(val: Rsta) -> u8 {
        Rsta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rstc {
    #[doc = "No Reset"]
    RSTC_0 = 0x0,
    #[doc = "Reset"]
    RSTC_1 = 0x01,
}
impl Rstc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstc {
    #[inline(always)]
    fn from(val: u8) -> Rstc {
        Rstc::from_bits(val)
    }
}
impl From<Rstc> for u8 {
    #[inline(always)]
    fn from(val: Rstc) -> u8 {
        Rstc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rstd {
    #[doc = "No Reset"]
    RSTD_0 = 0x0,
    #[doc = "Reset"]
    RSTD_1 = 0x01,
}
impl Rstd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstd {
    #[inline(always)]
    fn from(val: u8) -> Rstd {
        Rstd::from_bits(val)
    }
}
impl From<Rstd> for u8 {
    #[inline(always)]
    fn from(val: Rstd) -> u8 {
        Rstd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rta {
    #[doc = "No valid data"]
    RTA_0 = 0x0,
    #[doc = "Transferring data"]
    RTA_1 = 0x01,
}
impl Rta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rta {
    #[inline(always)]
    fn from(val: u8) -> Rta {
        Rta::from_bits(val)
    }
}
impl From<Rta> for u8 {
    #[inline(always)]
    fn from(val: Rta) -> u8 {
        Rta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rte {
    #[doc = "Re-Tuning is not required"]
    RTE_0 = 0x0,
    #[doc = "Re-Tuning should be performed"]
    RTE_1 = 0x01,
}
impl Rte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rte {
    #[inline(always)]
    fn from(val: u8) -> Rte {
        Rte::from_bits(val)
    }
}
impl From<Rte> for u8 {
    #[inline(always)]
    fn from(val: Rte) -> u8 {
        Rte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rteien {
    #[doc = "Masked"]
    RTEIEN_0 = 0x0,
    #[doc = "Enabled"]
    RTEIEN_1 = 0x01,
}
impl Rteien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rteien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rteien {
    #[inline(always)]
    fn from(val: u8) -> Rteien {
        Rteien::from_bits(val)
    }
}
impl From<Rteien> for u8 {
    #[inline(always)]
    fn from(val: Rteien) -> u8 {
        Rteien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rtesen {
    #[doc = "Masked"]
    RTESEN_0 = 0x0,
    #[doc = "Enabled"]
    RTESEN_1 = 0x01,
}
impl Rtesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtesen {
    #[inline(always)]
    fn from(val: u8) -> Rtesen {
        Rtesen::from_bits(val)
    }
}
impl From<Rtesen> for u8 {
    #[inline(always)]
    fn from(val: Rtesen) -> u8 {
        Rtesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rtr {
    #[doc = "Fixed or well tuned sampling clock"]
    RTR_0 = 0x0,
    #[doc = "Sampling clock needs re-tuning"]
    RTR_1 = 0x01,
}
impl Rtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtr {
    #[inline(always)]
    fn from(val: u8) -> Rtr {
        Rtr::from_bits(val)
    }
}
impl From<Rtr> for u8 {
    #[inline(always)]
    fn from(val: Rtr) -> u8 {
        Rtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rwctl {
    #[doc = "Disable Read Wait Control, and stop SD Clock at block gap when SABGREQ bit is set"]
    RWCTL_0 = 0x0,
    #[doc = "Enable Read Wait Control, and assert Read Wait without stopping SD Clock at block gap when SABGREQ bit is set"]
    RWCTL_1 = 0x01,
}
impl Rwctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwctl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwctl {
    #[inline(always)]
    fn from(val: u8) -> Rwctl {
        Rwctl::from_bits(val)
    }
}
impl From<Rwctl> for u8 {
    #[inline(always)]
    fn from(val: Rwctl) -> u8 {
        Rwctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sabgreq {
    #[doc = "Transfer"]
    SABGREQ_0 = 0x0,
    #[doc = "Stop"]
    SABGREQ_1 = 0x01,
}
impl Sabgreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sabgreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sabgreq {
    #[inline(always)]
    fn from(val: u8) -> Sabgreq {
        Sabgreq::from_bits(val)
    }
}
impl From<Sabgreq> for u8 {
    #[inline(always)]
    fn from(val: Sabgreq) -> u8 {
        Sabgreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sdoff {
    #[doc = "SD Clock is active."]
    SDOFF_0 = 0x0,
    #[doc = "SD Clock is gated off."]
    SDOFF_1 = 0x01,
}
impl Sdoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdoff {
    #[inline(always)]
    fn from(val: u8) -> Sdoff {
        Sdoff::from_bits(val)
    }
}
impl From<Sdoff> for u8 {
    #[inline(always)]
    fn from(val: Sdoff) -> u8 {
        Sdoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sdstb {
    #[doc = "Clock is changing frequency and not stable."]
    SDSTB_0 = 0x0,
    #[doc = "Clock is stable."]
    SDSTB_1 = 0x01,
}
impl Sdstb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdstb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdstb {
    #[inline(always)]
    fn from(val: u8) -> Sdstb {
        Sdstb::from_bits(val)
    }
}
impl From<Sdstb> for u8 {
    #[inline(always)]
    fn from(val: Sdstb) -> u8 {
        Sdstb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Srs {
    #[doc = "Not supported"]
    SRS_0 = 0x0,
    #[doc = "Supported"]
    SRS_1 = 0x01,
}
impl Srs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srs {
    #[inline(always)]
    fn from(val: u8) -> Srs {
        Srs::from_bits(val)
    }
}
impl From<Srs> for u8 {
    #[inline(always)]
    fn from(val: Srs) -> u8 {
        Srs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tc {
    #[doc = "Transfer not complete"]
    TC_0 = 0x0,
    #[doc = "Transfer complete"]
    TC_1 = 0x01,
}
impl Tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tc {
    #[inline(always)]
    fn from(val: u8) -> Tc {
        Tc::from_bits(val)
    }
}
impl From<Tc> for u8 {
    #[inline(always)]
    fn from(val: Tc) -> u8 {
        Tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tcien {
    #[doc = "Masked"]
    TCIEN_0 = 0x0,
    #[doc = "Enabled"]
    TCIEN_1 = 0x01,
}
impl Tcien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcien {
    #[inline(always)]
    fn from(val: u8) -> Tcien {
        Tcien::from_bits(val)
    }
}
impl From<Tcien> for u8 {
    #[inline(always)]
    fn from(val: Tcien) -> u8 {
        Tcien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tcsen {
    #[doc = "Masked"]
    TCSEN_0 = 0x0,
    #[doc = "Enabled"]
    TCSEN_1 = 0x01,
}
impl Tcsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsen {
    #[inline(always)]
    fn from(val: u8) -> Tcsen {
        Tcsen::from_bits(val)
    }
}
impl From<Tcsen> for u8 {
    #[inline(always)]
    fn from(val: Tcsen) -> u8 {
        Tcsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tneien {
    #[doc = "Masked"]
    TNEIEN_0 = 0x0,
    #[doc = "Enabled"]
    TNEIEN_1 = 0x01,
}
impl Tneien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tneien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tneien {
    #[inline(always)]
    fn from(val: u8) -> Tneien {
        Tneien::from_bits(val)
    }
}
impl From<Tneien> for u8 {
    #[inline(always)]
    fn from(val: Tneien) -> u8 {
        Tneien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tnesen {
    #[doc = "Masked"]
    TNESEN_0 = 0x0,
    #[doc = "Enabled"]
    TNESEN_1 = 0x01,
}
impl Tnesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnesen {
    #[inline(always)]
    fn from(val: u8) -> Tnesen {
        Tnesen::from_bits(val)
    }
}
impl From<Tnesen> for u8 {
    #[inline(always)]
    fn from(val: Tnesen) -> u8 {
        Tnesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tpien {
    #[doc = "Masked"]
    TPIEN_0 = 0x0,
    #[doc = "Enabled"]
    TPIEN_1 = 0x01,
}
impl Tpien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpien {
    #[inline(always)]
    fn from(val: u8) -> Tpien {
        Tpien::from_bits(val)
    }
}
impl From<Tpien> for u8 {
    #[inline(always)]
    fn from(val: Tpien) -> u8 {
        Tpien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tpsen {
    #[doc = "Masked"]
    TPSEN_0 = 0x0,
    #[doc = "Enabled"]
    TPSEN_1 = 0x01,
}
impl Tpsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsen {
    #[inline(always)]
    fn from(val: u8) -> Tpsen {
        Tpsen::from_bits(val)
    }
}
impl From<Tpsen> for u8 {
    #[inline(always)]
    fn from(val: Tpsen) -> u8 {
        Tpsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tscd {
    #[doc = "Delay cell select change is not finished."]
    TSCD_0 = 0x0,
    #[doc = "Delay cell select change is finished."]
    TSCD_1 = 0x01,
}
impl Tscd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tscd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tscd {
    #[inline(always)]
    fn from(val: u8) -> Tscd {
        Tscd::from_bits(val)
    }
}
impl From<Tscd> for u8 {
    #[inline(always)]
    fn from(val: Tscd) -> u8 {
        Tscd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TuningCmdEn {
    #[doc = "Auto tuning circuit does not check the CMD line."]
    TUNING_CMD_EN_0 = 0x0,
    #[doc = "Auto tuning circuit checks the CMD line."]
    TUNING_CMD_EN_1 = 0x01,
}
impl TuningCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TuningCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TuningCmdEn {
    #[inline(always)]
    fn from(val: u8) -> TuningCmdEn {
        TuningCmdEn::from_bits(val)
    }
}
impl From<TuningCmdEn> for u8 {
    #[inline(always)]
    fn from(val: TuningCmdEn) -> u8 {
        TuningCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UseTuningSdr50 {
    #[doc = "SDR does not require tuning"]
    USE_TUNING_SDR50_0 = 0x0,
    #[doc = "SDR50 requires tuning"]
    USE_TUNING_SDR50_1 = 0x01,
}
impl UseTuningSdr50 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UseTuningSdr50 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UseTuningSdr50 {
    #[inline(always)]
    fn from(val: u8) -> UseTuningSdr50 {
        UseTuningSdr50::from_bits(val)
    }
}
impl From<UseTuningSdr50> for u8 {
    #[inline(always)]
    fn from(val: UseTuningSdr50) -> u8 {
        UseTuningSdr50::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vs18 {
    #[doc = "1.8V not supported"]
    VS18_0 = 0x0,
    #[doc = "1.8V supported"]
    VS18_1 = 0x01,
}
impl Vs18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs18 {
    #[inline(always)]
    fn from(val: u8) -> Vs18 {
        Vs18::from_bits(val)
    }
}
impl From<Vs18> for u8 {
    #[inline(always)]
    fn from(val: Vs18) -> u8 {
        Vs18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vs30 {
    #[doc = "3.0V not supported"]
    VS30_0 = 0x0,
    #[doc = "3.0V supported"]
    VS30_1 = 0x01,
}
impl Vs30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs30 {
    #[inline(always)]
    fn from(val: u8) -> Vs30 {
        Vs30::from_bits(val)
    }
}
impl From<Vs30> for u8 {
    #[inline(always)]
    fn from(val: Vs30) -> u8 {
        Vs30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vs33 {
    #[doc = "3.3V not supported"]
    VS33_0 = 0x0,
    #[doc = "3.3V supported"]
    VS33_1 = 0x01,
}
impl Vs33 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs33 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs33 {
    #[inline(always)]
    fn from(val: u8) -> Vs33 {
        Vs33::from_bits(val)
    }
}
impl From<Vs33> for u8 {
    #[inline(always)]
    fn from(val: Vs33) -> u8 {
        Vs33::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vselect {
    #[doc = "Change the voltage to high voltage range, around 3.0 V"]
    VSELECT_0 = 0x0,
    #[doc = "Change the voltage to low voltage range, around 1.8 V"]
    VSELECT_1 = 0x01,
}
impl Vselect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vselect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vselect {
    #[inline(always)]
    fn from(val: u8) -> Vselect {
        Vselect::from_bits(val)
    }
}
impl From<Vselect> for u8 {
    #[inline(always)]
    fn from(val: Vselect) -> u8 {
        Vselect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wecins {
    #[doc = "Disable"]
    WECINS_0 = 0x0,
    #[doc = "Enable"]
    WECINS_1 = 0x01,
}
impl Wecins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wecins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wecins {
    #[inline(always)]
    fn from(val: u8) -> Wecins {
        Wecins::from_bits(val)
    }
}
impl From<Wecins> for u8 {
    #[inline(always)]
    fn from(val: Wecins) -> u8 {
        Wecins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wecint {
    #[doc = "Disable"]
    WECINT_0 = 0x0,
    #[doc = "Enable"]
    WECINT_1 = 0x01,
}
impl Wecint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wecint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wecint {
    #[inline(always)]
    fn from(val: u8) -> Wecint {
        Wecint::from_bits(val)
    }
}
impl From<Wecint> for u8 {
    #[inline(always)]
    fn from(val: Wecint) -> u8 {
        Wecint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wecrm {
    #[doc = "Disable"]
    WECRM_0 = 0x0,
    #[doc = "Enable"]
    WECRM_1 = 0x01,
}
impl Wecrm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wecrm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wecrm {
    #[inline(always)]
    fn from(val: u8) -> Wecrm {
        Wecrm::from_bits(val)
    }
}
impl From<Wecrm> for u8 {
    #[inline(always)]
    fn from(val: Wecrm) -> u8 {
        Wecrm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wpspl {
    #[doc = "Write protected (WP = 1)"]
    WPSPL_0 = 0x0,
    #[doc = "Write enabled (WP = 0)"]
    WPSPL_1 = 0x01,
}
impl Wpspl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wpspl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wpspl {
    #[inline(always)]
    fn from(val: u8) -> Wpspl {
        Wpspl::from_bits(val)
    }
}
impl From<Wpspl> for u8 {
    #[inline(always)]
    fn from(val: Wpspl) -> u8 {
        Wpspl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wta {
    #[doc = "No valid data"]
    WTA_0 = 0x0,
    #[doc = "Transferring data"]
    WTA_1 = 0x01,
}
impl Wta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wta {
    #[inline(always)]
    fn from(val: u8) -> Wta {
        Wta::from_bits(val)
    }
}
impl From<Wta> for u8 {
    #[inline(always)]
    fn from(val: Wta) -> u8 {
        Wta::to_bits(val)
    }
}
