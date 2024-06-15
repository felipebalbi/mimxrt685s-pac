#[doc = "Register `MDYNADDR` reader"]
pub type R = crate::R<MdynaddrSpec>;
#[doc = "Register `MDYNADDR` writer"]
pub type W = crate::W<MdynaddrSpec>;
#[doc = "Field `DAVALID` reader - Dynamic address valid"]
pub type DavalidR = crate::BitReader;
#[doc = "Field `DAVALID` writer - Dynamic address valid"]
pub type DavalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DADDR` reader - Dynamic address"]
pub type DaddrR = crate::FieldReader;
#[doc = "Field `DADDR` writer - Dynamic address"]
pub type DaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Dynamic address valid"]
    #[inline(always)]
    pub fn davalid(&self) -> DavalidR {
        DavalidR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Dynamic address"]
    #[inline(always)]
    pub fn daddr(&self) -> DaddrR {
        DaddrR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dynamic address valid"]
    #[inline(always)]
    #[must_use]
    pub fn davalid(&mut self) -> DavalidW<MdynaddrSpec> {
        DavalidW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Dynamic address"]
    #[inline(always)]
    #[must_use]
    pub fn daddr(&mut self) -> DaddrW<MdynaddrSpec> {
        DaddrW::new(self, 1)
    }
}
#[doc = "Master Dynamic Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdynaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdynaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdynaddrSpec;
impl crate::RegisterSpec for MdynaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdynaddr::R`](R) reader structure"]
impl crate::Readable for MdynaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdynaddr::W`](W) writer structure"]
impl crate::Writable for MdynaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDYNADDR to value 0"]
impl crate::Resettable for MdynaddrSpec {
    const RESET_VALUE: u32 = 0;
}
