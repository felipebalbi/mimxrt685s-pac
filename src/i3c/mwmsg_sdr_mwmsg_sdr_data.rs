#[doc = "Register `MWMSG_SDR_DATA` writer"]
pub type W = crate::W<MwmsgSdrMwmsgSdrDataSpec>;
#[doc = "Field `DATA16B` writer - Data"]
pub type Data16bW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `END` writer - End of message"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:15 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data16b(&mut self) -> Data16bW<MwmsgSdrMwmsgSdrDataSpec> {
        Data16bW::new(self, 0)
    }
    #[doc = "Bit 16 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<MwmsgSdrMwmsgSdrDataSpec> {
        EndW::new(self, 16)
    }
}
#[doc = "Master Write Message Data in SDR mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_sdr_mwmsg_sdr_data::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwmsgSdrMwmsgSdrDataSpec;
impl crate::RegisterSpec for MwmsgSdrMwmsgSdrDataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwmsg_sdr_mwmsg_sdr_data::W`](W) writer structure"]
impl crate::Writable for MwmsgSdrMwmsgSdrDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWMSG_SDR_DATA to value 0"]
impl crate::Resettable for MwmsgSdrMwmsgSdrDataSpec {
    const RESET_VALUE: u32 = 0;
}
