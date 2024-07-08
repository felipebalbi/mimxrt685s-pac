#[doc = "Capture register ."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap(pub u32);
impl Cap {
    #[doc = "Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
    #[inline(always)]
    pub const fn cap_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Capture value for the related capture event (UTICK_CAPn. Note: the value is 1 lower than the actual value of the Micro-tick Timer at the moment of the capture event."]
    #[inline(always)]
    pub fn set_cap_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Valid. When 1, a value has been captured based on a transition of the related UTICK_CAPn pin. Cleared by writing to the related bit in the CAPCLR register."]
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cap {
    #[inline(always)]
    fn default() -> Cap {
        Cap(0)
    }
}
#[doc = "Capture clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capclr(pub u32);
impl Capclr {
    #[doc = "Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[inline(always)]
    pub const fn capclr0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 0. Writing 1 to this bit clears the CAP0 register value."]
    #[inline(always)]
    pub fn set_capclr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[inline(always)]
    pub const fn capclr1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 1. Writing 1 to this bit clears the CAP1 register value."]
    #[inline(always)]
    pub fn set_capclr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[inline(always)]
    pub const fn capclr2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 2. Writing 1 to this bit clears the CAP2 register value."]
    #[inline(always)]
    pub fn set_capclr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[inline(always)]
    pub const fn capclr3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear capture 3. Writing 1 to this bit clears the CAP3 register value."]
    #[inline(always)]
    pub fn set_capclr3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Capclr {
    #[inline(always)]
    fn default() -> Capclr {
        Capclr(0)
    }
}
#[doc = "Capture configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn capen0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 0. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn set_capen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn capen1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 1. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn set_capen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn capen2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 2. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn set_capen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub const fn capen3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Capture 3. 1 = Enabled, 0 = Disabled."]
    #[inline(always)]
    pub fn set_capen3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn cappol0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 0. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn set_cappol0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn cappol1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 1. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn set_cappol1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn cappol2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 2. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn set_cappol2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub const fn cappol3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Capture Polarity 3. 0 = Positive edge capture, 1 = Negative edge capture."]
    #[inline(always)]
    pub fn set_cappol3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
#[doc = "Control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[inline(always)]
    pub const fn delayval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Tick interval value. The delay will be equal to DELAYVAL + 1 periods of the timer clock. The minimum usable value is 1, for a delay of 2 timer clocks. A value of 0 stops the timer."]
    #[inline(always)]
    pub fn set_delayval(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[inline(always)]
    pub const fn repeat(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Repeat delay. 0 = One-time delay. 1 = Delay repeats continuously."]
    #[inline(always)]
    pub fn set_repeat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Interrupt flag. 0 = No interrupt is pending. 1 = An interrupt is pending. A write of any value to this register clears this flag."]
    #[inline(always)]
    pub const fn intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt flag. 0 = No interrupt is pending. 1 = An interrupt is pending. A write of any value to this register clears this flag."]
    #[inline(always)]
    pub fn set_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Active flag. 0 = The Micro-Tick Timer is stopped. 1 = The Micro-Tick Timer is currently active."]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Active flag. 0 = The Micro-Tick Timer is stopped. 1 = The Micro-Tick Timer is currently active."]
    #[inline(always)]
    pub fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
