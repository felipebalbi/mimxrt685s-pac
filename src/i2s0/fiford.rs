#[doc = "Register `FIFORD` reader"]
pub type R = crate::R<FifordSpec>;
#[doc = "Field `RXDATA` reader - Received data from the FIFO. The number of bits used depends on configuration details."]
pub type RxdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received data from the FIFO. The number of bits used depends on configuration details."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new(self.bits)
    }
}
#[doc = "FIFO read data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fiford::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifordSpec;
impl crate::RegisterSpec for FifordSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fiford::R`](R) reader structure"]
impl crate::Readable for FifordSpec {}
#[doc = "`reset()` method sets FIFORD to value 0"]
impl crate::Resettable for FifordSpec {
    const RESET_VALUE: u32 = 0;
}
