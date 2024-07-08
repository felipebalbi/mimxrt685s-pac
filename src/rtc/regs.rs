#[doc = "RTC counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count(pub u32);
impl Count {
    #[doc = "A read reflects the current value of the main, 1 Hz RTC timer. A write loads a new initial value into the timer. The RTC counter will count up continuously at a 1 Hz rate once the RTC Software Reset is removed (by clearing bit 0 of the CTRL register). Only write to this register when the RTC_EN bit in the RTC CTRL Register is 0. The counter increments one second after the RTC_EN bit is set."]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "A read reflects the current value of the main, 1 Hz RTC timer. A write loads a new initial value into the timer. The RTC counter will count up continuously at a 1 Hz rate once the RTC Software Reset is removed (by clearing bit 0 of the CTRL register). Only write to this register when the RTC_EN bit in the RTC CTRL Register is 0. The counter increments one second after the RTC_EN bit is set."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Count {
    #[inline(always)]
    fn default() -> Count {
        Count(0)
    }
}
#[doc = "RTC control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Software reset control"]
    #[inline(always)]
    pub const fn swreset(&self) -> super::vals::Swreset {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Swreset::from_bits(val as u8)
    }
    #[doc = "Software reset control"]
    #[inline(always)]
    pub fn set_swreset(&mut self, val: super::vals::Swreset) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RTC 1 Hz timer alarm flag status."]
    #[inline(always)]
    pub const fn alarm1hz(&self) -> super::vals::Alarm1hz {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Alarm1hz::from_bits(val as u8)
    }
    #[doc = "RTC 1 Hz timer alarm flag status."]
    #[inline(always)]
    pub fn set_alarm1hz(&mut self, val: super::vals::Alarm1hz) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "RTC 1 kHz timer wake-up flag status."]
    #[inline(always)]
    pub const fn wake1khz(&self) -> super::vals::Wake1khz {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Wake1khz::from_bits(val as u8)
    }
    #[doc = "RTC 1 kHz timer wake-up flag status."]
    #[inline(always)]
    pub fn set_wake1khz(&mut self, val: super::vals::Wake1khz) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline(always)]
    pub const fn alarmdpd_en(&self) -> super::vals::AlarmdpdEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AlarmdpdEn::from_bits(val as u8)
    }
    #[doc = "RTC 1 Hz timer alarm enable for Deep power-down."]
    #[inline(always)]
    pub fn set_alarmdpd_en(&mut self, val: super::vals::AlarmdpdEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline(always)]
    pub const fn wakedpd_en(&self) -> super::vals::WakedpdEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::WakedpdEn::from_bits(val as u8)
    }
    #[doc = "RTC 1 kHz timer wake-up enable for Deep power-down."]
    #[inline(always)]
    pub fn set_wakedpd_en(&mut self, val: super::vals::WakedpdEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline(always)]
    pub const fn rtc1khz_en(&self) -> super::vals::Rtc1khzEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Rtc1khzEn::from_bits(val as u8)
    }
    #[doc = "RTC 1 kHz clock enable. This bit can be set to 0 to conserve power if the 1 kHz timer is not used. This bit has no effect when the RTC is disabled (bit 7 of this register is 0)."]
    #[inline(always)]
    pub fn set_rtc1khz_en(&mut self, val: super::vals::Rtc1khzEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RTC enable."]
    #[inline(always)]
    pub const fn rtc_en(&self) -> super::vals::RtcEn {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RtcEn::from_bits(val as u8)
    }
    #[doc = "RTC enable."]
    #[inline(always)]
    pub fn set_rtc_en(&mut self, val: super::vals::RtcEn) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "The RTC oscillator enable"]
    #[inline(always)]
    pub const fn rtc_osc_pd(&self) -> super::vals::RtcOscPd {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RtcOscPd::from_bits(val as u8)
    }
    #[doc = "The RTC oscillator enable"]
    #[inline(always)]
    pub fn set_rtc_osc_pd(&mut self, val: super::vals::RtcOscPd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "The 32 KHz sub-second counter enable"]
    #[inline(always)]
    pub const fn rtc_subsec_ena(&self) -> super::vals::RtcSubsecEna {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::RtcSubsecEna::from_bits(val as u8)
    }
    #[doc = "The 32 KHz sub-second counter enable"]
    #[inline(always)]
    pub fn set_rtc_subsec_ena(&mut self, val: super::vals::RtcSubsecEna) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "capacitive load selection"]
    #[inline(always)]
    pub const fn rtc_osc_loadcap(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "capacitive load selection"]
    #[inline(always)]
    pub fn set_rtc_osc_loadcap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "General Purpose register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpreg(pub u32);
impl Gpreg {
    #[doc = "Data retained during Deep power-down mode or loss of main power as long as VBAT is supplied."]
    #[inline(always)]
    pub const fn gpdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data retained during Deep power-down mode or loss of main power as long as VBAT is supplied."]
    #[inline(always)]
    pub fn set_gpdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpreg {
    #[inline(always)]
    fn default() -> Gpreg {
        Gpreg(0)
    }
}
#[doc = "RTC match register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match(pub u32);
impl Match {
    #[doc = "Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[inline(always)]
    pub const fn matval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Contains the match value against which the 1 Hz RTC timer will be compared to set the alarm flag RTC_ALARM and generate an alarm interrupt/wake-up if enabled."]
    #[inline(always)]
    pub fn set_matval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Match {
    #[inline(always)]
    fn default() -> Match {
        Match(0)
    }
}
#[doc = "RTC Sub-second Counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Subsec(pub u32);
impl Subsec {
    #[doc = "A read reflects the current value of the 32Khz sub-second counter. This counter will be cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32 KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep_powerdown mode or after the main RTC module has been disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[inline(always)]
    pub const fn rtc_subsec(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "A read reflects the current value of the 32Khz sub-second counter. This counter will be cleared whenever the SUBSEC_ENA bit in the RTC_CONTROL register is low. Up-counting at a 32 KHz rate commences at the start of the next one-second interval after the SUBSEC_ENA bit is set. This counter must be re-enabled after exiting deep_powerdown mode or after the main RTC module has been disabled and re-enabled. On modules not equipped with a sub-second counter, this register will read-back as all zeroes."]
    #[inline(always)]
    pub fn set_rtc_subsec(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Subsec {
    #[inline(always)]
    fn default() -> Subsec {
        Subsec(0)
    }
}
#[doc = "High-resolution/wake-up timer control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wake(pub u32);
impl Wake {
    #[doc = "A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[inline(always)]
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "A read reflects the current value of the high-resolution/wake-up timer. A write pre-loads a start count value into the wake-up timer and initializes a count-down sequence. Do not write to this register while counting is in progress."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Wake {
    #[inline(always)]
    fn default() -> Wake {
        Wake(0)
    }
}
