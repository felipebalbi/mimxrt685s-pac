#[doc = "Register `LUTCR` reader"]
pub type R = crate::R<LutcrSpec>;
#[doc = "Register `LUTCR` writer"]
pub type W = crate::W<LutcrSpec>;
#[doc = "Field `LOCK` reader - Lock LUT"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock LUT"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNLOCK` reader - Unlock LUT"]
pub type UnlockR = crate::BitReader;
#[doc = "Field `UNLOCK` writer - Unlock LUT"]
pub type UnlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock LUT"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Unlock LUT"]
    #[inline(always)]
    pub fn unlock(&self) -> UnlockR {
        UnlockR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock LUT"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<LutcrSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bit 1 - Unlock LUT"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UnlockW<LutcrSpec> {
        UnlockW::new(self, 1)
    }
}
#[doc = "LUT Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lutcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lutcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutcrSpec;
impl crate::RegisterSpec for LutcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lutcr::R`](R) reader structure"]
impl crate::Readable for LutcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lutcr::W`](W) writer structure"]
impl crate::Writable for LutcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LUTCR to value 0x02"]
impl crate::Resettable for LutcrSpec {
    const RESET_VALUE: u32 = 0x02;
}