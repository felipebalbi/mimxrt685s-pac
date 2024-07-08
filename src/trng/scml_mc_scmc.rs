#[doc = "Register `SCMC` reader"]
pub type R = crate::R<ScmlMcScmcSpec>;
#[doc = "Field `MONO_CT` reader - Monobit Count"]
pub type MonoCtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Monobit Count"]
    #[inline(always)]
    pub fn mono_ct(&self) -> MonoCtR {
        MonoCtR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Statistical Check Monobit Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scml_mc_scmc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmlMcScmcSpec;
impl crate::RegisterSpec for ScmlMcScmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scml_mc_scmc::R`](R) reader structure"]
impl crate::Readable for ScmlMcScmcSpec {}
#[doc = "`reset()` method sets SCMC to value 0"]
impl crate::Resettable for ScmlMcScmcSpec {
    const RESET_VALUE: u32 = 0;
}
