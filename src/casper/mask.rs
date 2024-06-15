#[doc = "Register `MASK` reader"]
pub type R = crate::R<MaskSpec>;
#[doc = "Register `MASK` writer"]
pub type W = crate::W<MaskSpec>;
#[doc = "Field `MASK` reader - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
pub type MaskR = crate::FieldReader<u32>;
#[doc = "Field `MASK` writer - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask to apply as side channel countermeasure. 0: No mask to be used. N: Mask to XOR onto values"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<MaskSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Optional mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskSpec;
impl crate::RegisterSpec for MaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask::R`](R) reader structure"]
impl crate::Readable for MaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mask::W`](W) writer structure"]
impl crate::Writable for MaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MaskSpec {
    const RESET_VALUE: u32 = 0;
}
