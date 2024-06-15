#[doc = "Register `MEMADDR` reader"]
pub type R = crate::R<MemaddrSpec>;
#[doc = "Register `MEMADDR` writer"]
pub type W = crate::W<MemaddrSpec>;
#[doc = "Field `BASE` reader - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
pub type BaseR = crate::FieldReader<u32>;
#[doc = "Field `BASE` writer - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address base to start copying from, word aligned (so bits 1:0 must be 0). This field will advance as it processes the words. If it fails with a bus error, the register will contain the failing word. N:Address in Flash or RAM space; RAM only as mapped in this part. May also be able to address SPIFI."]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<MemaddrSpec> {
        BaseW::new(self, 0)
    }
}
#[doc = "Address to start memory access from (if available).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemaddrSpec;
impl crate::RegisterSpec for MemaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memaddr::R`](R) reader structure"]
impl crate::Readable for MemaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`memaddr::W`](W) writer structure"]
impl crate::Writable for MemaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMADDR to value 0"]
impl crate::Resettable for MemaddrSpec {
    const RESET_VALUE: u32 = 0;
}
