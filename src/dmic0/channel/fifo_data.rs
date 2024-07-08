#[doc = "Register `FIFO_DATA` reader"]
pub type R = crate::R<FifoDataSpec>;
#[doc = "Field `DATA` reader - PCM Data"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - PCM Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "FIFO Data\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifoDataSpec;
impl crate::RegisterSpec for FifoDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_data::R`](R) reader structure"]
impl crate::Readable for FifoDataSpec {}
#[doc = "`reset()` method sets FIFO_DATA to value 0"]
impl crate::Resettable for FifoDataSpec {
    const RESET_VALUE: u32 = 0;
}
