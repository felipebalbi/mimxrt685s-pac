#[doc = "Register `MSR[%s]` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Register `MSR[%s]` writer"]
pub type W = crate::W<MsrSpec>;
#[doc = "Field `MATCH_Shadow` reader - Timer counter match value."]
pub type MatchShadowR = crate::FieldReader<u32>;
#[doc = "Field `MATCH_Shadow` writer - Timer counter match value."]
pub type MatchShadowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer counter match value."]
    #[inline(always)]
    pub fn match_shadow(&self) -> MatchShadowR {
        MatchShadowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer counter match value."]
    #[inline(always)]
    #[must_use]
    pub fn match_shadow(&mut self) -> MatchShadowW<MsrSpec> {
        MatchShadowW::new(self, 0)
    }
}
#[doc = "Match Shadow Register . If enabled, the Match Register will be automatically reloaded with the contents of this register whenever the TC is reset to zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`write(|w| ..)` method takes [`msr::W`](W) writer structure"]
impl crate::Writable for MsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSR[%s]
to value 0"]
impl crate::Resettable for MsrSpec {
    const RESET_VALUE: u32 = 0;
}