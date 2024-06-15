#[doc = "Register `SYSTEM_STICK_CALIB` reader"]
pub type R = crate::R<SystemStickCalibSpec>;
#[doc = "Register `SYSTEM_STICK_CALIB` writer"]
pub type W = crate::W<SystemStickCalibSpec>;
#[doc = "Field `SYSTEM_STICK_CALIB` reader - Selects the system secure tick calibration value of the M33."]
pub type SystemStickCalibR = crate::FieldReader<u32>;
#[doc = "Field `SYSTEM_STICK_CALIB` writer - Selects the system secure tick calibration value of the M33."]
pub type SystemStickCalibW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Selects the system secure tick calibration value of the M33."]
    #[inline(always)]
    pub fn system_stick_calib(&self) -> SystemStickCalibR {
        SystemStickCalibR::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Selects the system secure tick calibration value of the M33."]
    #[inline(always)]
    #[must_use]
    pub fn system_stick_calib(&mut self) -> SystemStickCalibW<SystemStickCalibSpec> {
        SystemStickCalibW::new(self, 0)
    }
}
#[doc = "system stick calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`system_stick_calib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`system_stick_calib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemStickCalibSpec;
impl crate::RegisterSpec for SystemStickCalibSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`system_stick_calib::R`](R) reader structure"]
impl crate::Readable for SystemStickCalibSpec {}
#[doc = "`write(|w| ..)` method takes [`system_stick_calib::W`](W) writer structure"]
impl crate::Writable for SystemStickCalibSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTEM_STICK_CALIB to value 0"]
impl crate::Resettable for SystemStickCalibSpec {
    const RESET_VALUE: u32 = 0;
}
