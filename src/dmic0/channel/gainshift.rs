#[doc = "Register `GAINSHIFT` reader"]
pub type R = crate::R<GainshiftSpec>;
#[doc = "Register `GAINSHIFT` writer"]
pub type W = crate::W<GainshiftSpec>;
#[doc = "Field `GAIN` reader - Gain shift for decimator output (can be positive or negative number)"]
pub type GainR = crate::FieldReader;
#[doc = "Field `GAIN` writer - Gain shift for decimator output (can be positive or negative number)"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Gain shift for decimator output (can be positive or negative number)"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Gain shift for decimator output (can be positive or negative number)"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GainW<GainshiftSpec> {
        GainW::new(self, 0)
    }
}
#[doc = "Decimator output gain adjustment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gainshift::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gainshift::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GainshiftSpec;
impl crate::RegisterSpec for GainshiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gainshift::R`](R) reader structure"]
impl crate::Readable for GainshiftSpec {}
#[doc = "`write(|w| ..)` method takes [`gainshift::W`](W) writer structure"]
impl crate::Writable for GainshiftSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAINSHIFT to value 0"]
impl crate::Resettable for GainshiftSpec {
    const RESET_VALUE: u32 = 0;
}