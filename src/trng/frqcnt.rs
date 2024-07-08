#[doc = "Register `FRQCNT` reader"]
pub type R = crate::R<FrqcntSpec>;
#[doc = "Field `FRQ_CT` reader - Frequency Count"]
pub type FrqCtR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency Count"]
    #[inline(always)]
    pub fn frq_ct(&self) -> FrqCtR {
        FrqCtR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Frequency Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frqcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrqcntSpec;
impl crate::RegisterSpec for FrqcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frqcnt::R`](R) reader structure"]
impl crate::Readable for FrqcntSpec {}
#[doc = "`reset()` method sets FRQCNT to value 0"]
impl crate::Resettable for FrqcntSpec {
    const RESET_VALUE: u32 = 0;
}
