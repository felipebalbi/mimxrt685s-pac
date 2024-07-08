#[doc = "Channel Abort control for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abort0(pub u32);
impl Abort0 {
    #[doc = "Abort control for DMA channel 0."]
    #[inline(always)]
    pub const fn abortctrl(&self) -> super::vals::Abortctrl {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Abortctrl::from_bits(val as u32)
    }
    #[doc = "Abort control for DMA channel 0."]
    #[inline(always)]
    pub fn set_abortctrl(&mut self, val: super::vals::Abortctrl) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Abort0 {
    #[inline(always)]
    fn default() -> Abort0 {
        Abort0(0)
    }
}
#[doc = "Channel Abort control for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abort1(pub u32);
impl Abort1 {
    #[doc = "Abort control for DMA channel 32."]
    #[inline(always)]
    pub const fn abort32(&self) -> super::vals::Abort32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Abort32::from_bits(val as u8)
    }
    #[doc = "Abort control for DMA channel 32."]
    #[inline(always)]
    pub fn set_abort32(&mut self, val: super::vals::Abort32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional Abort controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub const fn abort63_33(&self) -> super::vals::Abort6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Abort6333::from_bits(val as u32)
    }
    #[doc = "Additional Abort controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn set_abort63_33(&mut self, val: super::vals::Abort6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Abort1 {
    #[inline(always)]
    fn default() -> Abort1 {
        Abort1(0)
    }
}
#[doc = "Channel Active status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Active0(pub u32);
impl Active0 {
    #[doc = "Active flag for DMA channel 0."]
    #[inline(always)]
    pub const fn act(&self) -> super::vals::Act {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Act::from_bits(val as u32)
    }
    #[doc = "Active flag for DMA channel 0."]
    #[inline(always)]
    pub fn set_act(&mut self, val: super::vals::Act) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Active0 {
    #[inline(always)]
    fn default() -> Active0 {
        Active0(0)
    }
}
#[doc = "Channel Active status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Active1(pub u32);
impl Active1 {
    #[doc = "Active flag for DMA channel 32."]
    #[inline(always)]
    pub const fn active32(&self) -> super::vals::Active32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Active32::from_bits(val as u8)
    }
    #[doc = "Active flag for DMA channel 32."]
    #[inline(always)]
    pub fn set_active32(&mut self, val: super::vals::Active32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub const fn active63_33(&self) -> super::vals::Active6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Active6333::from_bits(val as u32)
    }
    #[doc = "Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn set_active63_33(&mut self, val: super::vals::Active6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Active1 {
    #[inline(always)]
    fn default() -> Active1 {
        Active1(0)
    }
}
#[doc = "Channel Busy status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busy0(pub u32);
impl Busy0 {
    #[doc = "Busy flag for DMA channel 0."]
    #[inline(always)]
    pub const fn bsy(&self) -> super::vals::Bsy {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Bsy::from_bits(val as u32)
    }
    #[doc = "Busy flag for DMA channel 0."]
    #[inline(always)]
    pub fn set_bsy(&mut self, val: super::vals::Bsy) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Busy0 {
    #[inline(always)]
    fn default() -> Busy0 {
        Busy0(0)
    }
}
#[doc = "Channel Busy status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busy1(pub u32);
impl Busy1 {
    #[doc = "Busy flag for DMA channel 32."]
    #[inline(always)]
    pub const fn busy32(&self) -> super::vals::Busy32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Busy32::from_bits(val as u8)
    }
    #[doc = "Busy flag for DMA channel 32."]
    #[inline(always)]
    pub fn set_busy32(&mut self, val: super::vals::Busy32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub const fn busy63_33(&self) -> super::vals::Busy6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Busy6333::from_bits(val as u32)
    }
    #[doc = "Additional Active flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn set_busy63_33(&mut self, val: super::vals::Busy6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Busy1 {
    #[inline(always)]
    fn default() -> Busy1 {
        Busy1(0)
    }
}
#[doc = "Configuration register for DMA channel ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub const fn periphreqen(&self) -> super::vals::Periphreqen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Periphreqen::from_bits(val as u8)
    }
    #[doc = "Peripheral request Enable. If a DMA channel is used to perform a memory-to-memory move, any peripheral DMA request associated with that channel can be disabled to prevent any interaction between the peripheral and the DMA controller."]
    #[inline(always)]
    pub fn set_periphreqen(&mut self, val: super::vals::Periphreqen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub const fn hwtrigen(&self) -> super::vals::Hwtrigen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Hwtrigen::from_bits(val as u8)
    }
    #[doc = "Hardware Triggering Enable for this channel."]
    #[inline(always)]
    pub fn set_hwtrigen(&mut self, val: super::vals::Hwtrigen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub const fn trigpol(&self) -> super::vals::Trigpol {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Trigpol::from_bits(val as u8)
    }
    #[doc = "Trigger Polarity. Selects the polarity of a hardware trigger for this channel."]
    #[inline(always)]
    pub fn set_trigpol(&mut self, val: super::vals::Trigpol) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub const fn trigtype(&self) -> super::vals::Trigtype {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Trigtype::from_bits(val as u8)
    }
    #[doc = "Trigger Type. Selects hardware trigger as edge triggered or level triggered."]
    #[inline(always)]
    pub fn set_trigtype(&mut self, val: super::vals::Trigtype) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub const fn trigburst(&self) -> super::vals::Trigburst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Trigburst::from_bits(val as u8)
    }
    #[doc = "Trigger Burst. Selects whether hardware triggers cause a single or burst transfer."]
    #[inline(always)]
    pub fn set_trigburst(&mut self, val: super::vals::Trigburst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline(always)]
    pub const fn burstpower(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Burst Power is used in two ways. It always selects the address wrap size when SRCBURSTWRAP and/or DSTBURSTWRAP modes are selected (see descriptions elsewhere in this register). When the TRIGBURST field elsewhere in this register = 1, Burst Power selects how many transfers are performed for each DMA trigger. This can be used, for example, with peripherals that contain a FIFO that can initiate a DMA operation when the FIFO reaches a certain level. 0000: Burst size = 1 (20). 0001: Burst size = 2 (21). 0010: Burst size = 4 (22). 1010: Burst size = 1024 (210). This corresponds to the maximum supported transfer count. others: not supported. The total transfer length as defined in the XFERCOUNT bits in the XFERCFG register must be an even multiple of the burst size."]
    #[inline(always)]
    pub fn set_burstpower(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub const fn srcburstwrap(&self) -> super::vals::Srcburstwrap {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Srcburstwrap::from_bits(val as u8)
    }
    #[doc = "Source Burst Wrap. When enabled, the source data address for the DMA is 'wrapped', meaning that the source address range for each burst will be the same. As an example, this could be used to read several sequential registers from a peripheral for each DMA burst, reading the same registers again for each burst."]
    #[inline(always)]
    pub fn set_srcburstwrap(&mut self, val: super::vals::Srcburstwrap) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub const fn dstburstwrap(&self) -> super::vals::Dstburstwrap {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dstburstwrap::from_bits(val as u8)
    }
    #[doc = "Destination Burst Wrap. When enabled, the destination data address for the DMA is 'wrapped', meaning that the destination address range for each burst will be the same. As an example, this could be used to write several sequential registers to a peripheral for each DMA burst, writing the same registers again for each burst."]
    #[inline(always)]
    pub fn set_dstburstwrap(&mut self, val: super::vals::Dstburstwrap) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline(always)]
    pub const fn chpriority(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Priority of this channel when multiple DMA requests are pending. Eight priority levels are supported: 0x0 = highest priority. 0x7 = lowest priority."]
    #[inline(always)]
    pub fn set_chpriority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
#[doc = "Control and status register for DMA channel ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctlstat(pub u32);
impl Ctlstat {
    #[doc = "Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[inline(always)]
    pub const fn validpending(&self) -> super::vals::Validpending {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Validpending::from_bits(val as u8)
    }
    #[doc = "Valid pending flag for this channel. This bit is set when a 1 is written to the corresponding bit in the related SETVALID register when CFGVALID = 1 for the same channel."]
    #[inline(always)]
    pub fn set_validpending(&mut self, val: super::vals::Validpending) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[inline(always)]
    pub const fn trig(&self) -> super::vals::CtlstatTrig {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CtlstatTrig::from_bits(val as u8)
    }
    #[doc = "Trigger flag. Indicates that the trigger for this channel is currently set. This bit is cleared at the end of an entire transfer or upon reload when CLRTRIG = 1."]
    #[inline(always)]
    pub fn set_trig(&mut self, val: super::vals::CtlstatTrig) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Ctlstat {
    #[inline(always)]
    fn default() -> Ctlstat {
        Ctlstat(0)
    }
}
#[doc = "DMA control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "DMA controller master enable."]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "DMA controller master enable."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Channel Enable Clear for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enableclr0(pub u32);
impl Enableclr0 {
    #[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub const fn clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears the corresponding bits in ENABLESET0."]
    #[inline(always)]
    pub fn set_clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Enableclr0 {
    #[inline(always)]
    fn default() -> Enableclr0 {
        Enableclr0(0)
    }
}
#[doc = "Channel Enable Clear for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enableclr1(pub u32);
impl Enableclr1 {
    #[doc = "Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    pub const fn clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears the corresponding bits in ENABLESET1."]
    #[inline(always)]
    pub fn set_clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Enableclr1 {
    #[inline(always)]
    fn default() -> Enableclr1 {
        Enableclr1(0)
    }
}
#[doc = "Channel Enable read and Set for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enableset0(pub u32);
impl Enableset0 {
    #[doc = "Enable for DMA channel 0"]
    #[inline(always)]
    pub const fn ena(&self) -> super::vals::Ena {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Ena::from_bits(val as u32)
    }
    #[doc = "Enable for DMA channel 0"]
    #[inline(always)]
    pub fn set_ena(&mut self, val: super::vals::Ena) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Enableset0 {
    #[inline(always)]
    fn default() -> Enableset0 {
        Enableset0(0)
    }
}
#[doc = "Channel Enable read and Set for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enableset1(pub u32);
impl Enableset1 {
    #[doc = "Enable for DMA channel 32"]
    #[inline(always)]
    pub const fn enable32(&self) -> super::vals::Enable32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable32::from_bits(val as u8)
    }
    #[doc = "Enable for DMA channel 32"]
    #[inline(always)]
    pub fn set_enable32(&mut self, val: super::vals::Enable32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional enables for remaining DMA channels in the range 63 to 33."]
    #[inline(always)]
    pub const fn enable63_33(&self) -> super::vals::Enable6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Enable6333::from_bits(val as u32)
    }
    #[doc = "Additional enables for remaining DMA channels in the range 63 to 33."]
    #[inline(always)]
    pub fn set_enable63_33(&mut self, val: super::vals::Enable6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Enableset1 {
    #[inline(always)]
    fn default() -> Enableset1 {
        Enableset1(0)
    }
}
#[doc = "Error Interrupt status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errint0(pub u32);
impl Errint0 {
    #[doc = "Error Interrupt flag for DMA channel 0."]
    #[inline(always)]
    pub const fn err(&self) -> super::vals::Err {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Err::from_bits(val as u32)
    }
    #[doc = "Error Interrupt flag for DMA channel 0."]
    #[inline(always)]
    pub fn set_err(&mut self, val: super::vals::Err) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Errint0 {
    #[inline(always)]
    fn default() -> Errint0 {
        Errint0(0)
    }
}
#[doc = "Error Interrupt status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errint1(pub u32);
impl Errint1 {
    #[doc = "Error Interrupt flag for DMA channel 32."]
    #[inline(always)]
    pub const fn err32(&self) -> super::vals::Err32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Err32::from_bits(val as u8)
    }
    #[doc = "Error Interrupt flag for DMA channel 32."]
    #[inline(always)]
    pub fn set_err32(&mut self, val: super::vals::Err32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional error Interrupt flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub const fn err63_33(&self) -> super::vals::Err6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Err6333::from_bits(val as u32)
    }
    #[doc = "Additional error Interrupt flags for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn set_err63_33(&mut self, val: super::vals::Err6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Errint1 {
    #[inline(always)]
    fn default() -> Errint1 {
        Errint1(0)
    }
}
#[doc = "Interrupt A status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inta0(pub u32);
impl Inta0 {
    #[doc = "Interrupt A status for DMA channel 0."]
    #[inline(always)]
    pub const fn ia(&self) -> super::vals::Ia {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Ia::from_bits(val as u32)
    }
    #[doc = "Interrupt A status for DMA channel 0."]
    #[inline(always)]
    pub fn set_ia(&mut self, val: super::vals::Ia) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Inta0 {
    #[inline(always)]
    fn default() -> Inta0 {
        Inta0(0)
    }
}
#[doc = "Interrupt A status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inta1(pub u32);
impl Inta1 {
    #[doc = "Interrupt A status for DMA channel 32."]
    #[inline(always)]
    pub const fn inta32(&self) -> super::vals::Inta32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Inta32::from_bits(val as u8)
    }
    #[doc = "Interrupt A status for DMA channel 32."]
    #[inline(always)]
    pub fn set_inta32(&mut self, val: super::vals::Inta32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional Interrupt A status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub const fn inta63_33(&self) -> super::vals::Inta6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Inta6333::from_bits(val as u32)
    }
    #[doc = "Additional Interrupt A status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn set_inta63_33(&mut self, val: super::vals::Inta6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Inta1 {
    #[inline(always)]
    fn default() -> Inta1 {
        Inta1(0)
    }
}
#[doc = "Interrupt B status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intb0(pub u32);
impl Intb0 {
    #[doc = "Interrupt B status for DMA channel 0."]
    #[inline(always)]
    pub const fn ib(&self) -> super::vals::Ib {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Ib::from_bits(val as u32)
    }
    #[doc = "Interrupt B status for DMA channel 0."]
    #[inline(always)]
    pub fn set_ib(&mut self, val: super::vals::Ib) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intb0 {
    #[inline(always)]
    fn default() -> Intb0 {
        Intb0(0)
    }
}
#[doc = "Interrupt B status for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intb1(pub u32);
impl Intb1 {
    #[doc = "Interrupt B status for DMA channel 32."]
    #[inline(always)]
    pub const fn intb32(&self) -> super::vals::Intb32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intb32::from_bits(val as u8)
    }
    #[doc = "Interrupt B status for DMA channel 32."]
    #[inline(always)]
    pub fn set_intb32(&mut self, val: super::vals::Intb32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional Interrupt B status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub const fn intb63_33(&self) -> super::vals::Intb6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Intb6333::from_bits(val as u32)
    }
    #[doc = "Additional Interrupt B status bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn set_intb63_33(&mut self, val: super::vals::Intb6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Intb1 {
    #[inline(always)]
    fn default() -> Intb1 {
        Intb1(0)
    }
}
#[doc = "Interrupt Enable Clear for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr0(pub u32);
impl Intenclr0 {
    #[doc = "Writing ones to this register clears corresponding bits in the DMAIntEnSet0."]
    #[inline(always)]
    pub const fn clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears corresponding bits in the DMAIntEnSet0."]
    #[inline(always)]
    pub fn set_clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intenclr0 {
    #[inline(always)]
    fn default() -> Intenclr0 {
        Intenclr0(0)
    }
}
#[doc = "Interrupt Enable Clear for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr1(pub u32);
impl Intenclr1 {
    #[doc = "Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
    #[inline(always)]
    pub const fn clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Writing ones to this register clears corresponding bits in the DMAIntEnSet1."]
    #[inline(always)]
    pub fn set_clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intenclr1 {
    #[inline(always)]
    fn default() -> Intenclr1 {
        Intenclr1(0)
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset0(pub u32);
impl Intenset0 {
    #[doc = "Interrupt Enable read and set for DMA channel 0."]
    #[inline(always)]
    pub const fn inten(&self) -> super::vals::Inten {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Inten::from_bits(val as u32)
    }
    #[doc = "Interrupt Enable read and set for DMA channel 0."]
    #[inline(always)]
    pub fn set_inten(&mut self, val: super::vals::Inten) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intenset0 {
    #[inline(always)]
    fn default() -> Intenset0 {
        Intenset0(0)
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset1(pub u32);
impl Intenset1 {
    #[doc = "Interrupt Enable read and set for DMA channel 32."]
    #[inline(always)]
    pub const fn inten32(&self) -> super::vals::Inten32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Inten32::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable read and set for DMA channel 32."]
    #[inline(always)]
    pub fn set_inten32(&mut self, val: super::vals::Inten32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional Interrupt Enable read and set bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub const fn inten63_33(&self) -> super::vals::Inten6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Inten6333::from_bits(val as u32)
    }
    #[doc = "Additional Interrupt Enable read and set bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn set_inten63_33(&mut self, val: super::vals::Inten6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Intenset1 {
    #[inline(always)]
    fn default() -> Intenset1 {
        Intenset1(0)
    }
}
#[doc = "Interrupt status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[inline(always)]
    pub const fn activeint(&self) -> super::vals::Activeint {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Activeint::from_bits(val as u8)
    }
    #[doc = "Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[inline(always)]
    pub fn set_activeint(&mut self, val: super::vals::Activeint) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Summarizes whether any error interrupts are pending."]
    #[inline(always)]
    pub const fn activeerrint(&self) -> super::vals::Activeerrint {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Activeerrint::from_bits(val as u8)
    }
    #[doc = "Summarizes whether any error interrupts are pending."]
    #[inline(always)]
    pub fn set_activeerrint(&mut self, val: super::vals::Activeerrint) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
#[doc = "Set Trigger control bits for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Settrig0(pub u32);
impl Settrig0 {
    #[doc = "Set Trigger control bit for DMA channel 0."]
    #[inline(always)]
    pub const fn trig(&self) -> super::vals::Settrig0Trig {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Settrig0Trig::from_bits(val as u32)
    }
    #[doc = "Set Trigger control bit for DMA channel 0."]
    #[inline(always)]
    pub fn set_trig(&mut self, val: super::vals::Settrig0Trig) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Settrig0 {
    #[inline(always)]
    fn default() -> Settrig0 {
        Settrig0(0)
    }
}
#[doc = "Set Trigger control bits for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Settrig1(pub u32);
impl Settrig1 {
    #[doc = "Set Trigger control bit for DMA channel 32."]
    #[inline(always)]
    pub const fn settrig32(&self) -> super::vals::Settrig32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Settrig32::from_bits(val as u8)
    }
    #[doc = "Set Trigger control bit for DMA channel 32."]
    #[inline(always)]
    pub fn set_settrig32(&mut self, val: super::vals::Settrig32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional Set Trigger control bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub const fn settrig63_33(&self) -> super::vals::Settrig6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Settrig6333::from_bits(val as u32)
    }
    #[doc = "Additional Set Trigger control bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn set_settrig63_33(&mut self, val: super::vals::Settrig6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Settrig1 {
    #[inline(always)]
    fn default() -> Settrig1 {
        Settrig1(0)
    }
}
#[doc = "Set ValidPending control bits for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setvalid0(pub u32);
impl Setvalid0 {
    #[doc = "SetValid control for DMA channel 0."]
    #[inline(always)]
    pub const fn sv(&self) -> super::vals::Sv {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Sv::from_bits(val as u32)
    }
    #[doc = "SetValid control for DMA channel 0."]
    #[inline(always)]
    pub fn set_sv(&mut self, val: super::vals::Sv) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Setvalid0 {
    #[inline(always)]
    fn default() -> Setvalid0 {
        Setvalid0(0)
    }
}
#[doc = "Set ValidPending control bits for all DMA channels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setvalid1(pub u32);
impl Setvalid1 {
    #[doc = "SetValid control for DMA channel 32."]
    #[inline(always)]
    pub const fn setvalid32(&self) -> super::vals::Setvalid32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Setvalid32::from_bits(val as u8)
    }
    #[doc = "SetValid control for DMA channel 32."]
    #[inline(always)]
    pub fn set_setvalid32(&mut self, val: super::vals::Setvalid32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Additional SetValid controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub const fn setvalid63_33(&self) -> super::vals::Setvalid6333 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        super::vals::Setvalid6333::from_bits(val as u32)
    }
    #[doc = "Additional SetValid controls for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn set_setvalid63_33(&mut self, val: super::vals::Setvalid6333) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize))
            | (((val.to_bits() as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for Setvalid1 {
    #[inline(always)]
    fn default() -> Setvalid1 {
        Setvalid1(0)
    }
}
#[doc = "SRAM address of the channel configuration table."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srambase(pub u32);
impl Srambase {
    #[doc = "Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    pub const fn offset(&self) -> u32 {
        let val = (self.0 >> 9usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Address bits 31:9 of the beginning of the DMA descriptor table. For 18 channels, the table must begin on a 512 byte boundary."]
    #[inline(always)]
    pub fn set_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 9usize)) | (((val as u32) & 0x007f_ffff) << 9usize);
    }
}
impl Default for Srambase {
    #[inline(always)]
    fn default() -> Srambase {
        Srambase(0)
    }
}
#[doc = "Transfer configuration register for DMA channel ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xfercfg(pub u32);
impl Xfercfg {
    #[doc = "Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub const fn cfgvalid(&self) -> super::vals::Cfgvalid {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cfgvalid::from_bits(val as u8)
    }
    #[doc = "Configuration Valid flag. This bit indicates whether the current channel descriptor is valid and can potentially be acted upon, if all other activation criteria are fulfilled."]
    #[inline(always)]
    pub fn set_cfgvalid(&mut self, val: super::vals::Cfgvalid) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub const fn reload(&self) -> super::vals::Reload {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Reload::from_bits(val as u8)
    }
    #[doc = "Indicates whether the channel's control structure will be reloaded when the current descriptor is exhausted. Reloading allows ping-pong and linked transfers."]
    #[inline(always)]
    pub fn set_reload(&mut self, val: super::vals::Reload) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Software Trigger."]
    #[inline(always)]
    pub const fn swtrig(&self) -> super::vals::Swtrig {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Swtrig::from_bits(val as u8)
    }
    #[doc = "Software Trigger."]
    #[inline(always)]
    pub fn set_swtrig(&mut self, val: super::vals::Swtrig) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Trigger."]
    #[inline(always)]
    pub const fn clrtrig(&self) -> super::vals::Clrtrig {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Clrtrig::from_bits(val as u8)
    }
    #[doc = "Clear Trigger."]
    #[inline(always)]
    pub fn set_clrtrig(&mut self, val: super::vals::Clrtrig) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub const fn setinta(&self) -> super::vals::Setinta {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Setinta::from_bits(val as u8)
    }
    #[doc = "Set Interrupt flag A for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn set_setinta(&mut self, val: super::vals::Setinta) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub const fn setintb(&self) -> super::vals::Setintb {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Setintb::from_bits(val as u8)
    }
    #[doc = "Set Interrupt flag B for this channel. There is no hardware distinction between interrupt A and B. They can be used by software to assist with more complex descriptor usage. By convention, interrupt A may be used when only one interrupt flag is needed."]
    #[inline(always)]
    pub fn set_setintb(&mut self, val: super::vals::Setintb) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer width used for this DMA channel."]
    #[inline(always)]
    pub const fn width(&self) -> super::vals::Width {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Width::from_bits(val as u8)
    }
    #[doc = "Transfer width used for this DMA channel."]
    #[inline(always)]
    pub fn set_width(&mut self, val: super::vals::Width) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub const fn srcinc(&self) -> super::vals::Srcinc {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Srcinc::from_bits(val as u8)
    }
    #[doc = "Determines whether the source address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn set_srcinc(&mut self, val: super::vals::Srcinc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub const fn dstinc(&self) -> super::vals::Dstinc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Dstinc::from_bits(val as u8)
    }
    #[doc = "Determines whether the destination address is incremented for each DMA transfer."]
    #[inline(always)]
    pub fn set_dstinc(&mut self, val: super::vals::Dstinc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub const fn xfercount(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Total number of transfers to be performed, minus 1 encoded. The number of bytes transferred is: (XFERCOUNT + 1) x data width (as defined by the WIDTH field). The DMA controller uses this bit field during transfer to count down. Hence, it cannot be used by software to read back the size of the transfer, for instance, in an interrupt handler. 0x0 = a total of 1 transfer will be performed. 0x1 = a total of 2 transfers will be performed. 0x3FF = a total of 1,024 transfers will be performed."]
    #[inline(always)]
    pub fn set_xfercount(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Xfercfg {
    #[inline(always)]
    fn default() -> Xfercfg {
        Xfercfg(0)
    }
}
