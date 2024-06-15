#[doc = "Register `MWDATAB` writer"]
pub type W = crate::W<MwdatabSpec>;
#[doc = "Field `DATA` writer - Data byte"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `END` writer - End of message"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `END_ALSO` writer - End of message also"]
pub type EndAlsoW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - Data byte"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<MwdatabSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bit 8 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<MwdatabSpec> {
        EndW::new(self, 8)
    }
    #[doc = "Bit 16 - End of message also"]
    #[inline(always)]
    #[must_use]
    pub fn end_also(&mut self) -> EndAlsoW<MwdatabSpec> {
        EndAlsoW::new(self, 16)
    }
}
#[doc = "Master Write Data Byte Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mwdatab::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwdatabSpec;
impl crate::RegisterSpec for MwdatabSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwdatab::W`](W) writer structure"]
impl crate::Writable for MwdatabSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWDATAB to value 0"]
impl crate::Resettable for MwdatabSpec {
    const RESET_VALUE: u32 = 0;
}
