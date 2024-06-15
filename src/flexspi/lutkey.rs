#[doc = "Register `LUTKEY` reader"]
pub type R = crate::R<LutkeySpec>;
#[doc = "Register `LUTKEY` writer"]
pub type W = crate::W<LutkeySpec>;
#[doc = "Field `KEY` reader - The Key to lock or unlock LUT."]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - The Key to lock or unlock LUT."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The Key to lock or unlock LUT."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The Key to lock or unlock LUT."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<LutkeySpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "LUT Key Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lutkey::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lutkey::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutkeySpec;
impl crate::RegisterSpec for LutkeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lutkey::R`](R) reader structure"]
impl crate::Readable for LutkeySpec {}
#[doc = "`write(|w| ..)` method takes [`lutkey::W`](W) writer structure"]
impl crate::Writable for LutkeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LUTKEY to value 0x5af0_5af0"]
impl crate::Resettable for LutkeySpec {
    const RESET_VALUE: u32 = 0x5af0_5af0;
}
