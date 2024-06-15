#[doc = "Register `KEY_BLOCK` reader"]
pub type R = crate::R<KeyBlockSpec>;
#[doc = "Register `KEY_BLOCK` writer"]
pub type W = crate::W<KeyBlockSpec>;
#[doc = "Field `KEY_BLOCK` reader - key block register"]
pub type KeyBlockR = crate::FieldReader<u32>;
#[doc = "Field `KEY_BLOCK` writer - key block register"]
pub type KeyBlockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - key block register"]
    #[inline(always)]
    pub fn key_block(&self) -> KeyBlockR {
        KeyBlockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - key block register"]
    #[inline(always)]
    #[must_use]
    pub fn key_block(&mut self) -> KeyBlockW<KeyBlockSpec> {
        KeyBlockW::new(self, 0)
    }
}
#[doc = "Key block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_block::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_block::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyBlockSpec;
impl crate::RegisterSpec for KeyBlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_block::R`](R) reader structure"]
impl crate::Readable for KeyBlockSpec {}
#[doc = "`write(|w| ..)` method takes [`key_block::W`](W) writer structure"]
impl crate::Writable for KeyBlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_BLOCK to value 0x3cc3_5aa5"]
impl crate::Resettable for KeyBlockSpec {
    const RESET_VALUE: u32 = 0x3cc3_5aa5;
}
