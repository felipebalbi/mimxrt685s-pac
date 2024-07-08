#[doc = "Register `PKRMAX` reader"]
pub type R = crate::R<MaxSqPkrmaxSpec>;
#[doc = "Register `PKRMAX` writer"]
pub type W = crate::W<MaxSqPkrmaxSpec>;
#[doc = "Field `PKR_MAX` reader - Poker Maximum Limit."]
pub type PkrMaxR = crate::FieldReader<u32>;
#[doc = "Field `PKR_MAX` writer - Poker Maximum Limit."]
pub type PkrMaxW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Poker Maximum Limit."]
    #[inline(always)]
    pub fn pkr_max(&self) -> PkrMaxR {
        PkrMaxR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Poker Maximum Limit."]
    #[inline(always)]
    #[must_use]
    pub fn pkr_max(&mut self) -> PkrMaxW<MaxSqPkrmaxSpec> {
        PkrMaxW::new(self, 0)
    }
}
#[doc = "Poker Maximum Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`max_sq_pkrmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_sq_pkrmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxSqPkrmaxSpec;
impl crate::RegisterSpec for MaxSqPkrmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_sq_pkrmax::R`](R) reader structure"]
impl crate::Readable for MaxSqPkrmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`max_sq_pkrmax::W`](W) writer structure"]
impl crate::Writable for MaxSqPkrmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKRMAX to value 0x6920"]
impl crate::Resettable for MaxSqPkrmaxSpec {
    const RESET_VALUE: u32 = 0x6920;
}
