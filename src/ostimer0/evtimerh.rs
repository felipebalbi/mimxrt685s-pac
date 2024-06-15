#[doc = "Register `EVTIMERH` reader"]
pub type R = crate::R<EvtimerhSpec>;
#[doc = "Field `EVTIMER_COUNT_VALUE` reader - A read reflects the current value of the upper 32 bits of the EVTIMER. Note there is physically only one EVTimer, readable from all domains."]
pub type EvtimerCountValueR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - A read reflects the current value of the upper 32 bits of the EVTIMER. Note there is physically only one EVTimer, readable from all domains."]
    #[inline(always)]
    pub fn evtimer_count_value(&self) -> EvtimerCountValueR {
        EvtimerCountValueR::new(self.bits)
    }
}
#[doc = "EVTIMER High Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtimerh::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtimerhSpec;
impl crate::RegisterSpec for EvtimerhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtimerh::R`](R) reader structure"]
impl crate::Readable for EvtimerhSpec {}
#[doc = "`reset()` method sets EVTIMERH to value 0"]
impl crate::Resettable for EvtimerhSpec {
    const RESET_VALUE: u32 = 0;
}
