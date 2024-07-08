#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_freqme_freqmectrl: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Freqeuncy Measurement (in Write mode)"]
    #[inline(always)]
    pub const fn freqme_freqmectrl_w(&self) -> &FreqmeFreqmectrlW {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Frequency Measurement (in Read mode)"]
    #[inline(always)]
    pub const fn freqme_freqmectrl_r(&self) -> &FreqmeFreqmectrlR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
}
#[doc = "FREQME_FREQMECTRL_R (r) register accessor: Frequency Measurement (in Read mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`freqme_freqmectrl_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqme_freqmectrl_r`]
module"]
#[doc(alias = "FREQME_FREQMECTRL_R")]
pub type FreqmeFreqmectrlR = crate::Reg<freqme_freqmectrl_r::FreqmeFreqmectrlRSpec>;
#[doc = "Frequency Measurement (in Read mode)"]
pub mod freqme_freqmectrl_r;
#[doc = "FREQME_FREQMECTRL_W (w) register accessor: Freqeuncy Measurement (in Write mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freqme_freqmectrl_w::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freqme_freqmectrl_w`]
module"]
#[doc(alias = "FREQME_FREQMECTRL_W")]
pub type FreqmeFreqmectrlW = crate::Reg<freqme_freqmectrl_w::FreqmeFreqmectrlWSpec>;
#[doc = "Freqeuncy Measurement (in Write mode)"]
pub mod freqme_freqmectrl_w;
