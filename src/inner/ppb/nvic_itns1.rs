#[doc = "Register `NVIC_ITNS1` reader"]
pub type R = crate::R<NVIC_ITNS1_SPEC>;
#[doc = "Register `NVIC_ITNS1` writer"]
pub type W = crate::W<NVIC_ITNS1_SPEC>;
#[doc = "Field `ITNS` reader - For ITNS\\[m\\]
in NVIC_ITNS*n, `IAAMO the target Security state for interrupt 32*n+m"]
pub type ITNS_R = crate::FieldReader<u32>;
#[doc = "Field `ITNS` writer - For ITNS\\[m\\]
in NVIC_ITNS*n, `IAAMO the target Security state for interrupt 32*n+m"]
pub type ITNS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For ITNS\\[m\\]
in NVIC_ITNS*n, `IAAMO the target Security state for interrupt 32*n+m"]
    #[inline(always)]
    pub fn itns(&self) -> ITNS_R {
        ITNS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - For ITNS\\[m\\]
in NVIC_ITNS*n, `IAAMO the target Security state for interrupt 32*n+m"]
    #[inline(always)]
    #[must_use]
    pub fn itns(&mut self) -> ITNS_W<NVIC_ITNS1_SPEC> {
        ITNS_W::new(self, 0)
    }
}
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state  

You can [`read`](crate::Reg::read) this register and get [`nvic_itns1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvic_itns1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVIC_ITNS1_SPEC;
impl crate::RegisterSpec for NVIC_ITNS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_itns1::R`](R) reader structure"]
impl crate::Readable for NVIC_ITNS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvic_itns1::W`](W) writer structure"]
impl crate::Writable for NVIC_ITNS1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ITNS1 to value 0"]
impl crate::Resettable for NVIC_ITNS1_SPEC {
    const RESET_VALUE: u32 = 0;
}