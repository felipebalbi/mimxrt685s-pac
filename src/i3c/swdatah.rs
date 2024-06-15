#[doc = "Register `SWDATAH` writer"]
pub type W = crate::W<SwdatahSpec>;
#[doc = "Field `DATA0` writer - The 1st byte to send to the master"]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` writer - The 2nd byte to send to the master"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `END` writer - End of message"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:7 - The 1st byte to send to the master"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<SwdatahSpec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - The 2nd byte to send to the master"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<SwdatahSpec> {
        Data1W::new(self, 8)
    }
    #[doc = "Bit 16 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<SwdatahSpec> {
        EndW::new(self, 16)
    }
}
#[doc = "Slave Write Data Half-word Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swdatah::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwdatahSpec;
impl crate::RegisterSpec for SwdatahSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swdatah::W`](W) writer structure"]
impl crate::Writable for SwdatahSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWDATAH to value 0"]
impl crate::Resettable for SwdatahSpec {
    const RESET_VALUE: u32 = 0;
}