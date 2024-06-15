#[doc = "Register `CTX_RGD_W0` reader"]
pub type R = crate::R<CtxRgdW0Spec>;
#[doc = "Register `CTX_RGD_W0` writer"]
pub type W = crate::W<CtxRgdW0Spec>;
#[doc = "Field `SRTADDR` reader - Start Address"]
pub type SrtaddrR = crate::FieldReader<u32>;
#[doc = "Field `SRTADDR` writer - Start Address"]
pub type SrtaddrW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 10:31 - Start Address"]
    #[inline(always)]
    pub fn srtaddr(&self) -> SrtaddrR {
        SrtaddrR::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 10:31 - Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn srtaddr(&mut self) -> SrtaddrW<CtxRgdW0Spec> {
        SrtaddrW::new(self, 10)
    }
}
#[doc = "AES Region Descriptor Word0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctx_rgd_w0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctx_rgd_w0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtxRgdW0Spec;
impl crate::RegisterSpec for CtxRgdW0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctx_rgd_w0::R`](R) reader structure"]
impl crate::Readable for CtxRgdW0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctx_rgd_w0::W`](W) writer structure"]
impl crate::Writable for CtxRgdW0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTX_RGD_W0 to value 0"]
impl crate::Resettable for CtxRgdW0Spec {
    const RESET_VALUE: u32 = 0;
}
