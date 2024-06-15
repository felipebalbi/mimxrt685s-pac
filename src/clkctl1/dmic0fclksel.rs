#[doc = "Register `DMIC0FCLKSEL` reader"]
pub type R = crate::R<Dmic0fclkselSpec>;
#[doc = "Register `DMIC0FCLKSEL` writer"]
pub type W = crate::W<Dmic0fclkselSpec>;
#[doc = "DMIC Functional Clock Source Selection. . .\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: SFRO Clock."]
    SfroClk = 0,
    #[doc = "1: FFRO Clock."]
    FfroClk = 1,
    #[doc = "2: Audio PLL Clock."]
    AudioPllClk = 2,
    #[doc = "3: Master Clock In."]
    MasterClk = 3,
    #[doc = "4: Low Power Oscillator Clock (LPOSC)."]
    Lposc = 4,
    #[doc = "5: 32KHZ Wake Clk."]
    WakeClk32khz = 5,
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
#[doc = "Field `SEL` reader - DMIC Functional Clock Source Selection. . ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::SfroClk),
            1 => Some(Sel::FfroClk),
            2 => Some(Sel::AudioPllClk),
            3 => Some(Sel::MasterClk),
            4 => Some(Sel::Lposc),
            5 => Some(Sel::WakeClk32khz),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn is_sfro_clk(&self) -> bool {
        *self == Sel::SfroClk
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn is_ffro_clk(&self) -> bool {
        *self == Sel::FfroClk
    }
    #[doc = "Audio PLL Clock."]
    #[inline(always)]
    pub fn is_audio_pll_clk(&self) -> bool {
        *self == Sel::AudioPllClk
    }
    #[doc = "Master Clock In."]
    #[inline(always)]
    pub fn is_master_clk(&self) -> bool {
        *self == Sel::MasterClk
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == Sel::Lposc
    }
    #[doc = "32KHZ Wake Clk."]
    #[inline(always)]
    pub fn is_wake_clk_32khz(&self) -> bool {
        *self == Sel::WakeClk32khz
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - DMIC Functional Clock Source Selection. . ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SFRO Clock."]
    #[inline(always)]
    pub fn sfro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::SfroClk)
    }
    #[doc = "FFRO Clock."]
    #[inline(always)]
    pub fn ffro_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::FfroClk)
    }
    #[doc = "Audio PLL Clock."]
    #[inline(always)]
    pub fn audio_pll_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::AudioPllClk)
    }
    #[doc = "Master Clock In."]
    #[inline(always)]
    pub fn master_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::MasterClk)
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Lposc)
    }
    #[doc = "32KHZ Wake Clk."]
    #[inline(always)]
    pub fn wake_clk_32khz(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::WakeClk32khz)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - DMIC Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - DMIC Functional Clock Source Selection. . ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<Dmic0fclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "DMIC0 clk selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmic0fclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmic0fclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmic0fclkselSpec;
impl crate::RegisterSpec for Dmic0fclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmic0fclksel::R`](R) reader structure"]
impl crate::Readable for Dmic0fclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`dmic0fclksel::W`](W) writer structure"]
impl crate::Writable for Dmic0fclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMIC0FCLKSEL to value 0x07"]
impl crate::Resettable for Dmic0fclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}