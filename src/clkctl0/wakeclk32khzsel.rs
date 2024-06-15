#[doc = "Register `WAKECLK32KHZSEL` reader"]
pub type R = crate::R<Wakeclk32khzselSpec>;
#[doc = "Register `WAKECLK32KHZSEL` writer"]
pub type W = crate::W<Wakeclk32khzselSpec>;
#[doc = "32KHz Wake Clock Low Power Functional Clock Source Selection. . .\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: 32KHz"]
    Freq32khz = 0,
    #[doc = "1: LPOSC (Divided by 32 by default)."]
    Lposc = 1,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    None = 7,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - 32KHz Wake Clock Low Power Functional Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::Freq32khz),
            1 => Some(Sel::Lposc),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "32KHz"]
    #[inline(always)]
    pub fn is_freq_32khz(&self) -> bool {
        *self == Sel::Freq32khz
    }
    #[doc = "LPOSC (Divided by 32 by default)."]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == Sel::Lposc
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - 32KHz Wake Clock Low Power Functional Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32KHz"]
    #[inline(always)]
    pub fn freq_32khz(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Freq32khz)
    }
    #[doc = "LPOSC (Divided by 32 by default)."]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Lposc)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - 32KHz Wake Clock Low Power Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 32KHz Wake Clock Low Power Functional Clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<Wakeclk32khzselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "32k wake clock selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wakeclk32khzsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeclk32khzsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wakeclk32khzselSpec;
impl crate::RegisterSpec for Wakeclk32khzselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeclk32khzsel::R`](R) reader structure"]
impl crate::Readable for Wakeclk32khzselSpec {}
#[doc = "`write(|w| ..)` method takes [`wakeclk32khzsel::W`](W) writer structure"]
impl crate::Writable for Wakeclk32khzselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAKECLK32KHZSEL to value 0x01"]
impl crate::Resettable for Wakeclk32khzselSpec {
    const RESET_VALUE: u32 = 0x01;
}