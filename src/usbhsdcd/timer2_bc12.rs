#[doc = "Register `TIMER2_BC12` reader"]
pub type R = crate::R<Timer2Bc12Spec>;
#[doc = "Register `TIMER2_BC12` writer"]
pub type W = crate::W<Timer2Bc12Spec>;
#[doc = "Field `TVDMSRC_ON` reader - TVDMSRC_ON"]
pub type TvdmsrcOnR = crate::FieldReader<u16>;
#[doc = "Field `TVDMSRC_ON` writer - TVDMSRC_ON"]
pub type TvdmsrcOnW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TWAIT_AFTER_PRD` reader - TWAIT_AFTER_PRD"]
pub type TwaitAfterPrdR = crate::FieldReader<u16>;
#[doc = "Field `TWAIT_AFTER_PRD` writer - TWAIT_AFTER_PRD"]
pub type TwaitAfterPrdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - TVDMSRC_ON"]
    #[inline(always)]
    pub fn tvdmsrc_on(&self) -> TvdmsrcOnR {
        TvdmsrcOnR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - TWAIT_AFTER_PRD"]
    #[inline(always)]
    pub fn twait_after_prd(&self) -> TwaitAfterPrdR {
        TwaitAfterPrdR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TVDMSRC_ON"]
    #[inline(always)]
    #[must_use]
    pub fn tvdmsrc_on(&mut self) -> TvdmsrcOnW<Timer2Bc12Spec> {
        TvdmsrcOnW::new(self, 0)
    }
    #[doc = "Bits 16:25 - TWAIT_AFTER_PRD"]
    #[inline(always)]
    #[must_use]
    pub fn twait_after_prd(&mut self) -> TwaitAfterPrdW<Timer2Bc12Spec> {
        TwaitAfterPrdW::new(self, 16)
    }
}
#[doc = "TIMER2_BC12 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_bc12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_bc12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2Bc12Spec;
impl crate::RegisterSpec for Timer2Bc12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2_bc12::R`](R) reader structure"]
impl crate::Readable for Timer2Bc12Spec {}
#[doc = "`write(|w| ..)` method takes [`timer2_bc12::W`](W) writer structure"]
impl crate::Writable for Timer2Bc12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2_BC12 to value 0x0001_0028"]
impl crate::Resettable for Timer2Bc12Spec {
    const RESET_VALUE: u32 = 0x0001_0028;
}
