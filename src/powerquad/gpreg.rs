#[doc = "Register `gpreg[%s]` reader"]
pub type R = crate::R<GpregSpec>;
#[doc = "Register `gpreg[%s]` writer"]
pub type W = crate::W<GpregSpec>;
#[doc = "Field `gpreg` reader - General purpose register bank"]
pub type GpregR = crate::FieldReader<u32>;
#[doc = "Field `gpreg` writer - General purpose register bank"]
pub type GpregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - General purpose register bank"]
    #[inline(always)]
    pub fn gpreg(&self) -> GpregR {
        GpregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - General purpose register bank"]
    #[inline(always)]
    #[must_use]
    pub fn gpreg(&mut self) -> GpregW<GpregSpec> {
        GpregW::new(self, 0)
    }
}
#[doc = "General purpose register bank N.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpregSpec;
impl crate::RegisterSpec for GpregSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpreg::R`](R) reader structure"]
impl crate::Readable for GpregSpec {}
#[doc = "`write(|w| ..)` method takes [`gpreg::W`](W) writer structure"]
impl crate::Writable for GpregSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpreg[%s]
to value 0"]
impl crate::Resettable for GpregSpec {
    const RESET_VALUE: u32 = 0;
}
