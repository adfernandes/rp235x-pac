#[doc = "Register `ITCTRL` reader"]
pub type R = crate::R<ITCTRL_SPEC>;
#[doc = "Register `ITCTRL` writer"]
pub type W = crate::W<ITCTRL_SPEC>;
#[doc = "Field `IME` reader - Integration Mode Enable"]
pub type IME_R = crate::BitReader;
#[doc = "Field `IME` writer - Integration Mode Enable"]
pub type IME_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    pub fn ime(&self) -> IME_R {
        IME_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ime(&mut self) -> IME_W<ITCTRL_SPEC> {
        IME_W::new(self, 0)
    }
}
#[doc = "Integration Mode Control register  

You can [`read`](crate::Reg::read) this register and get [`itctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITCTRL_SPEC;
impl crate::RegisterSpec for ITCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itctrl::R`](R) reader structure"]
impl crate::Readable for ITCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`itctrl::W`](W) writer structure"]
impl crate::Writable for ITCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ITCTRL to value 0"]
impl crate::Resettable for ITCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
