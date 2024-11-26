#[doc = "Register `OUT_SET` reader"]
pub type R = crate::R<OutSetSpec>;
#[doc = "Register `OUT_SET` writer"]
pub type W = crate::W<OutSetSpec>;
#[doc = "Field `SET` reader - A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
pub type SetR = crate::FieldReader<u16>;
#[doc = "Field `SET` writer - A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `OUT(0-15)` reader - "]
pub type OutR = crate::BitReader;
#[doc = "Field `OUT(0-15)` writer - "]
pub type OutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new((self.bits & 0xffff) as u16)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OUT0` field.</div>"]
    #[inline(always)]
    pub fn out(&self, n: u8) -> OutR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OutR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn out_iter(&self) -> impl Iterator<Item = OutR> + '_ {
        (0..16).map(move |n| OutR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - OUT0"]
    #[inline(always)]
    pub fn out0(&self) -> OutR {
        OutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUT1"]
    #[inline(always)]
    pub fn out1(&self) -> OutR {
        OutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OUT2"]
    #[inline(always)]
    pub fn out2(&self) -> OutR {
        OutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OUT3"]
    #[inline(always)]
    pub fn out3(&self) -> OutR {
        OutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT4"]
    #[inline(always)]
    pub fn out4(&self) -> OutR {
        OutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OUT5"]
    #[inline(always)]
    pub fn out5(&self) -> OutR {
        OutR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OUT6"]
    #[inline(always)]
    pub fn out6(&self) -> OutR {
        OutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - OUT7"]
    #[inline(always)]
    pub fn out7(&self) -> OutR {
        OutR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT8"]
    #[inline(always)]
    pub fn out8(&self) -> OutR {
        OutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OUT9"]
    #[inline(always)]
    pub fn out9(&self) -> OutR {
        OutR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OUT10"]
    #[inline(always)]
    pub fn out10(&self) -> OutR {
        OutR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OUT11"]
    #[inline(always)]
    pub fn out11(&self) -> OutR {
        OutR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OUT12"]
    #[inline(always)]
    pub fn out12(&self) -> OutR {
        OutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OUT13"]
    #[inline(always)]
    pub fn out13(&self) -> OutR {
        OutR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OUT14"]
    #[inline(always)]
    pub fn out14(&self) -> OutR {
        OutR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - OUT15"]
    #[inline(always)]
    pub fn out15(&self) -> OutR {
        OutR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_SET")
            .field("set_", &self.set_())
            .field("out0", &self.out0())
            .field("out1", &self.out1())
            .field("out2", &self.out2())
            .field("out3", &self.out3())
            .field("out4", &self.out4())
            .field("out5", &self.out5())
            .field("out6", &self.out6())
            .field("out7", &self.out7())
            .field("out8", &self.out8())
            .field("out9", &self.out9())
            .field("out10", &self.out10())
            .field("out11", &self.out11())
            .field("out12", &self.out12())
            .field("out13", &self.out13())
            .field("out14", &self.out14())
            .field("out15", &self.out15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - A 1 in bit m selects event m to set output n (or clear it if SETCLRn = 0x1 or 0x2) output 0 = bit 0, output 1 = bit 1, etc. The number of bits = number of events in this SCT. When the counter is used in bi-directional mode, it is possible to reverse the action specified by the output set and clear registers when counting down, See the OUTPUTCTRL register."]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<OutSetSpec> {
        SetW::new(self, 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OUT0` field.</div>"]
    #[inline(always)]
    pub fn out(&mut self, n: u8) -> OutW<OutSetSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OutW::new(self, n)
    }
    #[doc = "Bit 0 - OUT0"]
    #[inline(always)]
    pub fn out0(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 0)
    }
    #[doc = "Bit 1 - OUT1"]
    #[inline(always)]
    pub fn out1(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 1)
    }
    #[doc = "Bit 2 - OUT2"]
    #[inline(always)]
    pub fn out2(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 2)
    }
    #[doc = "Bit 3 - OUT3"]
    #[inline(always)]
    pub fn out3(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT4"]
    #[inline(always)]
    pub fn out4(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 4)
    }
    #[doc = "Bit 5 - OUT5"]
    #[inline(always)]
    pub fn out5(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 5)
    }
    #[doc = "Bit 6 - OUT6"]
    #[inline(always)]
    pub fn out6(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 6)
    }
    #[doc = "Bit 7 - OUT7"]
    #[inline(always)]
    pub fn out7(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 7)
    }
    #[doc = "Bit 8 - OUT8"]
    #[inline(always)]
    pub fn out8(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 8)
    }
    #[doc = "Bit 9 - OUT9"]
    #[inline(always)]
    pub fn out9(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 9)
    }
    #[doc = "Bit 10 - OUT10"]
    #[inline(always)]
    pub fn out10(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 10)
    }
    #[doc = "Bit 11 - OUT11"]
    #[inline(always)]
    pub fn out11(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 11)
    }
    #[doc = "Bit 12 - OUT12"]
    #[inline(always)]
    pub fn out12(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 12)
    }
    #[doc = "Bit 13 - OUT13"]
    #[inline(always)]
    pub fn out13(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 13)
    }
    #[doc = "Bit 14 - OUT14"]
    #[inline(always)]
    pub fn out14(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 14)
    }
    #[doc = "Bit 15 - OUT15"]
    #[inline(always)]
    pub fn out15(&mut self) -> OutW<OutSetSpec> {
        OutW::new(self, 15)
    }
}
#[doc = "SCT output 0 set register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutSetSpec;
impl crate::RegisterSpec for OutSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_set::R`](R) reader structure"]
impl crate::Readable for OutSetSpec {}
#[doc = "`write(|w| ..)` method takes [`out_set::W`](W) writer structure"]
impl crate::Writable for OutSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_SET to value 0"]
impl crate::Resettable for OutSetSpec {
    const RESET_VALUE: u32 = 0;
}
