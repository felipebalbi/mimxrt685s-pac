#[doc = "Register `FRQMAX` reader"]
pub type R = crate::R<MaxCntFrqmaxSpec>;
#[doc = "Register `FRQMAX` writer"]
pub type W = crate::W<MaxCntFrqmaxSpec>;
#[doc = "Field `FRQ_MAX` reader - Frequency Counter Maximum Limit"]
pub type FrqMaxR = crate::FieldReader<u32>;
#[doc = "Field `FRQ_MAX` writer - Frequency Counter Maximum Limit"]
pub type FrqMaxW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency Counter Maximum Limit"]
    #[inline(always)]
    pub fn frq_max(&self) -> FrqMaxR {
        FrqMaxR::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Frequency Counter Maximum Limit"]
    #[inline(always)]
    #[must_use]
    pub fn frq_max(&mut self) -> FrqMaxW<MaxCntFrqmaxSpec> {
        FrqMaxW::new(self, 0)
    }
}
#[doc = "Frequency Count Maximum Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`max_cnt_frqmax::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_cnt_frqmax::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxCntFrqmaxSpec;
impl crate::RegisterSpec for MaxCntFrqmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`max_cnt_frqmax::R`](R) reader structure"]
impl crate::Readable for MaxCntFrqmaxSpec {}
#[doc = "`write(|w| ..)` method takes [`max_cnt_frqmax::W`](W) writer structure"]
impl crate::Writable for MaxCntFrqmaxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRQMAX to value 0x6400"]
impl crate::Resettable for MaxCntFrqmaxSpec {
    const RESET_VALUE: u32 = 0x6400;
}
