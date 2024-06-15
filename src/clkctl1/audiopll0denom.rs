#[doc = "Register `AUDIOPLL0DENOM` reader"]
pub type R = crate::R<Audiopll0denomSpec>;
#[doc = "Register `AUDIOPLL0DENOM` writer"]
pub type W = crate::W<Audiopll0denomSpec>;
#[doc = "Field `DENOM` reader - This field contains the denominator of the AUDIOPLL0 fractional loop divider. NOTES: 1. The value of numerator must always be configured to be less than the value of the denominator. 2. The AUDIOPLL0DENOM register can only be changed when the AUDIOPLL0 is disabled."]
pub type DenomR = crate::FieldReader<u32>;
#[doc = "Field `DENOM` writer - This field contains the denominator of the AUDIOPLL0 fractional loop divider. NOTES: 1. The value of numerator must always be configured to be less than the value of the denominator. 2. The AUDIOPLL0DENOM register can only be changed when the AUDIOPLL0 is disabled."]
pub type DenomW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - This field contains the denominator of the AUDIOPLL0 fractional loop divider. NOTES: 1. The value of numerator must always be configured to be less than the value of the denominator. 2. The AUDIOPLL0DENOM register can only be changed when the AUDIOPLL0 is disabled."]
    #[inline(always)]
    pub fn denom(&self) -> DenomR {
        DenomR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - This field contains the denominator of the AUDIOPLL0 fractional loop divider. NOTES: 1. The value of numerator must always be configured to be less than the value of the denominator. 2. The AUDIOPLL0DENOM register can only be changed when the AUDIOPLL0 is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn denom(&mut self) -> DenomW<Audiopll0denomSpec> {
        DenomW::new(self, 0)
    }
}
#[doc = "Audio pll0 denom\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audiopll0denom::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audiopll0denom::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Audiopll0denomSpec;
impl crate::RegisterSpec for Audiopll0denomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audiopll0denom::R`](R) reader structure"]
impl crate::Readable for Audiopll0denomSpec {}
#[doc = "`write(|w| ..)` method takes [`audiopll0denom::W`](W) writer structure"]
impl crate::Writable for Audiopll0denomSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDIOPLL0DENOM to value 0x1fff_ffdb"]
impl crate::Resettable for Audiopll0denomSpec {
    const RESET_VALUE: u32 = 0x1fff_ffdb;
}
