#[doc = "Register `BOOTKEY2_3` reader"]
pub type R = crate::R<BOOTKEY2_3_SPEC>;
#[doc = "Register `BOOTKEY2_3` writer"]
pub type W = crate::W<BOOTKEY2_3_SPEC>;
#[doc = "Field `BOOTKEY2_3` reader - "]
pub type BOOTKEY2_3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey2_3(&self) -> BOOTKEY2_3_R {
        BOOTKEY2_3_R::new(self.bits)
    }
}
impl W {}
#[doc = "Bits 63:48 of SHA-256 hash of boot key 2 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey2_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootkey2_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY2_3_SPEC;
impl crate::RegisterSpec for BOOTKEY2_3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bootkey2_3::R`](R) reader structure"]
impl crate::Readable for BOOTKEY2_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bootkey2_3::W`](W) writer structure"]
impl crate::Writable for BOOTKEY2_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BOOTKEY2_3 to value 0"]
impl crate::Resettable for BOOTKEY2_3_SPEC {
    const RESET_VALUE: u16 = 0;
}
