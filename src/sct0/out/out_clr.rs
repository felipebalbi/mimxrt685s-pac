#[doc = "Register `OUT_CLR` reader"]
pub type R = crate::R<OutClrSpec>;
#[doc = "Register `OUT_CLR` writer"]
pub type W = crate::W<OutClrSpec>;
#[doc = "Field `CLR(0-15)` reader - "]
pub type ClrR = crate::BitReader;
#[doc = "Field `CLR(0-15)` writer - "]
pub type ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CLR0` field.</div>"]
    #[inline(always)]
    pub fn clr(&self, n: u8) -> ClrR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ClrR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn clr_iter(&self) -> impl Iterator<Item = ClrR> + '_ {
        (0..16).map(move |n| ClrR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - CLR0"]
    #[inline(always)]
    pub fn clr0(&self) -> ClrR {
        ClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLR1"]
    #[inline(always)]
    pub fn clr1(&self) -> ClrR {
        ClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLR2"]
    #[inline(always)]
    pub fn clr2(&self) -> ClrR {
        ClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLR3"]
    #[inline(always)]
    pub fn clr3(&self) -> ClrR {
        ClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CLR4"]
    #[inline(always)]
    pub fn clr4(&self) -> ClrR {
        ClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CLR5"]
    #[inline(always)]
    pub fn clr5(&self) -> ClrR {
        ClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CLR6"]
    #[inline(always)]
    pub fn clr6(&self) -> ClrR {
        ClrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CLR7"]
    #[inline(always)]
    pub fn clr7(&self) -> ClrR {
        ClrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CLR8"]
    #[inline(always)]
    pub fn clr8(&self) -> ClrR {
        ClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CLR9"]
    #[inline(always)]
    pub fn clr9(&self) -> ClrR {
        ClrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CLR10"]
    #[inline(always)]
    pub fn clr10(&self) -> ClrR {
        ClrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CLR11"]
    #[inline(always)]
    pub fn clr11(&self) -> ClrR {
        ClrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CLR12"]
    #[inline(always)]
    pub fn clr12(&self) -> ClrR {
        ClrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CLR13"]
    #[inline(always)]
    pub fn clr13(&self) -> ClrR {
        ClrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CLR14"]
    #[inline(always)]
    pub fn clr14(&self) -> ClrR {
        ClrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CLR15"]
    #[inline(always)]
    pub fn clr15(&self) -> ClrR {
        ClrR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CLR")
            .field("clr0", &self.clr0())
            .field("clr1", &self.clr1())
            .field("clr2", &self.clr2())
            .field("clr3", &self.clr3())
            .field("clr4", &self.clr4())
            .field("clr5", &self.clr5())
            .field("clr6", &self.clr6())
            .field("clr7", &self.clr7())
            .field("clr8", &self.clr8())
            .field("clr9", &self.clr9())
            .field("clr10", &self.clr10())
            .field("clr11", &self.clr11())
            .field("clr12", &self.clr12())
            .field("clr13", &self.clr13())
            .field("clr14", &self.clr14())
            .field("clr15", &self.clr15())
            .finish()
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CLR0` field.</div>"]
    #[inline(always)]
    pub fn clr(&mut self, n: u8) -> ClrW<OutClrSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        ClrW::new(self, n)
    }
    #[doc = "Bit 0 - CLR0"]
    #[inline(always)]
    pub fn clr0(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 0)
    }
    #[doc = "Bit 1 - CLR1"]
    #[inline(always)]
    pub fn clr1(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 1)
    }
    #[doc = "Bit 2 - CLR2"]
    #[inline(always)]
    pub fn clr2(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 2)
    }
    #[doc = "Bit 3 - CLR3"]
    #[inline(always)]
    pub fn clr3(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 3)
    }
    #[doc = "Bit 4 - CLR4"]
    #[inline(always)]
    pub fn clr4(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 4)
    }
    #[doc = "Bit 5 - CLR5"]
    #[inline(always)]
    pub fn clr5(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 5)
    }
    #[doc = "Bit 6 - CLR6"]
    #[inline(always)]
    pub fn clr6(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 6)
    }
    #[doc = "Bit 7 - CLR7"]
    #[inline(always)]
    pub fn clr7(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 7)
    }
    #[doc = "Bit 8 - CLR8"]
    #[inline(always)]
    pub fn clr8(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 8)
    }
    #[doc = "Bit 9 - CLR9"]
    #[inline(always)]
    pub fn clr9(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 9)
    }
    #[doc = "Bit 10 - CLR10"]
    #[inline(always)]
    pub fn clr10(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 10)
    }
    #[doc = "Bit 11 - CLR11"]
    #[inline(always)]
    pub fn clr11(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 11)
    }
    #[doc = "Bit 12 - CLR12"]
    #[inline(always)]
    pub fn clr12(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 12)
    }
    #[doc = "Bit 13 - CLR13"]
    #[inline(always)]
    pub fn clr13(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 13)
    }
    #[doc = "Bit 14 - CLR14"]
    #[inline(always)]
    pub fn clr14(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 14)
    }
    #[doc = "Bit 15 - CLR15"]
    #[inline(always)]
    pub fn clr15(&mut self) -> ClrW<OutClrSpec> {
        ClrW::new(self, 15)
    }
}
#[doc = "SCT output 0 clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutClrSpec;
impl crate::RegisterSpec for OutClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_clr::R`](R) reader structure"]
impl crate::Readable for OutClrSpec {}
#[doc = "`write(|w| ..)` method takes [`out_clr::W`](W) writer structure"]
impl crate::Writable for OutClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_CLR to value 0"]
impl crate::Resettable for OutClrSpec {
    const RESET_VALUE: u32 = 0;
}
