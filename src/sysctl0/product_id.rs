#[doc = "Register `PRODUCT_ID` reader"]
pub type R = crate::R<ProductIdSpec>;
#[doc = "Field `PRODUCT_ID` reader - This register contains the product ID which is unique for each part number."]
pub type ProductIdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - This register contains the product ID which is unique for each part number."]
    #[inline(always)]
    pub fn product_id(&self) -> ProductIdR {
        ProductIdR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "product ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`product_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProductIdSpec;
impl crate::RegisterSpec for ProductIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`product_id::R`](R) reader structure"]
impl crate::Readable for ProductIdSpec {}
#[doc = "`reset()` method sets PRODUCT_ID to value 0"]
impl crate::Resettable for ProductIdSpec {
    const RESET_VALUE: u32 = 0;
}
