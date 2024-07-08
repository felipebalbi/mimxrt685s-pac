#[doc = "Register `PKRSQ` reader"]
pub type R = crate::R<MaxSqPkrsqSpec>;
#[doc = "Field `PKR_SQ` reader - Poker Square Calculation Result."]
pub type PkrSqR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Poker Square Calculation Result."]
    #[inline(always)]
    pub fn pkr_sq(&self) -> PkrSqR {
        PkrSqR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Poker Square Calculation Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`max_sq_pkrsq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxSqPkrsqSpec;
impl crate::RegisterSpec for MaxSqPkrsqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_sq_pkrsq::R`](R) reader structure"]
impl crate::Readable for MaxSqPkrsqSpec {}
#[doc = "`reset()` method sets PKRSQ to value 0"]
impl crate::Resettable for MaxSqPkrsqSpec {
    const RESET_VALUE: u32 = 0;
}
