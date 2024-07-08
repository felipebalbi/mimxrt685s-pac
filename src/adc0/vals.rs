#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adcen {
    #[doc = "ADC is disabled."]
    ADCEN_0 = 0x0,
    #[doc = "ADC is enabled."]
    ADCEN_1 = 0x01,
}
impl Adcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adcen {
    #[inline(always)]
    fn from(val: u8) -> Adcen {
        Adcen::from_bits(val)
    }
}
impl From<Adcen> for u8 {
    #[inline(always)]
    fn from(val: Adcen) -> u8 {
        Adcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Calofsi {
    #[doc = "Offset calibration and offset trimming not implemented."]
    CALOFSI_0 = 0x0,
    #[doc = "Offset calibration and offset trimming implemented."]
    CALOFSI_1 = 0x01,
}
impl Calofsi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calofsi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calofsi {
    #[inline(always)]
    fn from(val: u8) -> Calofsi {
        Calofsi::from_bits(val)
    }
}
impl From<Calofsi> for u8 {
    #[inline(always)]
    fn from(val: Calofsi) -> u8 {
        Calofsi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdact {
    #[doc = "No command is currently in progress."]
    CMDACT_0 = 0x0,
    #[doc = "Command 1 currently being executed."]
    CMDACT_1 = 0x01,
    #[doc = "Command 2 currently being executed."]
    CMDACT_2 = 0x02,
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
    _RESERVED_e = 0x0e,
    #[doc = "Command 15 currently being executed."]
    CMDACT_15 = 0x0f,
}
impl Cmdact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdact {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdact {
    #[inline(always)]
    fn from(val: u8) -> Cmdact {
        Cmdact::from_bits(val)
    }
}
impl From<Cmdact> for u8 {
    #[inline(always)]
    fn from(val: Cmdact) -> u8 {
        Cmdact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh10Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh10Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Avgs {
        Cmdh10Avgs::from_bits(val)
    }
}
impl From<Cmdh10Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Avgs) -> u8 {
        Cmdh10Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh10Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh10Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Loop {
        Cmdh10Loop::from_bits(val)
    }
}
impl From<Cmdh10Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Loop) -> u8 {
        Cmdh10Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh10Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh10Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Lwi {
        Cmdh10Lwi::from_bits(val)
    }
}
impl From<Cmdh10Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Lwi) -> u8 {
        Cmdh10Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh10Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh10Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Next {
        Cmdh10Next::from_bits(val)
    }
}
impl From<Cmdh10Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Next) -> u8 {
        Cmdh10Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh10Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh10Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh10Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh10Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh10Sts {
        Cmdh10Sts::from_bits(val)
    }
}
impl From<Cmdh10Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh10Sts) -> u8 {
        Cmdh10Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh11Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh11Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Avgs {
        Cmdh11Avgs::from_bits(val)
    }
}
impl From<Cmdh11Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Avgs) -> u8 {
        Cmdh11Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh11Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh11Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Loop {
        Cmdh11Loop::from_bits(val)
    }
}
impl From<Cmdh11Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Loop) -> u8 {
        Cmdh11Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh11Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh11Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Lwi {
        Cmdh11Lwi::from_bits(val)
    }
}
impl From<Cmdh11Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Lwi) -> u8 {
        Cmdh11Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh11Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh11Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Next {
        Cmdh11Next::from_bits(val)
    }
}
impl From<Cmdh11Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Next) -> u8 {
        Cmdh11Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh11Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh11Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh11Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh11Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh11Sts {
        Cmdh11Sts::from_bits(val)
    }
}
impl From<Cmdh11Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh11Sts) -> u8 {
        Cmdh11Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh12Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh12Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Avgs {
        Cmdh12Avgs::from_bits(val)
    }
}
impl From<Cmdh12Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Avgs) -> u8 {
        Cmdh12Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh12Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh12Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Loop {
        Cmdh12Loop::from_bits(val)
    }
}
impl From<Cmdh12Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Loop) -> u8 {
        Cmdh12Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh12Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh12Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Lwi {
        Cmdh12Lwi::from_bits(val)
    }
}
impl From<Cmdh12Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Lwi) -> u8 {
        Cmdh12Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh12Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh12Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Next {
        Cmdh12Next::from_bits(val)
    }
}
impl From<Cmdh12Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Next) -> u8 {
        Cmdh12Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh12Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh12Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh12Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh12Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh12Sts {
        Cmdh12Sts::from_bits(val)
    }
}
impl From<Cmdh12Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh12Sts) -> u8 {
        Cmdh12Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh13Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh13Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Avgs {
        Cmdh13Avgs::from_bits(val)
    }
}
impl From<Cmdh13Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Avgs) -> u8 {
        Cmdh13Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh13Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh13Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Loop {
        Cmdh13Loop::from_bits(val)
    }
}
impl From<Cmdh13Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Loop) -> u8 {
        Cmdh13Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh13Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh13Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Lwi {
        Cmdh13Lwi::from_bits(val)
    }
}
impl From<Cmdh13Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Lwi) -> u8 {
        Cmdh13Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh13Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh13Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Next {
        Cmdh13Next::from_bits(val)
    }
}
impl From<Cmdh13Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Next) -> u8 {
        Cmdh13Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh13Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh13Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh13Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh13Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh13Sts {
        Cmdh13Sts::from_bits(val)
    }
}
impl From<Cmdh13Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh13Sts) -> u8 {
        Cmdh13Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh14Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh14Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Avgs {
        Cmdh14Avgs::from_bits(val)
    }
}
impl From<Cmdh14Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Avgs) -> u8 {
        Cmdh14Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh14Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh14Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Loop {
        Cmdh14Loop::from_bits(val)
    }
}
impl From<Cmdh14Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Loop) -> u8 {
        Cmdh14Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh14Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh14Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Lwi {
        Cmdh14Lwi::from_bits(val)
    }
}
impl From<Cmdh14Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Lwi) -> u8 {
        Cmdh14Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh14Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh14Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Next {
        Cmdh14Next::from_bits(val)
    }
}
impl From<Cmdh14Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Next) -> u8 {
        Cmdh14Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh14Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh14Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh14Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh14Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh14Sts {
        Cmdh14Sts::from_bits(val)
    }
}
impl From<Cmdh14Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh14Sts) -> u8 {
        Cmdh14Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh15Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh15Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Avgs {
        Cmdh15Avgs::from_bits(val)
    }
}
impl From<Cmdh15Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Avgs) -> u8 {
        Cmdh15Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh15Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh15Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Loop {
        Cmdh15Loop::from_bits(val)
    }
}
impl From<Cmdh15Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Loop) -> u8 {
        Cmdh15Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh15Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh15Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Lwi {
        Cmdh15Lwi::from_bits(val)
    }
}
impl From<Cmdh15Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Lwi) -> u8 {
        Cmdh15Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh15Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh15Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Next {
        Cmdh15Next::from_bits(val)
    }
}
impl From<Cmdh15Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Next) -> u8 {
        Cmdh15Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh15Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh15Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh15Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh15Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh15Sts {
        Cmdh15Sts::from_bits(val)
    }
}
impl From<Cmdh15Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh15Sts) -> u8 {
        Cmdh15Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh1Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh1Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Avgs {
        Cmdh1Avgs::from_bits(val)
    }
}
impl From<Cmdh1Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Avgs) -> u8 {
        Cmdh1Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh1Cmpen {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl Cmdh1Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Cmpen {
        Cmdh1Cmpen::from_bits(val)
    }
}
impl From<Cmdh1Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Cmpen) -> u8 {
        Cmdh1Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh1Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh1Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Loop {
        Cmdh1Loop::from_bits(val)
    }
}
impl From<Cmdh1Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Loop) -> u8 {
        Cmdh1Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh1Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh1Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Lwi {
        Cmdh1Lwi::from_bits(val)
    }
}
impl From<Cmdh1Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Lwi) -> u8 {
        Cmdh1Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh1Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh1Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Next {
        Cmdh1Next::from_bits(val)
    }
}
impl From<Cmdh1Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Next) -> u8 {
        Cmdh1Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh1Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh1Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh1Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh1Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh1Sts {
        Cmdh1Sts::from_bits(val)
    }
}
impl From<Cmdh1Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh1Sts) -> u8 {
        Cmdh1Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh2Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh2Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Avgs {
        Cmdh2Avgs::from_bits(val)
    }
}
impl From<Cmdh2Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Avgs) -> u8 {
        Cmdh2Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh2Cmpen {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl Cmdh2Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Cmpen {
        Cmdh2Cmpen::from_bits(val)
    }
}
impl From<Cmdh2Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Cmpen) -> u8 {
        Cmdh2Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh2Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh2Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Loop {
        Cmdh2Loop::from_bits(val)
    }
}
impl From<Cmdh2Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Loop) -> u8 {
        Cmdh2Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh2Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh2Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Lwi {
        Cmdh2Lwi::from_bits(val)
    }
}
impl From<Cmdh2Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Lwi) -> u8 {
        Cmdh2Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh2Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh2Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Next {
        Cmdh2Next::from_bits(val)
    }
}
impl From<Cmdh2Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Next) -> u8 {
        Cmdh2Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh2Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh2Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh2Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh2Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh2Sts {
        Cmdh2Sts::from_bits(val)
    }
}
impl From<Cmdh2Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh2Sts) -> u8 {
        Cmdh2Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh3Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh3Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Avgs {
        Cmdh3Avgs::from_bits(val)
    }
}
impl From<Cmdh3Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Avgs) -> u8 {
        Cmdh3Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh3Cmpen {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl Cmdh3Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Cmpen {
        Cmdh3Cmpen::from_bits(val)
    }
}
impl From<Cmdh3Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Cmpen) -> u8 {
        Cmdh3Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh3Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh3Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Loop {
        Cmdh3Loop::from_bits(val)
    }
}
impl From<Cmdh3Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Loop) -> u8 {
        Cmdh3Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh3Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh3Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Lwi {
        Cmdh3Lwi::from_bits(val)
    }
}
impl From<Cmdh3Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Lwi) -> u8 {
        Cmdh3Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh3Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh3Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Next {
        Cmdh3Next::from_bits(val)
    }
}
impl From<Cmdh3Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Next) -> u8 {
        Cmdh3Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh3Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh3Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh3Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh3Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh3Sts {
        Cmdh3Sts::from_bits(val)
    }
}
impl From<Cmdh3Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh3Sts) -> u8 {
        Cmdh3Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh4Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh4Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Avgs {
        Cmdh4Avgs::from_bits(val)
    }
}
impl From<Cmdh4Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Avgs) -> u8 {
        Cmdh4Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh4Cmpen {
    #[doc = "Compare disabled."]
    CMPEN_0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Compare enabled. Store on true."]
    CMPEN_2 = 0x02,
    #[doc = "Compare enabled. Repeat channel acquisition (sample/convert/compare) until true."]
    CMPEN_3 = 0x03,
}
impl Cmdh4Cmpen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Cmpen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Cmpen {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Cmpen {
        Cmdh4Cmpen::from_bits(val)
    }
}
impl From<Cmdh4Cmpen> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Cmpen) -> u8 {
        Cmdh4Cmpen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh4Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh4Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Loop {
        Cmdh4Loop::from_bits(val)
    }
}
impl From<Cmdh4Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Loop) -> u8 {
        Cmdh4Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh4Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh4Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Lwi {
        Cmdh4Lwi::from_bits(val)
    }
}
impl From<Cmdh4Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Lwi) -> u8 {
        Cmdh4Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh4Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh4Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Next {
        Cmdh4Next::from_bits(val)
    }
}
impl From<Cmdh4Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Next) -> u8 {
        Cmdh4Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh4Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh4Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh4Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh4Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh4Sts {
        Cmdh4Sts::from_bits(val)
    }
}
impl From<Cmdh4Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh4Sts) -> u8 {
        Cmdh4Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh5Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh5Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Avgs {
        Cmdh5Avgs::from_bits(val)
    }
}
impl From<Cmdh5Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Avgs) -> u8 {
        Cmdh5Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh5Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh5Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Loop {
        Cmdh5Loop::from_bits(val)
    }
}
impl From<Cmdh5Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Loop) -> u8 {
        Cmdh5Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh5Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh5Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Lwi {
        Cmdh5Lwi::from_bits(val)
    }
}
impl From<Cmdh5Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Lwi) -> u8 {
        Cmdh5Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh5Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh5Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Next {
        Cmdh5Next::from_bits(val)
    }
}
impl From<Cmdh5Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Next) -> u8 {
        Cmdh5Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh5Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh5Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh5Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh5Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh5Sts {
        Cmdh5Sts::from_bits(val)
    }
}
impl From<Cmdh5Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh5Sts) -> u8 {
        Cmdh5Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh6Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh6Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Avgs {
        Cmdh6Avgs::from_bits(val)
    }
}
impl From<Cmdh6Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Avgs) -> u8 {
        Cmdh6Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh6Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh6Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Loop {
        Cmdh6Loop::from_bits(val)
    }
}
impl From<Cmdh6Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Loop) -> u8 {
        Cmdh6Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh6Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh6Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Lwi {
        Cmdh6Lwi::from_bits(val)
    }
}
impl From<Cmdh6Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Lwi) -> u8 {
        Cmdh6Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh6Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh6Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Next {
        Cmdh6Next::from_bits(val)
    }
}
impl From<Cmdh6Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Next) -> u8 {
        Cmdh6Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh6Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh6Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh6Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh6Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh6Sts {
        Cmdh6Sts::from_bits(val)
    }
}
impl From<Cmdh6Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh6Sts) -> u8 {
        Cmdh6Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh7Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh7Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Avgs {
        Cmdh7Avgs::from_bits(val)
    }
}
impl From<Cmdh7Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Avgs) -> u8 {
        Cmdh7Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh7Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh7Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Loop {
        Cmdh7Loop::from_bits(val)
    }
}
impl From<Cmdh7Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Loop) -> u8 {
        Cmdh7Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh7Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh7Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Lwi {
        Cmdh7Lwi::from_bits(val)
    }
}
impl From<Cmdh7Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Lwi) -> u8 {
        Cmdh7Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh7Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh7Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Next {
        Cmdh7Next::from_bits(val)
    }
}
impl From<Cmdh7Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Next) -> u8 {
        Cmdh7Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh7Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh7Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh7Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh7Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh7Sts {
        Cmdh7Sts::from_bits(val)
    }
}
impl From<Cmdh7Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh7Sts) -> u8 {
        Cmdh7Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh8Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh8Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Avgs {
        Cmdh8Avgs::from_bits(val)
    }
}
impl From<Cmdh8Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Avgs) -> u8 {
        Cmdh8Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh8Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh8Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Loop {
        Cmdh8Loop::from_bits(val)
    }
}
impl From<Cmdh8Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Loop) -> u8 {
        Cmdh8Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh8Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh8Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Lwi {
        Cmdh8Lwi::from_bits(val)
    }
}
impl From<Cmdh8Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Lwi) -> u8 {
        Cmdh8Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh8Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh8Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Next {
        Cmdh8Next::from_bits(val)
    }
}
impl From<Cmdh8Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Next) -> u8 {
        Cmdh8Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh8Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh8Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh8Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh8Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh8Sts {
        Cmdh8Sts::from_bits(val)
    }
}
impl From<Cmdh8Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh8Sts) -> u8 {
        Cmdh8Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh9Avgs {
    #[doc = "Single conversion."]
    AVGS_0 = 0x0,
    #[doc = "2 conversions averaged."]
    AVGS_1 = 0x01,
    #[doc = "4 conversions averaged."]
    AVGS_2 = 0x02,
    #[doc = "8 conversions averaged."]
    AVGS_3 = 0x03,
    #[doc = "16 conversions averaged."]
    AVGS_4 = 0x04,
    #[doc = "32 conversions averaged."]
    AVGS_5 = 0x05,
    #[doc = "64 conversions averaged."]
    AVGS_6 = 0x06,
    #[doc = "128 conversions averaged."]
    AVGS_7 = 0x07,
}
impl Cmdh9Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Avgs {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Avgs {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Avgs {
        Cmdh9Avgs::from_bits(val)
    }
}
impl From<Cmdh9Avgs> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Avgs) -> u8 {
        Cmdh9Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh9Loop {
    #[doc = "Looping not enabled. Command executes 1 time."]
    LOOP_0 = 0x0,
    #[doc = "Loop 1 time. Command executes 2 times."]
    LOOP_1 = 0x01,
    #[doc = "Loop 2 times. Command executes 3 times."]
    LOOP_2 = 0x02,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_3 = 0x03,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_4 = 0x04,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_5 = 0x05,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_6 = 0x06,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_7 = 0x07,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_8 = 0x08,
    #[doc = "Loop corresponding number of times. Command executes LOOP+1 times."]
    LOOP_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Loop 15 times. Command executes 16 times."]
    LOOP_15 = 0x0f,
}
impl Cmdh9Loop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Loop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Loop {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Loop {
        Cmdh9Loop::from_bits(val)
    }
}
impl From<Cmdh9Loop> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Loop) -> u8 {
        Cmdh9Loop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh9Lwi {
    #[doc = "Auto channel increment disabled"]
    LWI_0 = 0x0,
    #[doc = "Auto channel increment enabled"]
    LWI_1 = 0x01,
}
impl Cmdh9Lwi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Lwi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Lwi {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Lwi {
        Cmdh9Lwi::from_bits(val)
    }
}
impl From<Cmdh9Lwi> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Lwi) -> u8 {
        Cmdh9Lwi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh9Next {
    #[doc = "No next command defined. Terminate conversions at completion of current command. If lower priority trigger pending, begin command associated with lower priority trigger."]
    NEXT_0 = 0x0,
    #[doc = "Select CMD1 command buffer register as next command."]
    NEXT_1 = 0x01,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_2 = 0x02,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_3 = 0x03,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_4 = 0x04,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_5 = 0x05,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_6 = 0x06,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_7 = 0x07,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_8 = 0x08,
    #[doc = "Select corresponding CMD command buffer register as next command"]
    NEXT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select CMD15 command buffer register as next command."]
    NEXT_15 = 0x0f,
}
impl Cmdh9Next {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Next {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Next {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Next {
        Cmdh9Next::from_bits(val)
    }
}
impl From<Cmdh9Next> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Next) -> u8 {
        Cmdh9Next::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdh9Sts {
    #[doc = "Minimum sample time of 3 ADCK cycles."]
    STS_0 = 0x0,
    #[doc = "3 + 2^1 ADCK cycles; 5 ADCK cycles total sample time."]
    STS_1 = 0x01,
    #[doc = "3 + 2^2 ADCK cycles; 7 ADCK cycles total sample time."]
    STS_2 = 0x02,
    #[doc = "3 + 2^3 ADCK cycles; 11 ADCK cycles total sample time."]
    STS_3 = 0x03,
    #[doc = "3 + 2^4 ADCK cycles; 19 ADCK cycles total sample time."]
    STS_4 = 0x04,
    #[doc = "3 + 2^5 ADCK cycles; 35 ADCK cycles total sample time."]
    STS_5 = 0x05,
    #[doc = "3 + 2^6 ADCK cycles; 67 ADCK cycles total sample time."]
    STS_6 = 0x06,
    #[doc = "3 + 2^7 ADCK cycles; 131 ADCK cycles total sample time."]
    STS_7 = 0x07,
}
impl Cmdh9Sts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdh9Sts {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdh9Sts {
    #[inline(always)]
    fn from(val: u8) -> Cmdh9Sts {
        Cmdh9Sts::from_bits(val)
    }
}
impl From<Cmdh9Sts> for u8 {
    #[inline(always)]
    fn from(val: Cmdh9Sts) -> u8 {
        Cmdh9Sts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl10Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl10Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Absel {
        Cmdl10Absel::from_bits(val)
    }
}
impl From<Cmdl10Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Absel) -> u8 {
        Cmdl10Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl10Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl10Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Adch {
        Cmdl10Adch::from_bits(val)
    }
}
impl From<Cmdl10Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Adch) -> u8 {
        Cmdl10Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl10Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl10Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Cscale {
        Cmdl10Cscale::from_bits(val)
    }
}
impl From<Cmdl10Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Cscale) -> u8 {
        Cmdl10Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl10Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl10Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl10Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl10Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl10Diff {
        Cmdl10Diff::from_bits(val)
    }
}
impl From<Cmdl10Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl10Diff) -> u8 {
        Cmdl10Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl11Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl11Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Absel {
        Cmdl11Absel::from_bits(val)
    }
}
impl From<Cmdl11Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Absel) -> u8 {
        Cmdl11Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl11Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl11Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Adch {
        Cmdl11Adch::from_bits(val)
    }
}
impl From<Cmdl11Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Adch) -> u8 {
        Cmdl11Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl11Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl11Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Cscale {
        Cmdl11Cscale::from_bits(val)
    }
}
impl From<Cmdl11Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Cscale) -> u8 {
        Cmdl11Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl11Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl11Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl11Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl11Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl11Diff {
        Cmdl11Diff::from_bits(val)
    }
}
impl From<Cmdl11Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl11Diff) -> u8 {
        Cmdl11Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl12Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl12Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Absel {
        Cmdl12Absel::from_bits(val)
    }
}
impl From<Cmdl12Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Absel) -> u8 {
        Cmdl12Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl12Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl12Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Adch {
        Cmdl12Adch::from_bits(val)
    }
}
impl From<Cmdl12Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Adch) -> u8 {
        Cmdl12Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl12Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl12Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Cscale {
        Cmdl12Cscale::from_bits(val)
    }
}
impl From<Cmdl12Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Cscale) -> u8 {
        Cmdl12Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl12Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl12Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl12Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl12Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl12Diff {
        Cmdl12Diff::from_bits(val)
    }
}
impl From<Cmdl12Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl12Diff) -> u8 {
        Cmdl12Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl13Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl13Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Absel {
        Cmdl13Absel::from_bits(val)
    }
}
impl From<Cmdl13Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Absel) -> u8 {
        Cmdl13Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl13Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl13Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Adch {
        Cmdl13Adch::from_bits(val)
    }
}
impl From<Cmdl13Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Adch) -> u8 {
        Cmdl13Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl13Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl13Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Cscale {
        Cmdl13Cscale::from_bits(val)
    }
}
impl From<Cmdl13Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Cscale) -> u8 {
        Cmdl13Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl13Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl13Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl13Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl13Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl13Diff {
        Cmdl13Diff::from_bits(val)
    }
}
impl From<Cmdl13Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl13Diff) -> u8 {
        Cmdl13Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl14Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl14Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Absel {
        Cmdl14Absel::from_bits(val)
    }
}
impl From<Cmdl14Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Absel) -> u8 {
        Cmdl14Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl14Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl14Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Adch {
        Cmdl14Adch::from_bits(val)
    }
}
impl From<Cmdl14Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Adch) -> u8 {
        Cmdl14Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl14Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl14Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Cscale {
        Cmdl14Cscale::from_bits(val)
    }
}
impl From<Cmdl14Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Cscale) -> u8 {
        Cmdl14Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl14Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl14Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl14Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl14Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl14Diff {
        Cmdl14Diff::from_bits(val)
    }
}
impl From<Cmdl14Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl14Diff) -> u8 {
        Cmdl14Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl15Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl15Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Absel {
        Cmdl15Absel::from_bits(val)
    }
}
impl From<Cmdl15Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Absel) -> u8 {
        Cmdl15Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl15Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl15Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Adch {
        Cmdl15Adch::from_bits(val)
    }
}
impl From<Cmdl15Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Adch) -> u8 {
        Cmdl15Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl15Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl15Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Cscale {
        Cmdl15Cscale::from_bits(val)
    }
}
impl From<Cmdl15Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Cscale) -> u8 {
        Cmdl15Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl15Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl15Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl15Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl15Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl15Diff {
        Cmdl15Diff::from_bits(val)
    }
}
impl From<Cmdl15Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl15Diff) -> u8 {
        Cmdl15Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl1Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl1Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Absel {
        Cmdl1Absel::from_bits(val)
    }
}
impl From<Cmdl1Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Absel) -> u8 {
        Cmdl1Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl1Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl1Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Adch {
        Cmdl1Adch::from_bits(val)
    }
}
impl From<Cmdl1Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Adch) -> u8 {
        Cmdl1Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl1Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl1Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Cscale {
        Cmdl1Cscale::from_bits(val)
    }
}
impl From<Cmdl1Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Cscale) -> u8 {
        Cmdl1Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl1Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl1Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl1Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl1Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl1Diff {
        Cmdl1Diff::from_bits(val)
    }
}
impl From<Cmdl1Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl1Diff) -> u8 {
        Cmdl1Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl2Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl2Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Absel {
        Cmdl2Absel::from_bits(val)
    }
}
impl From<Cmdl2Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Absel) -> u8 {
        Cmdl2Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl2Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl2Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Adch {
        Cmdl2Adch::from_bits(val)
    }
}
impl From<Cmdl2Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Adch) -> u8 {
        Cmdl2Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl2Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl2Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Cscale {
        Cmdl2Cscale::from_bits(val)
    }
}
impl From<Cmdl2Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Cscale) -> u8 {
        Cmdl2Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl2Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl2Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl2Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl2Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl2Diff {
        Cmdl2Diff::from_bits(val)
    }
}
impl From<Cmdl2Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl2Diff) -> u8 {
        Cmdl2Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl3Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl3Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Absel {
        Cmdl3Absel::from_bits(val)
    }
}
impl From<Cmdl3Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Absel) -> u8 {
        Cmdl3Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl3Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl3Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Adch {
        Cmdl3Adch::from_bits(val)
    }
}
impl From<Cmdl3Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Adch) -> u8 {
        Cmdl3Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl3Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl3Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Cscale {
        Cmdl3Cscale::from_bits(val)
    }
}
impl From<Cmdl3Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Cscale) -> u8 {
        Cmdl3Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl3Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl3Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl3Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl3Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl3Diff {
        Cmdl3Diff::from_bits(val)
    }
}
impl From<Cmdl3Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl3Diff) -> u8 {
        Cmdl3Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl4Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl4Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Absel {
        Cmdl4Absel::from_bits(val)
    }
}
impl From<Cmdl4Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Absel) -> u8 {
        Cmdl4Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl4Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl4Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Adch {
        Cmdl4Adch::from_bits(val)
    }
}
impl From<Cmdl4Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Adch) -> u8 {
        Cmdl4Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl4Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl4Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Cscale {
        Cmdl4Cscale::from_bits(val)
    }
}
impl From<Cmdl4Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Cscale) -> u8 {
        Cmdl4Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl4Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl4Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl4Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl4Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl4Diff {
        Cmdl4Diff::from_bits(val)
    }
}
impl From<Cmdl4Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl4Diff) -> u8 {
        Cmdl4Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl5Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl5Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Absel {
        Cmdl5Absel::from_bits(val)
    }
}
impl From<Cmdl5Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Absel) -> u8 {
        Cmdl5Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl5Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl5Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Adch {
        Cmdl5Adch::from_bits(val)
    }
}
impl From<Cmdl5Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Adch) -> u8 {
        Cmdl5Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl5Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl5Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Cscale {
        Cmdl5Cscale::from_bits(val)
    }
}
impl From<Cmdl5Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Cscale) -> u8 {
        Cmdl5Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl5Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl5Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl5Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl5Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl5Diff {
        Cmdl5Diff::from_bits(val)
    }
}
impl From<Cmdl5Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl5Diff) -> u8 {
        Cmdl5Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl6Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl6Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Absel {
        Cmdl6Absel::from_bits(val)
    }
}
impl From<Cmdl6Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Absel) -> u8 {
        Cmdl6Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl6Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl6Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Adch {
        Cmdl6Adch::from_bits(val)
    }
}
impl From<Cmdl6Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Adch) -> u8 {
        Cmdl6Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl6Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl6Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Cscale {
        Cmdl6Cscale::from_bits(val)
    }
}
impl From<Cmdl6Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Cscale) -> u8 {
        Cmdl6Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl6Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl6Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl6Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl6Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl6Diff {
        Cmdl6Diff::from_bits(val)
    }
}
impl From<Cmdl6Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl6Diff) -> u8 {
        Cmdl6Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl7Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl7Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Absel {
        Cmdl7Absel::from_bits(val)
    }
}
impl From<Cmdl7Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Absel) -> u8 {
        Cmdl7Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl7Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl7Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Adch {
        Cmdl7Adch::from_bits(val)
    }
}
impl From<Cmdl7Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Adch) -> u8 {
        Cmdl7Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl7Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl7Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Cscale {
        Cmdl7Cscale::from_bits(val)
    }
}
impl From<Cmdl7Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Cscale) -> u8 {
        Cmdl7Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl7Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl7Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl7Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl7Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl7Diff {
        Cmdl7Diff::from_bits(val)
    }
}
impl From<Cmdl7Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl7Diff) -> u8 {
        Cmdl7Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl8Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl8Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Absel {
        Cmdl8Absel::from_bits(val)
    }
}
impl From<Cmdl8Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Absel) -> u8 {
        Cmdl8Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl8Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl8Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Adch {
        Cmdl8Adch::from_bits(val)
    }
}
impl From<Cmdl8Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Adch) -> u8 {
        Cmdl8Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl8Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl8Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Cscale {
        Cmdl8Cscale::from_bits(val)
    }
}
impl From<Cmdl8Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Cscale) -> u8 {
        Cmdl8Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl8Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl8Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl8Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl8Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl8Diff {
        Cmdl8Diff::from_bits(val)
    }
}
impl From<Cmdl8Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl8Diff) -> u8 {
        Cmdl8Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl9Absel {
    #[doc = "When DIFF=0b0, the associated A-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnA-CHnB)."]
    ABSEL_0 = 0x0,
    #[doc = "When DIFF=0b0, the associated B-side channel is converted as single-ended. When DIFF=0b1, the ADC result is (CHnB-CHnA)."]
    ABSEL_1 = 0x01,
}
impl Cmdl9Absel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Absel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Absel {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Absel {
        Cmdl9Absel::from_bits(val)
    }
}
impl From<Cmdl9Absel> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Absel) -> u8 {
        Cmdl9Absel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl9Adch {
    #[doc = "Select CH0A or CH0B or CH0A/CH0B pair."]
    ADCH_0 = 0x0,
    #[doc = "Select CH1A or CH1B or CH1A/CH1B pair."]
    ADCH_1 = 0x01,
    #[doc = "Select CH2A or CH2B or CH2A/CH2B pair."]
    ADCH_2 = 0x02,
    #[doc = "Select CH3A or CH3B or CH3A/CH3B pair."]
    ADCH_3 = 0x03,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_4 = 0x04,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_5 = 0x05,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_6 = 0x06,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_7 = 0x07,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_8 = 0x08,
    #[doc = "Select corresponding channel CHnA or CHnB or CHnA/CHnB pair."]
    ADCH_9 = 0x09,
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
    #[doc = "Select CH30A or CH30B or CH30A/CH30B pair."]
    ADCH_30 = 0x1e,
    #[doc = "Select CH31A or CH31B or CH31A/CH31B pair."]
    ADCH_31 = 0x1f,
}
impl Cmdl9Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Adch {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Adch {
        Cmdl9Adch::from_bits(val)
    }
}
impl From<Cmdl9Adch> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Adch) -> u8 {
        Cmdl9Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl9Cscale {
    #[doc = "Scale selected analog channel (Factor of 30/64)"]
    CSCALE_0 = 0x0,
    #[doc = "(Default) Full scale (Factor of 1)"]
    CSCALE_1 = 0x01,
}
impl Cmdl9Cscale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Cscale {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Cscale {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Cscale {
        Cmdl9Cscale::from_bits(val)
    }
}
impl From<Cmdl9Cscale> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Cscale) -> u8 {
        Cmdl9Cscale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdl9Diff {
    #[doc = "Single-ended mode."]
    DIFF_0 = 0x0,
    #[doc = "Differential mode."]
    DIFF_1 = 0x01,
}
impl Cmdl9Diff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdl9Diff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdl9Diff {
    #[inline(always)]
    fn from(val: u8) -> Cmdl9Diff {
        Cmdl9Diff::from_bits(val)
    }
}
impl From<Cmdl9Diff> for u8 {
    #[inline(always)]
    fn from(val: Cmdl9Diff) -> u8 {
        Cmdl9Diff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cmdsrc {
    #[doc = "Not a valid value CMDSRC value for a dataword in RESFIFO. 0x0 is only found in initial FIFO state prior to an ADC conversion result dataword being stored to a RESFIFO buffer."]
    CMDSRC_0 = 0x0,
    #[doc = "CMD1 buffer used as control settings for this conversion."]
    CMDSRC_1 = 0x01,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_2 = 0x02,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_3 = 0x03,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_4 = 0x04,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_5 = 0x05,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_6 = 0x06,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_7 = 0x07,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_8 = 0x08,
    #[doc = "Corresponding command buffer used as control settings for this conversion."]
    CMDSRC_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15 buffer used as control settings for this conversion."]
    CMDSRC_15 = 0x0f,
}
impl Cmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Cmdsrc {
        Cmdsrc::from_bits(val)
    }
}
impl From<Cmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Cmdsrc) -> u8 {
        Cmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Csw {
    #[doc = "Channel scaling not supported."]
    CSW_0 = 0x0,
    #[doc = "Channel scaling supported. 1-bit CSCALE control field."]
    CSW_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Channel scaling supported. 6-bit CSCALE control field."]
    CSW_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Csw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csw {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csw {
    #[inline(always)]
    fn from(val: u8) -> Csw {
        Csw::from_bits(val)
    }
}
impl From<Csw> for u8 {
    #[inline(always)]
    fn from(val: Csw) -> u8 {
        Csw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Diffen {
    #[doc = "Differential operation not supported."]
    DIFFEN_0 = 0x0,
    #[doc = "Differential operation supported. CMDLa\\[DIFF\\] and CMDLa\\[ABSEL\\] control fields implemented."]
    DIFFEN_1 = 0x01,
}
impl Diffen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Diffen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Diffen {
    #[inline(always)]
    fn from(val: u8) -> Diffen {
        Diffen::from_bits(val)
    }
}
impl From<Diffen> for u8 {
    #[inline(always)]
    fn from(val: Diffen) -> u8 {
        Diffen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Dozen {
    #[doc = "ADC is enabled in Doze mode."]
    DOZEN_0 = 0x0,
    #[doc = "ADC is disabled in Doze mode."]
    DOZEN_1 = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fifosize {
    _RESERVED_0 = 0x0,
    #[doc = "Result FIFO depth = 1 dataword."]
    FIFOSIZE_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Result FIFO depth = 4 datawords."]
    FIFOSIZE_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Result FIFO depth = 8 datawords."]
    FIFOSIZE_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Result FIFO depth = 16 datawords."]
    FIFOSIZE_16 = 0x10,
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
    #[doc = "Result FIFO depth = 32 datawords."]
    FIFOSIZE_32 = 0x20,
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
    #[doc = "Result FIFO depth = 64 datawords."]
    FIFOSIZE_64 = 0x40,
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
impl Fifosize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fifosize {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fifosize {
    #[inline(always)]
    fn from(val: u8) -> Fifosize {
        Fifosize::from_bits(val)
    }
}
impl From<Fifosize> for u8 {
    #[inline(always)]
    fn from(val: Fifosize) -> u8 {
        Fifosize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fof {
    #[doc = "No result FIFO overflow has occurred since the last time the flag was cleared."]
    FOF_0 = 0x0,
    #[doc = "At least one result FIFO overflow has occurred since the last time the flag was cleared."]
    FOF_1 = 0x01,
}
impl Fof {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fof {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fof {
    #[inline(always)]
    fn from(val: u8) -> Fof {
        Fof::from_bits(val)
    }
}
impl From<Fof> for u8 {
    #[inline(always)]
    fn from(val: Fof) -> u8 {
        Fof::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fofie {
    #[doc = "FIFO overflow interrupts are not enabled."]
    FOFIE_0 = 0x0,
    #[doc = "FIFO overflow interrupts are enabled."]
    FOFIE_1 = 0x01,
}
impl Fofie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fofie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fofie {
    #[inline(always)]
    fn from(val: u8) -> Fofie {
        Fofie::from_bits(val)
    }
}
impl From<Fofie> for u8 {
    #[inline(always)]
    fn from(val: Fofie) -> u8 {
        Fofie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fwmde {
    #[doc = "DMA request disabled."]
    FWMDE_0 = 0x0,
    #[doc = "DMA request enabled."]
    FWMDE_1 = 0x01,
}
impl Fwmde {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fwmde {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fwmde {
    #[inline(always)]
    fn from(val: u8) -> Fwmde {
        Fwmde::from_bits(val)
    }
}
impl From<Fwmde> for u8 {
    #[inline(always)]
    fn from(val: Fwmde) -> u8 {
        Fwmde::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Fwmie {
    #[doc = "FIFO watermark interrupts are not enabled."]
    FWMIE_0 = 0x0,
    #[doc = "FIFO watermark interrupts are enabled."]
    FWMIE_1 = 0x01,
}
impl Fwmie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fwmie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fwmie {
    #[inline(always)]
    fn from(val: u8) -> Fwmie {
        Fwmie::from_bits(val)
    }
}
impl From<Fwmie> for u8 {
    #[inline(always)]
    fn from(val: Fwmie) -> u8 {
        Fwmie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Hten {
    #[doc = "Hardware trigger source disabled"]
    HTEN_0 = 0x0,
    #[doc = "Hardware trigger source enabled"]
    HTEN_1 = 0x01,
}
impl Hten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hten {
    #[inline(always)]
    fn from(val: u8) -> Hten {
        Hten::from_bits(val)
    }
}
impl From<Hten> for u8 {
    #[inline(always)]
    fn from(val: Hten) -> u8 {
        Hten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Iadcki {
    #[doc = "Internal clock source not implemented."]
    IADCKI_0 = 0x0,
    #[doc = "Internal clock source (and CFG\\[ADCKEN\\]) implemented."]
    IADCKI_1 = 0x01,
}
impl Iadcki {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iadcki {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iadcki {
    #[inline(always)]
    fn from(val: u8) -> Iadcki {
        Iadcki::from_bits(val)
    }
}
impl From<Iadcki> for u8 {
    #[inline(always)]
    fn from(val: Iadcki) -> u8 {
        Iadcki::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Loopcnt {
    #[doc = "Result is from initial conversion in command."]
    LOOPCNT_0 = 0x0,
    #[doc = "Result is from second conversion in command."]
    LOOPCNT_1 = 0x01,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_2 = 0x02,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_3 = 0x03,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_4 = 0x04,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_5 = 0x05,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_6 = 0x06,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_7 = 0x07,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_8 = 0x08,
    #[doc = "Result is from LOOPCNT+1 conversion in command."]
    LOOPCNT_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Result is from 16th conversion in command."]
    LOOPCNT_15 = 0x0f,
}
impl Loopcnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopcnt {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopcnt {
    #[inline(always)]
    fn from(val: u8) -> Loopcnt {
        Loopcnt::from_bits(val)
    }
}
impl From<Loopcnt> for u8 {
    #[inline(always)]
    fn from(val: Loopcnt) -> u8 {
        Loopcnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mvi {
    #[doc = "Single voltage reference high (VREFH) input supported."]
    MVI_0 = 0x0,
    #[doc = "Multiple voltage reference high (VREFH) inputs supported."]
    MVI_1 = 0x01,
}
impl Mvi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mvi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mvi {
    #[inline(always)]
    fn from(val: u8) -> Mvi {
        Mvi::from_bits(val)
    }
}
impl From<Mvi> for u8 {
    #[inline(always)]
    fn from(val: Mvi) -> u8 {
        Mvi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pauseen {
    #[doc = "Pause operation disabled"]
    PAUSEEN_0 = 0x0,
    #[doc = "Pause operation enabled"]
    PAUSEEN_1 = 0x01,
}
impl Pauseen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pauseen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pauseen {
    #[inline(always)]
    fn from(val: u8) -> Pauseen {
        Pauseen::from_bits(val)
    }
}
impl From<Pauseen> for u8 {
    #[inline(always)]
    fn from(val: Pauseen) -> u8 {
        Pauseen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwren {
    #[doc = "ADC analog circuits are only enabled while conversions are active. Performance is affected due to analog startup delays."]
    PWREN_0 = 0x0,
    #[doc = "ADC analog circuits are pre-enabled and ready to execute conversions without startup delays (at the cost of higher DC current consumption). When PWREN is set, the power up delay is enforced such that any detected trigger does not begin ADC operation until the power up delay time has passed."]
    PWREN_1 = 0x01,
}
impl Pwren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwren {
    #[inline(always)]
    fn from(val: u8) -> Pwren {
        Pwren::from_bits(val)
    }
}
impl From<Pwren> for u8 {
    #[inline(always)]
    fn from(val: Pwren) -> u8 {
        Pwren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pwrsel {
    #[doc = "Lowest power setting."]
    PWRSEL_0 = 0x0,
    #[doc = "Next lowest power setting."]
    PWRSEL_1 = 0x01,
    #[doc = "...."]
    PWRSEL_2 = 0x02,
    #[doc = "Highest power setting."]
    PWRSEL_3 = 0x03,
}
impl Pwrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwrsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwrsel {
    #[inline(always)]
    fn from(val: u8) -> Pwrsel {
        Pwrsel::from_bits(val)
    }
}
impl From<Pwrsel> for u8 {
    #[inline(always)]
    fn from(val: Pwrsel) -> u8 {
        Pwrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rdy {
    #[doc = "Result FIFO data level not above watermark level."]
    RDY_0 = 0x0,
    #[doc = "Result FIFO holding data above watermark level."]
    RDY_1 = 0x01,
}
impl Rdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdy {
    #[inline(always)]
    fn from(val: u8) -> Rdy {
        Rdy::from_bits(val)
    }
}
impl From<Rdy> for u8 {
    #[inline(always)]
    fn from(val: Rdy) -> u8 {
        Rdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Refsel {
    #[doc = "(Default) Option 1 setting."]
    REFSEL_0 = 0x0,
    #[doc = "Option 2 setting."]
    REFSEL_1 = 0x01,
    #[doc = "Option 3 setting."]
    REFSEL_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Res {
    #[doc = "Up to 13-bit differential/12-bit single ended resolution supported."]
    RES_0 = 0x0,
    #[doc = "Up to 16-bit differential/15-bit single ended resolution supported."]
    RES_1 = 0x01,
}
impl Res {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Res {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Res {
    #[inline(always)]
    fn from(val: u8) -> Res {
        Res::from_bits(val)
    }
}
impl From<Res> for u8 {
    #[inline(always)]
    fn from(val: Res) -> u8 {
        Res::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rst {
    #[doc = "ADC logic is not reset."]
    RST_0 = 0x0,
    #[doc = "ADC logic is reset."]
    RST_1 = 0x01,
}
impl Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rst {
    #[inline(always)]
    fn from(val: u8) -> Rst {
        Rst::from_bits(val)
    }
}
impl From<Rst> for u8 {
    #[inline(always)]
    fn from(val: Rst) -> u8 {
        Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Rstfifo {
    #[doc = "No effect."]
    RSTFIFO_0 = 0x0,
    #[doc = "FIFO is reset."]
    RSTFIFO_1 = 0x01,
}
impl Rstfifo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstfifo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstfifo {
    #[inline(always)]
    fn from(val: u8) -> Rstfifo {
        Rstfifo::from_bits(val)
    }
}
impl From<Rstfifo> for u8 {
    #[inline(always)]
    fn from(val: Rstfifo) -> u8 {
        Rstfifo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt0 {
    #[doc = "No trigger 0 event generated."]
    SWT0_0 = 0x0,
    #[doc = "Trigger 0 event generated."]
    SWT0_1 = 0x01,
}
impl Swt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt0 {
    #[inline(always)]
    fn from(val: u8) -> Swt0 {
        Swt0::from_bits(val)
    }
}
impl From<Swt0> for u8 {
    #[inline(always)]
    fn from(val: Swt0) -> u8 {
        Swt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt1 {
    #[doc = "No trigger 1 event generated."]
    SWT1_0 = 0x0,
    #[doc = "Trigger 1 event generated."]
    SWT1_1 = 0x01,
}
impl Swt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt1 {
    #[inline(always)]
    fn from(val: u8) -> Swt1 {
        Swt1::from_bits(val)
    }
}
impl From<Swt1> for u8 {
    #[inline(always)]
    fn from(val: Swt1) -> u8 {
        Swt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt10 {
    #[doc = "No trigger 10 event generated."]
    SWT10_0 = 0x0,
    #[doc = "Trigger 10 event generated."]
    SWT10_1 = 0x01,
}
impl Swt10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt10 {
    #[inline(always)]
    fn from(val: u8) -> Swt10 {
        Swt10::from_bits(val)
    }
}
impl From<Swt10> for u8 {
    #[inline(always)]
    fn from(val: Swt10) -> u8 {
        Swt10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt11 {
    #[doc = "No trigger 11 event generated."]
    SWT11_0 = 0x0,
    #[doc = "Trigger 11 event generated."]
    SWT11_1 = 0x01,
}
impl Swt11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt11 {
    #[inline(always)]
    fn from(val: u8) -> Swt11 {
        Swt11::from_bits(val)
    }
}
impl From<Swt11> for u8 {
    #[inline(always)]
    fn from(val: Swt11) -> u8 {
        Swt11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt12 {
    #[doc = "No trigger 12 event generated."]
    SWT12_0 = 0x0,
    #[doc = "Trigger 12 event generated."]
    SWT12_1 = 0x01,
}
impl Swt12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt12 {
    #[inline(always)]
    fn from(val: u8) -> Swt12 {
        Swt12::from_bits(val)
    }
}
impl From<Swt12> for u8 {
    #[inline(always)]
    fn from(val: Swt12) -> u8 {
        Swt12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt13 {
    #[doc = "No trigger 13 event generated."]
    SWT13_0 = 0x0,
    #[doc = "Trigger 13 event generated."]
    SWT13_1 = 0x01,
}
impl Swt13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt13 {
    #[inline(always)]
    fn from(val: u8) -> Swt13 {
        Swt13::from_bits(val)
    }
}
impl From<Swt13> for u8 {
    #[inline(always)]
    fn from(val: Swt13) -> u8 {
        Swt13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt14 {
    #[doc = "No trigger 14 event generated."]
    SWT14_0 = 0x0,
    #[doc = "Trigger 14 event generated."]
    SWT14_1 = 0x01,
}
impl Swt14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt14 {
    #[inline(always)]
    fn from(val: u8) -> Swt14 {
        Swt14::from_bits(val)
    }
}
impl From<Swt14> for u8 {
    #[inline(always)]
    fn from(val: Swt14) -> u8 {
        Swt14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt15 {
    #[doc = "No trigger 15 event generated."]
    SWT15_0 = 0x0,
    #[doc = "Trigger 15 event generated."]
    SWT15_1 = 0x01,
}
impl Swt15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt15 {
    #[inline(always)]
    fn from(val: u8) -> Swt15 {
        Swt15::from_bits(val)
    }
}
impl From<Swt15> for u8 {
    #[inline(always)]
    fn from(val: Swt15) -> u8 {
        Swt15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt2 {
    #[doc = "No trigger 2 event generated."]
    SWT2_0 = 0x0,
    #[doc = "Trigger 2 event generated."]
    SWT2_1 = 0x01,
}
impl Swt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt2 {
    #[inline(always)]
    fn from(val: u8) -> Swt2 {
        Swt2::from_bits(val)
    }
}
impl From<Swt2> for u8 {
    #[inline(always)]
    fn from(val: Swt2) -> u8 {
        Swt2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt3 {
    #[doc = "No trigger 3 event generated."]
    SWT3_0 = 0x0,
    #[doc = "Trigger 3 event generated."]
    SWT3_1 = 0x01,
}
impl Swt3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt3 {
    #[inline(always)]
    fn from(val: u8) -> Swt3 {
        Swt3::from_bits(val)
    }
}
impl From<Swt3> for u8 {
    #[inline(always)]
    fn from(val: Swt3) -> u8 {
        Swt3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt4 {
    #[doc = "No trigger 4 event generated."]
    SWT4_0 = 0x0,
    #[doc = "Trigger 4 event generated."]
    SWT4_1 = 0x01,
}
impl Swt4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt4 {
    #[inline(always)]
    fn from(val: u8) -> Swt4 {
        Swt4::from_bits(val)
    }
}
impl From<Swt4> for u8 {
    #[inline(always)]
    fn from(val: Swt4) -> u8 {
        Swt4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt5 {
    #[doc = "No trigger 5 event generated."]
    SWT5_0 = 0x0,
    #[doc = "Trigger 5 event generated."]
    SWT5_1 = 0x01,
}
impl Swt5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt5 {
    #[inline(always)]
    fn from(val: u8) -> Swt5 {
        Swt5::from_bits(val)
    }
}
impl From<Swt5> for u8 {
    #[inline(always)]
    fn from(val: Swt5) -> u8 {
        Swt5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt6 {
    #[doc = "No trigger 6 event generated."]
    SWT6_0 = 0x0,
    #[doc = "Trigger 6 event generated."]
    SWT6_1 = 0x01,
}
impl Swt6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt6 {
    #[inline(always)]
    fn from(val: u8) -> Swt6 {
        Swt6::from_bits(val)
    }
}
impl From<Swt6> for u8 {
    #[inline(always)]
    fn from(val: Swt6) -> u8 {
        Swt6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt7 {
    #[doc = "No trigger 7 event generated."]
    SWT7_0 = 0x0,
    #[doc = "Trigger 7 event generated."]
    SWT7_1 = 0x01,
}
impl Swt7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt7 {
    #[inline(always)]
    fn from(val: u8) -> Swt7 {
        Swt7::from_bits(val)
    }
}
impl From<Swt7> for u8 {
    #[inline(always)]
    fn from(val: Swt7) -> u8 {
        Swt7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt8 {
    #[doc = "No trigger 8 event generated."]
    SWT8_0 = 0x0,
    #[doc = "Trigger 8 event generated."]
    SWT8_1 = 0x01,
}
impl Swt8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt8 {
    #[inline(always)]
    fn from(val: u8) -> Swt8 {
        Swt8::from_bits(val)
    }
}
impl From<Swt8> for u8 {
    #[inline(always)]
    fn from(val: Swt8) -> u8 {
        Swt8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Swt9 {
    #[doc = "No trigger 9 event generated."]
    SWT9_0 = 0x0,
    #[doc = "Trigger 9 event generated."]
    SWT9_1 = 0x01,
}
impl Swt9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swt9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swt9 {
    #[inline(always)]
    fn from(val: u8) -> Swt9 {
        Swt9::from_bits(val)
    }
}
impl From<Swt9> for u8 {
    #[inline(always)]
    fn from(val: Swt9) -> u8 {
        Swt9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tcmd {
    #[doc = "Not a valid selection from the command buffer. Trigger event is ignored."]
    TCMD_0 = 0x0,
    #[doc = "CMD1 is executed"]
    TCMD_1 = 0x01,
    #[doc = "Corresponding CMD is executed"]
    TCMD_2 = 0x02,
    #[doc = "Corresponding CMD is executed"]
    TCMD_3 = 0x03,
    #[doc = "Corresponding CMD is executed"]
    TCMD_4 = 0x04,
    #[doc = "Corresponding CMD is executed"]
    TCMD_5 = 0x05,
    #[doc = "Corresponding CMD is executed"]
    TCMD_6 = 0x06,
    #[doc = "Corresponding CMD is executed"]
    TCMD_7 = 0x07,
    #[doc = "Corresponding CMD is executed"]
    TCMD_8 = 0x08,
    #[doc = "Corresponding CMD is executed"]
    TCMD_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CMD15 is executed"]
    TCMD_15 = 0x0f,
}
impl Tcmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcmd {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcmd {
    #[inline(always)]
    fn from(val: u8) -> Tcmd {
        Tcmd::from_bits(val)
    }
}
impl From<Tcmd> for u8 {
    #[inline(always)]
    fn from(val: Tcmd) -> u8 {
        Tcmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tpri {
    #[doc = "Set to highest priority, Level 1"]
    TPRI_0 = 0x0,
    #[doc = "Set to corresponding priority level"]
    TPRI_1 = 0x01,
    #[doc = "Set to corresponding priority level"]
    TPRI_2 = 0x02,
    #[doc = "Set to corresponding priority level"]
    TPRI_3 = 0x03,
    #[doc = "Set to corresponding priority level"]
    TPRI_4 = 0x04,
    #[doc = "Set to corresponding priority level"]
    TPRI_5 = 0x05,
    #[doc = "Set to corresponding priority level"]
    TPRI_6 = 0x06,
    #[doc = "Set to corresponding priority level"]
    TPRI_7 = 0x07,
    #[doc = "Set to corresponding priority level"]
    TPRI_8 = 0x08,
    #[doc = "Set to corresponding priority level"]
    TPRI_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Set to lowest priority, Level 16"]
    TPRI_15 = 0x0f,
}
impl Tpri {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpri {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpri {
    #[inline(always)]
    fn from(val: u8) -> Tpri {
        Tpri::from_bits(val)
    }
}
impl From<Tpri> for u8 {
    #[inline(always)]
    fn from(val: Tpri) -> u8 {
        Tpri::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tprictrl {
    #[doc = "If a higher priority trigger is detected during command processing, the current conversion is aborted and the new command specified by the trigger is started."]
    TPRICTRL_0 = 0x0,
    #[doc = "If a higher priority trigger is received during command processing, the current conversion is completed (including averaging iterations and compare function if enabled) and stored to the RESFIFO before the higher priority trigger/command is initiated."]
    TPRICTRL_1 = 0x01,
}
impl Tprictrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tprictrl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tprictrl {
    #[inline(always)]
    fn from(val: u8) -> Tprictrl {
        Tprictrl::from_bits(val)
    }
}
impl From<Tprictrl> for u8 {
    #[inline(always)]
    fn from(val: Tprictrl) -> u8 {
        Tprictrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Trgact {
    #[doc = "Command (sequence) associated with Trigger 0 currently being executed."]
    TRGACT_0 = 0x0,
    #[doc = "Command (sequence) associated with Trigger 1 currently being executed."]
    TRGACT_1 = 0x01,
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
    _RESERVED_e = 0x0e,
    #[doc = "Command (sequence) associated with Trigger 15 currently being executed."]
    TRGACT_15 = 0x0f,
}
impl Trgact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgact {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgact {
    #[inline(always)]
    fn from(val: u8) -> Trgact {
        Trgact::from_bits(val)
    }
}
impl From<Trgact> for u8 {
    #[inline(always)]
    fn from(val: Trgact) -> u8 {
        Trgact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Tsrc {
    #[doc = "Trigger source 0 initiated this conversion."]
    TSRC_0 = 0x0,
    #[doc = "Trigger source 1 initiated this conversion."]
    TSRC_1 = 0x01,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_2 = 0x02,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_3 = 0x03,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_4 = 0x04,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_5 = 0x05,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_6 = 0x06,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_7 = 0x07,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_8 = 0x08,
    #[doc = "Corresponding trigger source initiated this conversion."]
    TSRC_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Trigger source 15 initiated this conversion."]
    TSRC_15 = 0x0f,
}
impl Tsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsrc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsrc {
    #[inline(always)]
    fn from(val: u8) -> Tsrc {
        Tsrc::from_bits(val)
    }
}
impl From<Tsrc> for u8 {
    #[inline(always)]
    fn from(val: Tsrc) -> u8 {
        Tsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Valid {
    #[doc = "FIFO is empty. Discard any read from RESFIFO."]
    VALID_0 = 0x0,
    #[doc = "FIFO record read from RESFIFO is valid."]
    VALID_1 = 0x01,
}
impl Valid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Valid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Valid {
    #[inline(always)]
    fn from(val: u8) -> Valid {
        Valid::from_bits(val)
    }
}
impl From<Valid> for u8 {
    #[inline(always)]
    fn from(val: Valid) -> u8 {
        Valid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Vr1rngi {
    #[doc = "Range control not required. CFG\\[VREF1RNG\\] is not implemented."]
    VR1RNGI_0 = 0x0,
    #[doc = "Range control required. CFG\\[VREF1RNG\\] is implemented."]
    VR1RNGI_1 = 0x01,
}
impl Vr1rngi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vr1rngi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vr1rngi {
    #[inline(always)]
    fn from(val: u8) -> Vr1rngi {
        Vr1rngi::from_bits(val)
    }
}
impl From<Vr1rngi> for u8 {
    #[inline(always)]
    fn from(val: Vr1rngi) -> u8 {
        Vr1rngi::to_bits(val)
    }
}
