#[doc = "Clock register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clock(pub u32);
impl Clock {
    #[doc = "Unit of Measurement Encoding for Clock Speed"]
    #[inline(always)]
    pub const fn clock_unit(&self) -> super::vals::ClockUnit {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ClockUnit::from_bits(val as u8)
    }
    #[doc = "Unit of Measurement Encoding for Clock Speed"]
    #[inline(always)]
    pub fn set_clock_unit(&mut self, val: super::vals::ClockUnit) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Numerical Value of Clock Speed in Binary"]
    #[inline(always)]
    pub const fn clock_speed(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x03ff;
        val as u16
    }
    #[doc = "Numerical Value of Clock Speed in Binary"]
    #[inline(always)]
    pub fn set_clock_speed(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 2usize)) | (((val as u32) & 0x03ff) << 2usize);
    }
}
impl Default for Clock {
    #[inline(always)]
    fn default() -> Clock {
        Clock(0)
    }
}
#[doc = "Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "Interrupt Acknowledge"]
    #[inline(always)]
    pub const fn iack(&self) -> super::vals::Iack {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Iack::from_bits(val as u8)
    }
    #[doc = "Interrupt Acknowledge"]
    #[inline(always)]
    pub fn set_iack(&mut self, val: super::vals::Iack) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Flag"]
    #[inline(always)]
    pub const fn if_(&self) -> super::vals::If {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::If::from_bits(val as u8)
    }
    #[doc = "Interrupt Flag"]
    #[inline(always)]
    pub fn set_if_(&mut self, val: super::vals::If) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn ie(&self) -> super::vals::Ie {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ie::from_bits(val as u8)
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: super::vals::Ie) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "BC12"]
    #[inline(always)]
    pub const fn bc12(&self) -> super::vals::Bc12 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Bc12::from_bits(val as u8)
    }
    #[doc = "BC12"]
    #[inline(always)]
    pub fn set_bc12(&mut self, val: super::vals::Bc12) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Start Change Detection Sequence"]
    #[inline(always)]
    pub const fn start(&self) -> super::vals::Start {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Start::from_bits(val as u8)
    }
    #[doc = "Start Change Detection Sequence"]
    #[inline(always)]
    pub fn set_start(&mut self, val: super::vals::Start) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn sr(&self) -> super::vals::Sr {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Sr::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn set_sr(&mut self, val: super::vals::Sr) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
#[doc = "Signal Override Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SignalOverride(pub u32);
impl SignalOverride {
    #[doc = "Phase Selection"]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Phase Selection"]
    #[inline(always)]
    pub fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for SignalOverride {
    #[inline(always)]
    fn default() -> SignalOverride {
        SignalOverride(0)
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Charger Detection Sequence Results"]
    #[inline(always)]
    pub const fn seq_res(&self) -> super::vals::SeqRes {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::SeqRes::from_bits(val as u8)
    }
    #[doc = "Charger Detection Sequence Results"]
    #[inline(always)]
    pub fn set_seq_res(&mut self, val: super::vals::SeqRes) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Charger Detection Sequence Status"]
    #[inline(always)]
    pub const fn seq_stat(&self) -> super::vals::SeqStat {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::SeqStat::from_bits(val as u8)
    }
    #[doc = "Charger Detection Sequence Status"]
    #[inline(always)]
    pub fn set_seq_stat(&mut self, val: super::vals::SeqStat) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Error Flag"]
    #[inline(always)]
    pub const fn err(&self) -> super::vals::Err {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Err::from_bits(val as u8)
    }
    #[doc = "Error Flag"]
    #[inline(always)]
    pub fn set_err(&mut self, val: super::vals::Err) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Timeout Flag"]
    #[inline(always)]
    pub const fn to(&self) -> super::vals::To {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::To::from_bits(val as u8)
    }
    #[doc = "Timeout Flag"]
    #[inline(always)]
    pub fn set_to(&mut self, val: super::vals::To) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Active Status Indicator"]
    #[inline(always)]
    pub const fn active(&self) -> super::vals::Active {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Active::from_bits(val as u8)
    }
    #[doc = "Active Status Indicator"]
    #[inline(always)]
    pub fn set_active(&mut self, val: super::vals::Active) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "TIMER0 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0(pub u32);
impl Timer0 {
    #[doc = "Unit Connection Timer Elapse (in ms)"]
    #[inline(always)]
    pub const fn tunitcon(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Unit Connection Timer Elapse (in ms)"]
    #[inline(always)]
    pub fn set_tunitcon(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Sequence Initiation Time"]
    #[inline(always)]
    pub const fn tseq_init(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sequence Initiation Time"]
    #[inline(always)]
    pub fn set_tseq_init(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Timer0 {
    #[inline(always)]
    fn default() -> Timer0 {
        Timer0(0)
    }
}
#[doc = "TIMER1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1(pub u32);
impl Timer1 {
    #[doc = "Time Period Comparator Enabled"]
    #[inline(always)]
    pub const fn tvdpsrc_on(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Time Period Comparator Enabled"]
    #[inline(always)]
    pub fn set_tvdpsrc_on(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Time Period to Debounce D+ Signal"]
    #[inline(always)]
    pub const fn tdcd_dbnc(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Time Period to Debounce D+ Signal"]
    #[inline(always)]
    pub fn set_tdcd_dbnc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Timer1 {
    #[inline(always)]
    fn default() -> Timer1 {
        Timer1(0)
    }
}
#[doc = "TIMER2_BC11 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2Bc11(pub u32);
impl Timer2Bc11 {
    #[doc = "Time Before Check of D- Line"]
    #[inline(always)]
    pub const fn check_dm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Time Before Check of D- Line"]
    #[inline(always)]
    pub fn set_check_dm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    pub const fn tvdpsrc_con(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    pub fn set_tvdpsrc_con(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Timer2Bc11 {
    #[inline(always)]
    fn default() -> Timer2Bc11 {
        Timer2Bc11(0)
    }
}
#[doc = "TIMER2_BC12 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2Bc12(pub u32);
impl Timer2Bc12 {
    #[doc = "TVDMSRC_ON"]
    #[inline(always)]
    pub const fn tvdmsrc_on(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "TVDMSRC_ON"]
    #[inline(always)]
    pub fn set_tvdmsrc_on(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "TWAIT_AFTER_PRD"]
    #[inline(always)]
    pub const fn twait_after_prd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "TWAIT_AFTER_PRD"]
    #[inline(always)]
    pub fn set_twait_after_prd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Timer2Bc12 {
    #[inline(always)]
    fn default() -> Timer2Bc12 {
        Timer2Bc12(0)
    }
}
