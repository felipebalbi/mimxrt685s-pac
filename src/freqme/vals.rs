#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FreqmectrlRMeasureInProgress {
    #[doc = "Process complete. Measurement cycle is complete. The results are ready in the RESULT field."]
    CYCLE_DONE = 0x0,
    #[doc = "In Progress. Measurement cycle is in progress."]
    IN_PROGRESS = 0x01,
}
impl FreqmectrlRMeasureInProgress {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmectrlRMeasureInProgress {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmectrlRMeasureInProgress {
    #[inline(always)]
    fn from(val: u8) -> FreqmectrlRMeasureInProgress {
        FreqmectrlRMeasureInProgress::from_bits(val)
    }
}
impl From<FreqmectrlRMeasureInProgress> for u8 {
    #[inline(always)]
    fn from(val: FreqmectrlRMeasureInProgress) -> u8 {
        FreqmectrlRMeasureInProgress::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FreqmectrlWMeasureInProgress {
    #[doc = "Force Terminate. Forces the termination of any measurement cycle currently in progress and resets RESULT or just resets RESULT if in idle."]
    FORCE_TERMINATE = 0x0,
    #[doc = "Initiates Measurement Cycle. Initiates frequency or pulse width measurement process. Hardware clears the MEASURE_IN_PROGRESS bit when the measurement cycle completes. A new measurement starts if there is an active measurement in progress."]
    INITIATE_A_FREQME_CYCLE = 0x01,
}
impl FreqmectrlWMeasureInProgress {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmectrlWMeasureInProgress {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmectrlWMeasureInProgress {
    #[inline(always)]
    fn from(val: u8) -> FreqmectrlWMeasureInProgress {
        FreqmectrlWMeasureInProgress::from_bits(val)
    }
}
impl From<FreqmectrlWMeasureInProgress> for u8 {
    #[inline(always)]
    fn from(val: FreqmectrlWMeasureInProgress) -> u8 {
        FreqmectrlWMeasureInProgress::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PulseMode {
    #[doc = "Frequency Measurement Mode. FREQMECTRL works in a Frequency Measurement mode. Once the measurement starts (real count start is aligned at rising edge arrival on reference clock), the target counter increments by the target clock until the reference counter running by the reference clock reaches the count end point selected by REF_SCALE."]
    FREQ_ME_MODE = 0x0,
    #[doc = "Pulse Width Measurement mode. FREQMECTRL works in a Pulse Width Measurement mode, measuring the high or low period of reference clock input selected by PULSE_POL. The target counter starts incrementing by the target clock once a corresponding trigger edge (rising edge for high period measurement and falling edge for low period) occurs."]
    PULSE_ME_MODE = 0x01,
}
impl PulseMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PulseMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PulseMode {
    #[inline(always)]
    fn from(val: u8) -> PulseMode {
        PulseMode::from_bits(val)
    }
}
impl From<PulseMode> for u8 {
    #[inline(always)]
    fn from(val: PulseMode) -> u8 {
        PulseMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum PulsePol {
    #[doc = "High Period. High period of reference clock is measured in Pulse Width Measurement mode triggered by the rising edge on the reference clock input."]
    HIGH_PERIOD = 0x0,
    #[doc = "Low Period. Low period of reference clock is measured in Pulse Width Measurement mode triggered by the falling edge on the reference clock input."]
    LOW_PERIOD = 0x01,
}
impl PulsePol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PulsePol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PulsePol {
    #[inline(always)]
    fn from(val: u8) -> PulsePol {
        PulsePol::from_bits(val)
    }
}
impl From<PulsePol> for u8 {
    #[inline(always)]
    fn from(val: PulsePol) -> u8 {
        PulsePol::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct RefScale(pub u8);
impl RefScale {
    #[doc = "Count cycle = 2^0 = 1"]
    pub const COUNTCYCLE_1: Self = Self(0x0);
    #[doc = "Count cycle = 2^1 = 2"]
    pub const COUNTCYCLE_2: Self = Self(0x01);
    #[doc = "Count cycle = 2^4 = 4"]
    pub const COUNTCYCLE_4: Self = Self(0x02);
    #[doc = "Count cycle = 2^31 = 2,147,483,648"]
    pub const COUNTCYCLE_31: Self = Self(0x1f);
}
impl RefScale {
    pub const fn from_bits(val: u8) -> RefScale {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl From<u8> for RefScale {
    #[inline(always)]
    fn from(val: u8) -> RefScale {
        RefScale::from_bits(val)
    }
}
impl From<RefScale> for u8 {
    #[inline(always)]
    fn from(val: RefScale) -> u8 {
        RefScale::to_bits(val)
    }
}
