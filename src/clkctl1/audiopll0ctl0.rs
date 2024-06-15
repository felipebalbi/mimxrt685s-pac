#[doc = "Register `AUDIOPLL0CTL0` reader"]
pub type R = crate::R<Audiopll0ctl0Spec>;
#[doc = "Register `AUDIOPLL0CTL0` writer"]
pub type W = crate::W<Audiopll0ctl0Spec>;
#[doc = "AUDIOPLL0 BYPASS Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypass {
    #[doc = "0: PFD output is PFD programmed clock."]
    ProgrammedClk = 0,
    #[doc = "1: PFD output is AUDIOPLL0 reference input clock. (Bypass Mode)"]
    Bypass = 1,
}
impl From<Bypass> for bool {
    #[inline(always)]
    fn from(variant: Bypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - AUDIOPLL0 BYPASS Mode"]
pub type BypassR = crate::BitReader<Bypass>;
impl BypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypass {
        match self.bits {
            false => Bypass::ProgrammedClk,
            true => Bypass::Bypass,
        }
    }
    #[doc = "PFD output is PFD programmed clock."]
    #[inline(always)]
    pub fn is_programmed_clk(&self) -> bool {
        *self == Bypass::ProgrammedClk
    }
    #[doc = "PFD output is AUDIOPLL0 reference input clock. (Bypass Mode)"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Bypass::Bypass
    }
}
#[doc = "Field `BYPASS` writer - AUDIOPLL0 BYPASS Mode"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG, Bypass>;
impl<'a, REG> BypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PFD output is PFD programmed clock."]
    #[inline(always)]
    pub fn programmed_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::ProgrammedClk)
    }
    #[doc = "PFD output is AUDIOPLL0 reference input clock. (Bypass Mode)"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Bypass)
    }
}
#[doc = "AUDIOPLL0 Reset:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: AUDIOPLL0 reset is removed."]
    Normal = 0,
    #[doc = "1: AUDIOPLL0 is placed into reset."]
    Reset = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - AUDIOPLL0 Reset:"]
pub type ResetR = crate::BitReader<Reset>;
impl ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::Normal,
            true => Reset::Reset,
        }
    }
    #[doc = "AUDIOPLL0 reset is removed."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Reset::Normal
    }
    #[doc = "AUDIOPLL0 is placed into reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Reset::Reset
    }
}
#[doc = "Field `RESET` writer - AUDIOPLL0 Reset:"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUDIOPLL0 reset is removed."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Normal)
    }
    #[doc = "AUDIOPLL0 is placed into reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Reset)
    }
}
#[doc = "Hold Ring Off Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HoldringoffEna {
    #[doc = "0: disbale"]
    Dsiable = 0,
    #[doc = "1: enable"]
    Enable = 1,
}
impl From<HoldringoffEna> for bool {
    #[inline(always)]
    fn from(variant: HoldringoffEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOLDRINGOFF_ENA` reader - Hold Ring Off Control"]
pub type HoldringoffEnaR = crate::BitReader<HoldringoffEna>;
impl HoldringoffEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HoldringoffEna {
        match self.bits {
            false => HoldringoffEna::Dsiable,
            true => HoldringoffEna::Enable,
        }
    }
    #[doc = "disbale"]
    #[inline(always)]
    pub fn is_dsiable(&self) -> bool {
        *self == HoldringoffEna::Dsiable
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HoldringoffEna::Enable
    }
}
#[doc = "Field `HOLDRINGOFF_ENA` writer - Hold Ring Off Control"]
pub type HoldringoffEnaW<'a, REG> = crate::BitWriter<'a, REG, HoldringoffEna>;
impl<'a, REG> HoldringoffEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disbale"]
    #[inline(always)]
    pub fn dsiable(self) -> &'a mut crate::W<REG> {
        self.variant(HoldringoffEna::Dsiable)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HoldringoffEna::Enable)
    }
}
#[doc = "Multiplication Factor for FAUDIOPLL0_OUTPUT:\n\nValue on reset: 22"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mult {
    #[doc = "16: Div 16"]
    Div16 = 16,
    #[doc = "17: Div 17"]
    Div17 = 17,
    #[doc = "20: Div 20"]
    Div20 = 20,
    #[doc = "22: Div 22"]
    Div22 = 22,
    #[doc = "27: Div 27"]
    Div27 = 27,
    #[doc = "33: Div 33"]
    Div33 = 33,
}
impl From<Mult> for u8 {
    #[inline(always)]
    fn from(variant: Mult) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mult {
    type Ux = u8;
}
impl crate::IsEnum for Mult {}
#[doc = "Field `MULT` reader - Multiplication Factor for FAUDIOPLL0_OUTPUT:"]
pub type MultR = crate::FieldReader<Mult>;
impl MultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mult> {
        match self.bits {
            16 => Some(Mult::Div16),
            17 => Some(Mult::Div17),
            20 => Some(Mult::Div20),
            22 => Some(Mult::Div22),
            27 => Some(Mult::Div27),
            33 => Some(Mult::Div33),
            _ => None,
        }
    }
    #[doc = "Div 16"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == Mult::Div16
    }
    #[doc = "Div 17"]
    #[inline(always)]
    pub fn is_div_17(&self) -> bool {
        *self == Mult::Div17
    }
    #[doc = "Div 20"]
    #[inline(always)]
    pub fn is_div_20(&self) -> bool {
        *self == Mult::Div20
    }
    #[doc = "Div 22"]
    #[inline(always)]
    pub fn is_div_22(&self) -> bool {
        *self == Mult::Div22
    }
    #[doc = "Div 27"]
    #[inline(always)]
    pub fn is_div_27(&self) -> bool {
        *self == Mult::Div27
    }
    #[doc = "Div 33"]
    #[inline(always)]
    pub fn is_div_33(&self) -> bool {
        *self == Mult::Div33
    }
}
#[doc = "Field `MULT` writer - Multiplication Factor for FAUDIOPLL0_OUTPUT:"]
pub type MultW<'a, REG> = crate::FieldWriter<'a, REG, 8, Mult>;
impl<'a, REG> MultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Div 16"]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::Div16)
    }
    #[doc = "Div 17"]
    #[inline(always)]
    pub fn div_17(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::Div17)
    }
    #[doc = "Div 20"]
    #[inline(always)]
    pub fn div_20(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::Div20)
    }
    #[doc = "Div 22"]
    #[inline(always)]
    pub fn div_22(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::Div22)
    }
    #[doc = "Div 27"]
    #[inline(always)]
    pub fn div_27(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::Div27)
    }
    #[doc = "Div 33"]
    #[inline(always)]
    pub fn div_33(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::Div33)
    }
}
impl R {
    #[doc = "Bit 0 - AUDIOPLL0 BYPASS Mode"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AUDIOPLL0 Reset:"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 13 - Hold Ring Off Control"]
    #[inline(always)]
    pub fn holdringoff_ena(&self) -> HoldringoffEnaR {
        HoldringoffEnaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Multiplication Factor for FAUDIOPLL0_OUTPUT:"]
    #[inline(always)]
    pub fn mult(&self) -> MultR {
        MultR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AUDIOPLL0 BYPASS Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<Audiopll0ctl0Spec> {
        BypassW::new(self, 0)
    }
    #[doc = "Bit 1 - AUDIOPLL0 Reset:"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<Audiopll0ctl0Spec> {
        ResetW::new(self, 1)
    }
    #[doc = "Bit 13 - Hold Ring Off Control"]
    #[inline(always)]
    #[must_use]
    pub fn holdringoff_ena(&mut self) -> HoldringoffEnaW<Audiopll0ctl0Spec> {
        HoldringoffEnaW::new(self, 13)
    }
    #[doc = "Bits 16:23 - Multiplication Factor for FAUDIOPLL0_OUTPUT:"]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MultW<Audiopll0ctl0Spec> {
        MultW::new(self, 16)
    }
}
#[doc = "audio pll0 control0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audiopll0ctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audiopll0ctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Audiopll0ctl0Spec;
impl crate::RegisterSpec for Audiopll0ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audiopll0ctl0::R`](R) reader structure"]
impl crate::Readable for Audiopll0ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`audiopll0ctl0::W`](W) writer structure"]
impl crate::Writable for Audiopll0ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDIOPLL0CTL0 to value 0x0016_0002"]
impl crate::Resettable for Audiopll0ctl0Spec {
    const RESET_VALUE: u32 = 0x0016_0002;
}