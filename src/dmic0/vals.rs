#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dcpole {
    #[doc = "Flat response, no filter."]
    FLAT_RESPONSE = 0x0,
    #[doc = "155 Hz."]
    HZ_155 = 0x01,
    #[doc = "78 Hz."]
    HZ_78 = 0x02,
    #[doc = "39 Hz"]
    HZ_39 = 0x03,
}
impl Dcpole {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcpole {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcpole {
    #[inline(always)]
    fn from(val: u8) -> Dcpole {
        Dcpole::from_bits(val)
    }
}
impl From<Dcpole> for u8 {
    #[inline(always)]
    fn from(val: Dcpole) -> u8 {
        Dcpole::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Decreset(pub u8);
impl Decreset {
    #[doc = "release reset to decimator"]
    pub const RELEASE_RESET: Self = Self(0x0);
    #[doc = "assert reset to decimator Note : resets are applied in pairs. So bit 0 corresponds to channels 0/1, bit1 corresponds to channels 2/3, bit2 to channel 4/5 and bit3 to channel 6/7"]
    pub const ASSERT_RESET: Self = Self(0x01);
}
impl Decreset {
    pub const fn from_bits(val: u8) -> Decreset {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for Decreset {
    #[inline(always)]
    fn from(val: u8) -> Decreset {
        Decreset::from_bits(val)
    }
}
impl From<Decreset> for u8 {
    #[inline(always)]
    fn from(val: Decreset) -> u8 {
        Decreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dmaen {
    #[doc = "DMA requests are not enabled."]
    DISABLED = 0x0,
    #[doc = "DMA requests based on FIFO level are enabled."]
    ENABLED = 0x01,
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
pub enum Enable {
    #[doc = "FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    DISABLED = 0x0,
    #[doc = "FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    ENABLED = 0x01,
}
impl Enable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable {
    #[inline(always)]
    fn from(val: u8) -> Enable {
        Enable::from_bits(val)
    }
}
impl From<Enable> for u8 {
    #[inline(always)]
    fn from(val: Enable) -> u8 {
        Enable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hpfs {
    #[doc = "First filter by-pass."]
    BYPASS = 0x0,
    #[doc = "High pass filter with -3dB cut-off at 1750Hz."]
    HIGH_PASS_1750HZ = 0x01,
    #[doc = "High pass filter with -3dB cut-off at 215Hz."]
    HIGH_PASS_215HZ = 0x02,
    _RESERVED_3 = 0x03,
}
impl Hpfs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hpfs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hpfs {
    #[inline(always)]
    fn from(val: u8) -> Hpfs {
        Hpfs::from_bits(val)
    }
}
impl From<Hpfs> for u8 {
    #[inline(always)]
    fn from(val: Hpfs) -> u8 {
        Hpfs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Inten {
    #[doc = "FIFO level interrupts are not enabled."]
    DISABLED = 0x0,
    #[doc = "FIFO level interrupts are enabled."]
    ENABLED = 0x01,
}
impl Inten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inten {
    #[inline(always)]
    fn from(val: u8) -> Inten {
        Inten::from_bits(val)
    }
}
impl From<Inten> for u8 {
    #[inline(always)]
    fn from(val: Inten) -> u8 {
        Inten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PhyFall {
    #[doc = "Capture PDM_DATA on the rising edge of PDM_CLK."]
    RISING_EDGE = 0x0,
    #[doc = "Capture PDM_DATA on the falling edge of PDM_CLK."]
    FALLING_EDGE = 0x01,
}
impl PhyFall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyFall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyFall {
    #[inline(always)]
    fn from(val: u8) -> PhyFall {
        PhyFall::from_bits(val)
    }
}
impl From<PhyFall> for u8 {
    #[inline(always)]
    fn from(val: PhyFall) -> u8 {
        PhyFall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PhyHalf {
    #[doc = "Standard half rate sampling. The clock to the DMIC is sent at the same rate as the decimator is providing."]
    STANDARD = 0x0,
    #[doc = "Use half rate sampling. The clock to the DMIC is sent at half the rate as the decimator is providing."]
    HALF_RATE = 0x01,
}
impl PhyHalf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhyHalf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhyHalf {
    #[inline(always)]
    fn from(val: u8) -> PhyHalf {
        PhyHalf::from_bits(val)
    }
}
impl From<PhyHalf> for u8 {
    #[inline(always)]
    fn from(val: PhyHalf) -> u8 {
        PhyHalf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resetn {
    #[doc = "Reset the FIFO."]
    RESET = 0x0,
    #[doc = "Normal operation"]
    NORMAL = 0x01,
}
impl Resetn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resetn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resetn {
    #[inline(always)]
    fn from(val: u8) -> Resetn {
        Resetn::from_bits(val)
    }
}
impl From<Resetn> for u8 {
    #[inline(always)]
    fn from(val: Resetn) -> u8 {
        Resetn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Saturateat16bit {
    #[doc = "Results roll over if out range and do not saturate."]
    DO_NOT_SATURATE = 0x0,
    #[doc = "If the result overflows, it saturates at 0xFFFF for positive overflow and 0x8000 for negative overflow."]
    SATURATE = 0x01,
}
impl Saturateat16bit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Saturateat16bit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Saturateat16bit {
    #[inline(always)]
    fn from(val: u8) -> Saturateat16bit {
        Saturateat16bit::from_bits(val)
    }
}
impl From<Saturateat16bit> for u8 {
    #[inline(always)]
    fn from(val: Saturateat16bit) -> u8 {
        Saturateat16bit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Signextend {
    #[doc = "The top byte of the FIFODATA register is always 0."]
    DO_NOT_SIGNEXTEND = 0x0,
    #[doc = "The top byte of the FIFODATA register is sign extended. This allows processing of 24-bit audio data on 32-bit machines."]
    SIGNEXTEND = 0x01,
}
impl Signextend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Signextend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Signextend {
    #[inline(always)]
    fn from(val: u8) -> Signextend {
        Signextend::from_bits(val)
    }
}
impl From<Signextend> for u8 {
    #[inline(always)]
    fn from(val: Signextend) -> u8 {
        Signextend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum St10 {
    #[doc = "Normal operation, waiting for HWVAD trigger event (stage 0)."]
    NORMAL = 0x0,
    #[doc = "Reset internal interrupt flag by writing a '1' pulse."]
    RESET = 0x01,
}
impl St10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> St10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for St10 {
    #[inline(always)]
    fn from(val: u8) -> St10 {
        St10::from_bits(val)
    }
}
impl From<St10> for u8 {
    #[inline(always)]
    fn from(val: St10) -> u8 {
        St10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Use2fs {
    #[doc = "Use 1FS output for PCM data."]
    USE_1FS = 0x0,
    #[doc = "Use 2FS output for PCM data."]
    USE_2FS = 0x01,
}
impl Use2fs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Use2fs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Use2fs {
    #[inline(always)]
    fn from(val: u8) -> Use2fs {
        Use2fs::from_bits(val)
    }
}
impl From<Use2fs> for u8 {
    #[inline(always)]
    fn from(val: Use2fs) -> u8 {
        Use2fs::to_bits(val)
    }
}
