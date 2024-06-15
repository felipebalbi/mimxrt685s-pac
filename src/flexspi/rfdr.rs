#[doc = "Register `RFDR[%s]` reader"]
pub type R = crate::R<RfdrSpec>;
#[doc = "Field `RXDATA` reader - RX Data"]
pub type RxdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new(self.bits)
    }
}
#[doc = "IP RX FIFO Data Register x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfdrSpec;
impl crate::RegisterSpec for RfdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdr::R`](R) reader structure"]
impl crate::Readable for RfdrSpec {}
#[doc = "`reset()` method sets RFDR[%s]
to value 0"]
impl crate::Resettable for RfdrSpec {
    const RESET_VALUE: u32 = 0;
}
