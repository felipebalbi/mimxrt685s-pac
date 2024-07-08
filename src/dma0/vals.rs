#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Abort32 {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Aborts DMA operations on channel 32."]
    EFFECT = 0x01,
}
impl Abort32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abort32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abort32 {
    #[inline(always)]
    fn from(val: u8) -> Abort32 {
        Abort32::from_bits(val)
    }
}
impl From<Abort32> for u8 {
    #[inline(always)]
    fn from(val: Abort32) -> u8 {
        Abort32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Abort6333(pub u32);
impl Abort6333 {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Aborts DMA operations on the relevant channel."]
    pub const EFFECT: Self = Self(0x01);
}
impl Abort6333 {
    pub const fn from_bits(val: u32) -> Abort6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Abort6333 {
    #[inline(always)]
    fn from(val: u32) -> Abort6333 {
        Abort6333::from_bits(val)
    }
}
impl From<Abort6333> for u32 {
    #[inline(always)]
    fn from(val: Abort6333) -> u32 {
        Abort6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Abortctrl(pub u32);
impl Abortctrl {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Aborts DMA operations on channel 0."]
    pub const EFFECT: Self = Self(0x01);
}
impl Abortctrl {
    pub const fn from_bits(val: u32) -> Abortctrl {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Abortctrl {
    #[inline(always)]
    fn from(val: u32) -> Abortctrl {
        Abortctrl::from_bits(val)
    }
}
impl From<Abortctrl> for u32 {
    #[inline(always)]
    fn from(val: Abortctrl) -> u32 {
        Abortctrl::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Act(pub u32);
impl Act {
    #[doc = "DMAchannel 0 is not active."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "DMAchannel 0 is active."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Act {
    pub const fn from_bits(val: u32) -> Act {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Act {
    #[inline(always)]
    fn from(val: u32) -> Act {
        Act::from_bits(val)
    }
}
impl From<Act> for u32 {
    #[inline(always)]
    fn from(val: Act) -> u32 {
        Act::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Active32 {
    #[doc = "DMAchannel 32 is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "DMAchannel 32 is active."]
    ACTIVE = 0x01,
}
impl Active32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active32 {
    #[inline(always)]
    fn from(val: u8) -> Active32 {
        Active32::from_bits(val)
    }
}
impl From<Active32> for u8 {
    #[inline(always)]
    fn from(val: Active32) -> u8 {
        Active32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Active6333(pub u32);
impl Active6333 {
    #[doc = "The relevant DMA channel is not active."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The relevant DMA channel is active."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Active6333 {
    pub const fn from_bits(val: u32) -> Active6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Active6333 {
    #[inline(always)]
    fn from(val: u32) -> Active6333 {
        Active6333::from_bits(val)
    }
}
impl From<Active6333> for u32 {
    #[inline(always)]
    fn from(val: Active6333) -> u32 {
        Active6333::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Activeerrint {
    #[doc = "Not pending. No error interrupts are pending."]
    NOT_PENDING = 0x0,
    #[doc = "Pending. At least one error interrupt is pending."]
    PENDING = 0x01,
}
impl Activeerrint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Activeerrint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Activeerrint {
    #[inline(always)]
    fn from(val: u8) -> Activeerrint {
        Activeerrint::from_bits(val)
    }
}
impl From<Activeerrint> for u8 {
    #[inline(always)]
    fn from(val: Activeerrint) -> u8 {
        Activeerrint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Activeint {
    #[doc = "Not pending. No enabled interrupts are pending."]
    NOT_PENDING = 0x0,
    #[doc = "Pending. At least one enabled interrupt is pending."]
    PENDING = 0x01,
}
impl Activeint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Activeint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Activeint {
    #[inline(always)]
    fn from(val: u8) -> Activeint {
        Activeint::from_bits(val)
    }
}
impl From<Activeint> for u8 {
    #[inline(always)]
    fn from(val: Activeint) -> u8 {
        Activeint::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bsy(pub u32);
impl Bsy {
    #[doc = "DMAchannel 0 is not busy."]
    pub const NOT_BUSY: Self = Self(0x0);
    #[doc = "DMAchannel 0 is busy."]
    pub const BUSY: Self = Self(0x01);
}
impl Bsy {
    pub const fn from_bits(val: u32) -> Bsy {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Bsy {
    #[inline(always)]
    fn from(val: u32) -> Bsy {
        Bsy::from_bits(val)
    }
}
impl From<Bsy> for u32 {
    #[inline(always)]
    fn from(val: Bsy) -> u32 {
        Bsy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Busy32 {
    #[doc = "DMAchannel 32 is not busy."]
    NOT_BUSY = 0x0,
    #[doc = "DMAchannel 0 is busy."]
    BUSY = 0x01,
}
impl Busy32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busy32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busy32 {
    #[inline(always)]
    fn from(val: u8) -> Busy32 {
        Busy32::from_bits(val)
    }
}
impl From<Busy32> for u8 {
    #[inline(always)]
    fn from(val: Busy32) -> u8 {
        Busy32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Busy6333(pub u32);
impl Busy6333 {
    #[doc = "The relevant DMA channel is not busy."]
    pub const NOT_BUSY: Self = Self(0x0);
    #[doc = "The relevant DMA channel is busy."]
    pub const BUSY: Self = Self(0x01);
}
impl Busy6333 {
    pub const fn from_bits(val: u32) -> Busy6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Busy6333 {
    #[inline(always)]
    fn from(val: u32) -> Busy6333 {
        Busy6333::from_bits(val)
    }
}
impl From<Busy6333> for u32 {
    #[inline(always)]
    fn from(val: Busy6333) -> u32 {
        Busy6333::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cfgvalid {
    #[doc = "Not valid. The channel descriptor is not considered valid until validated by an associated SETVALID0 setting."]
    NOT_VALID = 0x0,
    #[doc = "Valid. The current channel descriptor is considered valid."]
    VALID = 0x01,
}
impl Cfgvalid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfgvalid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfgvalid {
    #[inline(always)]
    fn from(val: u8) -> Cfgvalid {
        Cfgvalid::from_bits(val)
    }
}
impl From<Cfgvalid> for u8 {
    #[inline(always)]
    fn from(val: Cfgvalid) -> u8 {
        Cfgvalid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Clrtrig {
    #[doc = "Not cleared. The trigger is not cleared when this descriptor is exhausted. If there is a reload, the next descriptor will be started."]
    NOT_CLEARED = 0x0,
    #[doc = "Cleared. The trigger is cleared when this descriptor is exhausted"]
    CLEARED = 0x01,
}
impl Clrtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrtrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrtrig {
    #[inline(always)]
    fn from(val: u8) -> Clrtrig {
        Clrtrig::from_bits(val)
    }
}
impl From<Clrtrig> for u8 {
    #[inline(always)]
    fn from(val: Clrtrig) -> u8 {
        Clrtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum CtlstatTrig {
    #[doc = "Not triggered. The trigger for this DMA channel is not set. DMA operations will not be carried out."]
    NOT_TRIGGERED = 0x0,
    #[doc = "Triggered. The trigger for this DMA channel is set. DMA operations will be carried out."]
    TRIGGERED = 0x01,
}
impl CtlstatTrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtlstatTrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtlstatTrig {
    #[inline(always)]
    fn from(val: u8) -> CtlstatTrig {
        CtlstatTrig::from_bits(val)
    }
}
impl From<CtlstatTrig> for u8 {
    #[inline(always)]
    fn from(val: CtlstatTrig) -> u8 {
        CtlstatTrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dstburstwrap {
    #[doc = "Disabled. Destination burst wrapping is not enabled for this DMA channel."]
    DISABLED = 0x0,
    #[doc = "Enabled. Destination burst wrapping is enabled for this DMA channel."]
    ENABLED = 0x01,
}
impl Dstburstwrap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dstburstwrap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dstburstwrap {
    #[inline(always)]
    fn from(val: u8) -> Dstburstwrap {
        Dstburstwrap::from_bits(val)
    }
}
impl From<Dstburstwrap> for u8 {
    #[inline(always)]
    fn from(val: Dstburstwrap) -> u8 {
        Dstburstwrap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dstinc {
    #[doc = "No increment. The destination address is not incremented for each transfer. This is the usual case when the destination is a peripheral device."]
    NO_INCREMENT = 0x0,
    #[doc = "1 x width. The destination address is incremented by the amount specified by Width for each transfer. This is the usual case when the destination is memory."]
    WIDTH_X_1 = 0x01,
    #[doc = "2 x width. The destination address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 0x02,
    #[doc = "4 x width. The destination address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 0x03,
}
impl Dstinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dstinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dstinc {
    #[inline(always)]
    fn from(val: u8) -> Dstinc {
        Dstinc::from_bits(val)
    }
}
impl From<Dstinc> for u8 {
    #[inline(always)]
    fn from(val: Dstinc) -> u8 {
        Dstinc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ena(pub u32);
impl Ena {
    #[doc = "DMAchannel 0 is disabled."]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "DMAchannel 0 is enabled."]
    pub const ENABLED: Self = Self(0x01);
}
impl Ena {
    pub const fn from_bits(val: u32) -> Ena {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Ena {
    #[inline(always)]
    fn from(val: u32) -> Ena {
        Ena::from_bits(val)
    }
}
impl From<Ena> for u32 {
    #[inline(always)]
    fn from(val: Ena) -> u32 {
        Ena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Enable {
    #[doc = "Disabled. The DMA controller is disabled. This clears any triggers that were asserted at the point when disabled, but does not prevent re-triggering when the DMA controller is re-enabled."]
    DISABLED = 0x0,
    #[doc = "Enabled. The DMA controller is enabled."]
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
pub enum Enable32 {
    #[doc = "DMAchannel 32 is disabled."]
    DISABLED = 0x0,
    #[doc = "DMAchannel 32 is enabled."]
    ENABLED = 0x01,
}
impl Enable32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enable32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enable32 {
    #[inline(always)]
    fn from(val: u8) -> Enable32 {
        Enable32::from_bits(val)
    }
}
impl From<Enable32> for u8 {
    #[inline(always)]
    fn from(val: Enable32) -> u8 {
        Enable32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enable6333(pub u32);
impl Enable6333 {
    #[doc = "The relevant DMA channel is disabled."]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "The relevant DMA channel is enabled."]
    pub const ENABLED: Self = Self(0x01);
}
impl Enable6333 {
    pub const fn from_bits(val: u32) -> Enable6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Enable6333 {
    #[inline(always)]
    fn from(val: u32) -> Enable6333 {
        Enable6333::from_bits(val)
    }
}
impl From<Enable6333> for u32 {
    #[inline(always)]
    fn from(val: Enable6333) -> u32 {
        Enable6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Err(pub u32);
impl Err {
    #[doc = "The Error Interrupt is not active for DMA channel 0."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The Error Interrupt is pending for DMA channel 0."]
    pub const PENDING: Self = Self(0x01);
}
impl Err {
    pub const fn from_bits(val: u32) -> Err {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Err {
    #[inline(always)]
    fn from(val: u32) -> Err {
        Err::from_bits(val)
    }
}
impl From<Err> for u32 {
    #[inline(always)]
    fn from(val: Err) -> u32 {
        Err::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Err32 {
    #[doc = "The Error Interrupt is not active for DMA channel 32."]
    NOT_ACTIVE = 0x0,
    #[doc = "The Error Interrupt is pending for DMA channel 32."]
    PENDING = 0x01,
}
impl Err32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Err32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Err32 {
    #[inline(always)]
    fn from(val: u8) -> Err32 {
        Err32::from_bits(val)
    }
}
impl From<Err32> for u8 {
    #[inline(always)]
    fn from(val: Err32) -> u8 {
        Err32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Err6333(pub u32);
impl Err6333 {
    #[doc = "The Error Interrupt is not active for the relevant DMA channel."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The Error Interrupt is pending for the relevant DMA channel."]
    pub const PENDING: Self = Self(0x01);
}
impl Err6333 {
    pub const fn from_bits(val: u32) -> Err6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Err6333 {
    #[inline(always)]
    fn from(val: u32) -> Err6333 {
        Err6333::from_bits(val)
    }
}
impl From<Err6333> for u32 {
    #[inline(always)]
    fn from(val: Err6333) -> u32 {
        Err6333::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hwtrigen {
    #[doc = "Disabled. Hardware triggering is not used."]
    DISABLED = 0x0,
    #[doc = "Enabled. Use hardware triggering."]
    ENABLED = 0x01,
}
impl Hwtrigen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hwtrigen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hwtrigen {
    #[inline(always)]
    fn from(val: u8) -> Hwtrigen {
        Hwtrigen::from_bits(val)
    }
}
impl From<Hwtrigen> for u8 {
    #[inline(always)]
    fn from(val: Hwtrigen) -> u8 {
        Hwtrigen::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ia(pub u32);
impl Ia {
    #[doc = "The DMAchannel 0 interrupt A is not active."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The DMAchannel 0 interrupt A is active."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Ia {
    pub const fn from_bits(val: u32) -> Ia {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Ia {
    #[inline(always)]
    fn from(val: u32) -> Ia {
        Ia::from_bits(val)
    }
}
impl From<Ia> for u32 {
    #[inline(always)]
    fn from(val: Ia) -> u32 {
        Ia::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Ib(pub u32);
impl Ib {
    #[doc = "The DMAchannel 0 interrupt B is not active."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "The DMAchannel 0 interrupt B is active."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Ib {
    pub const fn from_bits(val: u32) -> Ib {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Ib {
    #[inline(always)]
    fn from(val: u32) -> Ib {
        Ib::from_bits(val)
    }
}
impl From<Ib> for u32 {
    #[inline(always)]
    fn from(val: Ib) -> u32 {
        Ib::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Inta32 {
    #[doc = "The DMAchannel 32 interrupt A is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The DMAchannel 0 interrupt A is active."]
    ACTIVE = 0x01,
}
impl Inta32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inta32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inta32 {
    #[inline(always)]
    fn from(val: u8) -> Inta32 {
        Inta32::from_bits(val)
    }
}
impl From<Inta32> for u8 {
    #[inline(always)]
    fn from(val: Inta32) -> u8 {
        Inta32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Inta6333(pub u32);
impl Inta6333 {
    #[doc = "Interrupt A is not active for the relevant DMA channel."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "Interrupt A is active for the relevant DMA channel."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Inta6333 {
    pub const fn from_bits(val: u32) -> Inta6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Inta6333 {
    #[inline(always)]
    fn from(val: u32) -> Inta6333 {
        Inta6333::from_bits(val)
    }
}
impl From<Inta6333> for u32 {
    #[inline(always)]
    fn from(val: Inta6333) -> u32 {
        Inta6333::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Intb32 {
    #[doc = "The DMAchannel 32 interrupt B is not active."]
    NOT_ACTIVE = 0x0,
    #[doc = "The DMAchannel 32 interrupt B is active."]
    ACTIVE = 0x01,
}
impl Intb32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Intb32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Intb32 {
    #[inline(always)]
    fn from(val: u8) -> Intb32 {
        Intb32::from_bits(val)
    }
}
impl From<Intb32> for u8 {
    #[inline(always)]
    fn from(val: Intb32) -> u8 {
        Intb32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Intb6333(pub u32);
impl Intb6333 {
    #[doc = "Interrupt B is not active for the relevant DMA channel."]
    pub const NOT_ACTIVE: Self = Self(0x0);
    #[doc = "Interrupt B is active for the relevant DMA channel."]
    pub const ACTIVE: Self = Self(0x01);
}
impl Intb6333 {
    pub const fn from_bits(val: u32) -> Intb6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Intb6333 {
    #[inline(always)]
    fn from(val: u32) -> Intb6333 {
        Intb6333::from_bits(val)
    }
}
impl From<Intb6333> for u32 {
    #[inline(always)]
    fn from(val: Intb6333) -> u32 {
        Intb6333::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "The Interrupt for DMA channel 0 is disabled."]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "The Interrupt for DMA channel 0 is enabled."]
    pub const ENABLED: Self = Self(0x01);
}
impl Inten {
    pub const fn from_bits(val: u32) -> Inten {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Inten {
    #[inline(always)]
    fn from(val: u32) -> Inten {
        Inten::from_bits(val)
    }
}
impl From<Inten> for u32 {
    #[inline(always)]
    fn from(val: Inten) -> u32 {
        Inten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Inten32 {
    #[doc = "The Interrupt for DMA channel 32 is disabled."]
    DISABLED = 0x0,
    #[doc = "The Interrupt for DMA channel 32 is enabled."]
    ENABLED = 0x01,
}
impl Inten32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inten32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inten32 {
    #[inline(always)]
    fn from(val: u8) -> Inten32 {
        Inten32::from_bits(val)
    }
}
impl From<Inten32> for u8 {
    #[inline(always)]
    fn from(val: Inten32) -> u8 {
        Inten32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Inten6333(pub u32);
impl Inten6333 {
    #[doc = "The Interrupt for the relevant DMA channel is disabled."]
    pub const DISABLED: Self = Self(0x0);
    #[doc = "The Interrupt for the relevant DMA channel is enabled."]
    pub const ENABLED: Self = Self(0x01);
}
impl Inten6333 {
    pub const fn from_bits(val: u32) -> Inten6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Inten6333 {
    #[inline(always)]
    fn from(val: u32) -> Inten6333 {
        Inten6333::from_bits(val)
    }
}
impl From<Inten6333> for u32 {
    #[inline(always)]
    fn from(val: Inten6333) -> u32 {
        Inten6333::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Periphreqen {
    #[doc = "Disabled. Peripheral DMA requests are disabled."]
    DISABLED = 0x0,
    #[doc = "Enabled. Peripheral DMA requests are enabled."]
    ENABLED = 0x01,
}
impl Periphreqen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Periphreqen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Periphreqen {
    #[inline(always)]
    fn from(val: u8) -> Periphreqen {
        Periphreqen::from_bits(val)
    }
}
impl From<Periphreqen> for u8 {
    #[inline(always)]
    fn from(val: Periphreqen) -> u8 {
        Periphreqen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Reload {
    #[doc = "Disabled. Do not reload the channels' control structure when the current descriptor is exhausted."]
    DISABLED = 0x0,
    #[doc = "Enabled. Reload the channels' control structure when the current descriptor is exhausted."]
    ENABLED = 0x01,
}
impl Reload {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reload {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reload {
    #[inline(always)]
    fn from(val: u8) -> Reload {
        Reload::from_bits(val)
    }
}
impl From<Reload> for u8 {
    #[inline(always)]
    fn from(val: Reload) -> u8 {
        Reload::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Setinta {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Set. The INTA flag for this channel will be set when the current descriptor is exhausted."]
    SET = 0x01,
}
impl Setinta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setinta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setinta {
    #[inline(always)]
    fn from(val: u8) -> Setinta {
        Setinta::from_bits(val)
    }
}
impl From<Setinta> for u8 {
    #[inline(always)]
    fn from(val: Setinta) -> u8 {
        Setinta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Setintb {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Set. The INTB flag for this channel will be set when the current descriptor is exhausted."]
    SET = 0x01,
}
impl Setintb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setintb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setintb {
    #[inline(always)]
    fn from(val: u8) -> Setintb {
        Setintb::from_bits(val)
    }
}
impl From<Setintb> for u8 {
    #[inline(always)]
    fn from(val: Setintb) -> u8 {
        Setintb::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Settrig0Trig(pub u32);
impl Settrig0Trig {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Sets the Trig bit for DMA channel 0."]
    pub const EFFECT: Self = Self(0x01);
}
impl Settrig0Trig {
    pub const fn from_bits(val: u32) -> Settrig0Trig {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Settrig0Trig {
    #[inline(always)]
    fn from(val: u32) -> Settrig0Trig {
        Settrig0Trig::from_bits(val)
    }
}
impl From<Settrig0Trig> for u32 {
    #[inline(always)]
    fn from(val: Settrig0Trig) -> u32 {
        Settrig0Trig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Settrig32 {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Sets the Trig bit for DMA channel 32."]
    EFFECT = 0x01,
}
impl Settrig32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Settrig32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Settrig32 {
    #[inline(always)]
    fn from(val: u8) -> Settrig32 {
        Settrig32::from_bits(val)
    }
}
impl From<Settrig32> for u8 {
    #[inline(always)]
    fn from(val: Settrig32) -> u8 {
        Settrig32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Settrig6333(pub u32);
impl Settrig6333 {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Sets the Trig bit for DMA channel for the relevant DMA channel."]
    pub const EFFECT: Self = Self(0x01);
}
impl Settrig6333 {
    pub const fn from_bits(val: u32) -> Settrig6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Settrig6333 {
    #[inline(always)]
    fn from(val: u32) -> Settrig6333 {
        Settrig6333::from_bits(val)
    }
}
impl From<Settrig6333> for u32 {
    #[inline(always)]
    fn from(val: Settrig6333) -> u32 {
        Settrig6333::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Setvalid32 {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Sets the ValidPending control bit for DMA channel 32."]
    EFFECT = 0x01,
}
impl Setvalid32 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setvalid32 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setvalid32 {
    #[inline(always)]
    fn from(val: u8) -> Setvalid32 {
        Setvalid32::from_bits(val)
    }
}
impl From<Setvalid32> for u8 {
    #[inline(always)]
    fn from(val: Setvalid32) -> u8 {
        Setvalid32::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Setvalid6333(pub u32);
impl Setvalid6333 {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Sets the ValidPending control bit for the relevant DMA channel."]
    pub const EFFECT: Self = Self(0x01);
}
impl Setvalid6333 {
    pub const fn from_bits(val: u32) -> Setvalid6333 {
        Self(val & 0x7fff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Setvalid6333 {
    #[inline(always)]
    fn from(val: u32) -> Setvalid6333 {
        Setvalid6333::from_bits(val)
    }
}
impl From<Setvalid6333> for u32 {
    #[inline(always)]
    fn from(val: Setvalid6333) -> u32 {
        Setvalid6333::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Srcburstwrap {
    #[doc = "Disabled. Source burst wrapping is not enabled for this DMA channel."]
    DISABLED = 0x0,
    #[doc = "Enabled. Source burst wrapping is enabled for this DMA channel."]
    ENABLED = 0x01,
}
impl Srcburstwrap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srcburstwrap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srcburstwrap {
    #[inline(always)]
    fn from(val: u8) -> Srcburstwrap {
        Srcburstwrap::from_bits(val)
    }
}
impl From<Srcburstwrap> for u8 {
    #[inline(always)]
    fn from(val: Srcburstwrap) -> u8 {
        Srcburstwrap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Srcinc {
    #[doc = "No increment. The source address is not incremented for each transfer. This is the usual case when the source is a peripheral device."]
    NO_INCREMENT = 0x0,
    #[doc = "1 x width. The source address is incremented by the amount specified by Width for each transfer. This is the usual case when the source is memory."]
    WIDTH_X_1 = 0x01,
    #[doc = "2 x width. The source address is incremented by 2 times the amount specified by Width for each transfer."]
    WIDTH_X_2 = 0x02,
    #[doc = "4 x width. The source address is incremented by 4 times the amount specified by Width for each transfer."]
    WIDTH_X_4 = 0x03,
}
impl Srcinc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srcinc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srcinc {
    #[inline(always)]
    fn from(val: u8) -> Srcinc {
        Srcinc::from_bits(val)
    }
}
impl From<Srcinc> for u8 {
    #[inline(always)]
    fn from(val: Srcinc) -> u8 {
        Srcinc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sv(pub u32);
impl Sv {
    #[doc = "No effect."]
    pub const NO_EFFECT: Self = Self(0x0);
    #[doc = "Sets the ValidPending control bit for DMA channel 0."]
    pub const EFFECT: Self = Self(0x01);
}
impl Sv {
    pub const fn from_bits(val: u32) -> Sv {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Sv {
    #[inline(always)]
    fn from(val: u32) -> Sv {
        Sv::from_bits(val)
    }
}
impl From<Sv> for u32 {
    #[inline(always)]
    fn from(val: Sv) -> u32 {
        Sv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swtrig {
    #[doc = "Not set. When written by software, the trigger for this channel is not set. A new trigger, as defined by the HWTRIGEN, TRIGPOL, and TRIGTYPE will be needed to start the channel."]
    NOT_SET = 0x0,
    #[doc = "Set. When written by software, the trigger for this channel is set immediately. This feature should not be used with level triggering when TRIGBURST = 0."]
    SET = 0x01,
}
impl Swtrig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swtrig {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swtrig {
    #[inline(always)]
    fn from(val: u8) -> Swtrig {
        Swtrig::from_bits(val)
    }
}
impl From<Swtrig> for u8 {
    #[inline(always)]
    fn from(val: Swtrig) -> u8 {
        Swtrig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trigburst {
    #[doc = "Single transfer. Hardware trigger causes a single transfer."]
    SINGLE = 0x0,
    #[doc = "Burst transfer. When the trigger for this channel is set to edge triggered, a hardware trigger causes a burst transfer, as defined by BURSTPOWER. When the trigger for this channel is set to level triggered, a hardware trigger causes transfers to continue as long as the trigger is asserted, unless the transfer is complete."]
    BURST = 0x01,
}
impl Trigburst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigburst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigburst {
    #[inline(always)]
    fn from(val: u8) -> Trigburst {
        Trigburst::from_bits(val)
    }
}
impl From<Trigburst> for u8 {
    #[inline(always)]
    fn from(val: Trigburst) -> u8 {
        Trigburst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trigpol {
    #[doc = "Active low - falling edge. Hardware trigger is active low or falling edge triggered, based on TRIGTYPE."]
    ACTIVE_LOW_FALLING = 0x0,
    #[doc = "Active high - rising edge. Hardware trigger is active high or rising edge triggered, based on TRIGTYPE."]
    ACTIVE_HIGH_RISING = 0x01,
}
impl Trigpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigpol {
    #[inline(always)]
    fn from(val: u8) -> Trigpol {
        Trigpol::from_bits(val)
    }
}
impl From<Trigpol> for u8 {
    #[inline(always)]
    fn from(val: Trigpol) -> u8 {
        Trigpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trigtype {
    #[doc = "Edge. Hardware trigger is edge triggered. Transfers will be initiated and completed, as specified for a single trigger."]
    EDGE = 0x0,
    #[doc = "Level. Hardware trigger is level triggered. Note that when level triggering without burst (BURSTPOWER = 0) is selected, only hardware triggers should be used on that channel. Transfers continue as long as the trigger level is asserted. Once the trigger is de-asserted, the transfer will be paused until the trigger is, again, asserted. However, the transfer will not be paused until any remaining transfers within the current BURSTPOWER length are completed."]
    LEVEL = 0x01,
}
impl Trigtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trigtype {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trigtype {
    #[inline(always)]
    fn from(val: u8) -> Trigtype {
        Trigtype::from_bits(val)
    }
}
impl From<Trigtype> for u8 {
    #[inline(always)]
    fn from(val: Trigtype) -> u8 {
        Trigtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Validpending {
    #[doc = "No effect. No effect on DMA operation."]
    NO_EFFECT = 0x0,
    #[doc = "Valid pending."]
    VALID_PENDING = 0x01,
}
impl Validpending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Validpending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Validpending {
    #[inline(always)]
    fn from(val: u8) -> Validpending {
        Validpending::from_bits(val)
    }
}
impl From<Validpending> for u8 {
    #[inline(always)]
    fn from(val: Validpending) -> u8 {
        Validpending::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Width {
    #[doc = "8-bit. 8-bit transfers are performed (8-bit source reads and destination writes)."]
    BIT_8 = 0x0,
    #[doc = "16-bit. 6-bit transfers are performed (16-bit source reads and destination writes)."]
    BIT_16 = 0x01,
    #[doc = "32-bit. 32-bit transfers are performed (32-bit source reads and destination writes)."]
    BIT_32 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Width {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Width {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Width {
    #[inline(always)]
    fn from(val: u8) -> Width {
        Width::from_bits(val)
    }
}
impl From<Width> for u8 {
    #[inline(always)]
    fn from(val: Width) -> u8 {
        Width::to_bits(val)
    }
}
