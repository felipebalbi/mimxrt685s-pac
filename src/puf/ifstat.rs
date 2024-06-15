#[doc = "Register `IFSTAT` reader"]
pub type R = crate::R<IfstatSpec>;
#[doc = "Register `IFSTAT` writer"]
pub type W = crate::W<IfstatSpec>;
#[doc = "Field `ERROR` reader - Error"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error"]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IfstatSpec> {
        ErrorW::new(self, 0)
    }
}
#[doc = "PUF Interface Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfstatSpec;
impl crate::RegisterSpec for IfstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifstat::R`](R) reader structure"]
impl crate::Readable for IfstatSpec {}
#[doc = "`write(|w| ..)` method takes [`ifstat::W`](W) writer structure"]
impl crate::Writable for IfstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets IFSTAT to value 0"]
impl crate::Resettable for IfstatSpec {
    const RESET_VALUE: u32 = 0;
}
