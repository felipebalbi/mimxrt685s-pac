#[doc = "Register `FRQCNT` reader"]
pub type R = crate::R<MaxCntFrqcntSpec>;
#[doc = "Field `FRQ_CT` reader - Frequency Count"]
pub type FrqCtR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency Count"]
    #[inline(always)]
    pub fn frq_ct(&self) -> FrqCtR {
        FrqCtR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Frequency Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`max_cnt_frqcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxCntFrqcntSpec;
impl crate::RegisterSpec for MaxCntFrqcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_cnt_frqcnt::R`](R) reader structure"]
impl crate::Readable for MaxCntFrqcntSpec {}
#[doc = "`reset()` method sets FRQCNT to value 0"]
impl crate::Resettable for MaxCntFrqcntSpec {
    const RESET_VALUE: u32 = 0;
}
