#[doc = "Register `PRNG_SEED` writer"]
pub type W = crate::W<PrngSeedSpec>;
#[doc = "Field `PRNG_SEED` writer - Random input value used as an entropy source"]
pub type PrngSeedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Random input value used as an entropy source"]
    #[inline(always)]
    #[must_use]
    pub fn prng_seed(&mut self) -> PrngSeedW<PrngSeedSpec> {
        PrngSeedW::new(self, 0)
    }
}
#[doc = "PRNG random seed input value used as an entropy source\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prng_seed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrngSeedSpec;
impl crate::RegisterSpec for PrngSeedSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prng_seed::W`](W) writer structure"]
impl crate::Writable for PrngSeedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRNG_SEED to value 0"]
impl crate::Resettable for PrngSeedSpec {
    const RESET_VALUE: u32 = 0;
}