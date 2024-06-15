#[doc = "Register `CMD_RSP0` reader"]
pub type R = crate::R<CmdRsp0Spec>;
#[doc = "Field `CMDRSP0` reader - Command Response 0"]
pub type Cmdrsp0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 0"]
    #[inline(always)]
    pub fn cmdrsp0(&self) -> Cmdrsp0R {
        Cmdrsp0R::new(self.bits)
    }
}
#[doc = "Command Response0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_rsp0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdRsp0Spec;
impl crate::RegisterSpec for CmdRsp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_rsp0::R`](R) reader structure"]
impl crate::Readable for CmdRsp0Spec {}
#[doc = "`reset()` method sets CMD_RSP0 to value 0"]
impl crate::Resettable for CmdRsp0Spec {
    const RESET_VALUE: u32 = 0;
}
