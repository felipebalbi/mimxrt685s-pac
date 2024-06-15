#[doc = "Register `INTENB[%s]` reader"]
pub type R = crate::R<IntenbSpec>;
#[doc = "Register `INTENB[%s]` writer"]
pub type W = crate::W<IntenbSpec>;
#[doc = "Field `INT_EN` reader - interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
pub type IntEnR = crate::FieldReader<u32>;
#[doc = "Field `INT_EN` writer - interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
pub type IntEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
    #[inline(always)]
    pub fn int_en(&self) -> IntEnR {
        IntEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
    #[inline(always)]
    #[must_use]
    pub fn int_en(&mut self) -> IntEnW<IntenbSpec> {
        IntEnW::new(self, 0)
    }
}
#[doc = "interrupt B enable control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenbSpec;
impl crate::RegisterSpec for IntenbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenb::R`](R) reader structure"]
impl crate::Readable for IntenbSpec {}
#[doc = "`write(|w| ..)` method takes [`intenb::W`](W) writer structure"]
impl crate::Writable for IntenbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENB[%s]
to value 0"]
impl crate::Resettable for IntenbSpec {
    const RESET_VALUE: u32 = 0;
}
