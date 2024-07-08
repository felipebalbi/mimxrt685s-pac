#[doc = "Frequency Measurement (in Read mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmectrlR(pub u32);
impl FreqmectrlR {
    #[doc = "Result"]
    #[inline(always)]
    pub const fn result(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Result"]
    #[inline(always)]
    pub fn set_result(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Measure in Progress"]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> super::vals::FreqmectrlRMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmectrlRMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measure in Progress"]
    #[inline(always)]
    pub fn set_measure_in_progress(&mut self, val: super::vals::FreqmectrlRMeasureInProgress) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FreqmectrlR {
    #[inline(always)]
    fn default() -> FreqmectrlR {
        FreqmectrlR(0)
    }
}
#[doc = "Freqeuncy Measurement (in Write mode)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmectrlW(pub u32);
impl FreqmectrlW {
    #[doc = "Reference Clock Scaling Factor"]
    #[inline(always)]
    pub const fn ref_scale(&self) -> super::vals::RefScale {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::RefScale::from_bits(val as u8)
    }
    #[doc = "Reference Clock Scaling Factor"]
    #[inline(always)]
    pub fn set_ref_scale(&mut self, val: super::vals::RefScale) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Pulse Width Measurement mode select"]
    #[inline(always)]
    pub const fn pulse_mode(&self) -> super::vals::PulseMode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PulseMode::from_bits(val as u8)
    }
    #[doc = "Pulse Width Measurement mode select"]
    #[inline(always)]
    pub fn set_pulse_mode(&mut self, val: super::vals::PulseMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Pulse Polarity"]
    #[inline(always)]
    pub const fn pulse_pol(&self) -> super::vals::PulsePol {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::PulsePol::from_bits(val as u8)
    }
    #[doc = "Pulse Polarity"]
    #[inline(always)]
    pub fn set_pulse_pol(&mut self, val: super::vals::PulsePol) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Measure in Progress"]
    #[inline(always)]
    pub const fn measure_in_progress(&self) -> super::vals::FreqmectrlWMeasureInProgress {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::FreqmectrlWMeasureInProgress::from_bits(val as u8)
    }
    #[doc = "Measure in Progress"]
    #[inline(always)]
    pub fn set_measure_in_progress(&mut self, val: super::vals::FreqmectrlWMeasureInProgress) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FreqmectrlW {
    #[inline(always)]
    fn default() -> FreqmectrlW {
        FreqmectrlW(0)
    }
}
