#[doc = "Register `CCVR` reader"]
pub type R = crate::R<CcvrSpec>;
#[doc = "Register `CCVR` writer"]
pub type W = crate::W<CcvrSpec>;
#[doc = "Field `DATA` reader - Cache read/write Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Cache read/write Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cache read/write Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Cache read/write Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<CcvrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Cache read/write value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcvrSpec;
impl crate::RegisterSpec for CcvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccvr::R`](R) reader structure"]
impl crate::Readable for CcvrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccvr::W`](W) writer structure"]
impl crate::Writable for CcvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCVR to value 0"]
impl crate::Resettable for CcvrSpec {
    const RESET_VALUE: u32 = 0;
}