#[doc = "Register `DIVHFCLK` reader"]
pub type R = crate::R<DivhfclkSpec>;
#[doc = "Register `DIVHFCLK` writer"]
pub type W = crate::W<DivhfclkSpec>;
#[doc = "Field `PDMDIV` reader - Divide by factor to create PDM Clock (enumerated type)"]
pub type PdmdivR = crate::FieldReader;
#[doc = "Field `PDMDIV` writer - Divide by factor to create PDM Clock (enumerated type)"]
pub type PdmdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Divide by factor to create PDM Clock (enumerated type)"]
    #[inline(always)]
    pub fn pdmdiv(&self) -> PdmdivR {
        PdmdivR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Divide by factor to create PDM Clock (enumerated type)"]
    #[inline(always)]
    #[must_use]
    pub fn pdmdiv(&mut self) -> PdmdivW<DivhfclkSpec> {
        PdmdivW::new(self, 0)
    }
}
#[doc = "Divider for generating PDM clock from DMIC clock input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divhfclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divhfclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivhfclkSpec;
impl crate::RegisterSpec for DivhfclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divhfclk::R`](R) reader structure"]
impl crate::Readable for DivhfclkSpec {}
#[doc = "`write(|w| ..)` method takes [`divhfclk::W`](W) writer structure"]
impl crate::Writable for DivhfclkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVHFCLK to value 0"]
impl crate::Resettable for DivhfclkSpec {
    const RESET_VALUE: u32 = 0;
}
