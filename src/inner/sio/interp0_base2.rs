#[doc = "Register `INTERP0_BASE2` reader"]
pub type R = crate::R<INTERP0_BASE2_SPEC>;
#[doc = "Register `INTERP0_BASE2` writer"]
pub type W = crate::W<INTERP0_BASE2_SPEC>;
#[doc = "Field `INTERP0_BASE2` reader - "]
pub type INTERP0_BASE2_R = crate::FieldReader<u32>;
#[doc = "Field `INTERP0_BASE2` writer - "]
pub type INTERP0_BASE2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interp0_base2(&self) -> INTERP0_BASE2_R {
        INTERP0_BASE2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn interp0_base2(&mut self) -> INTERP0_BASE2_W<INTERP0_BASE2_SPEC> {
        INTERP0_BASE2_W::new(self, 0)
    }
}
#[doc = "Read/write access to BASE2 register.  

You can [`read`](crate::Reg::read) this register and get [`interp0_base2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_base2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_BASE2_SPEC;
impl crate::RegisterSpec for INTERP0_BASE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_base2::R`](R) reader structure"]
impl crate::Readable for INTERP0_BASE2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp0_base2::W`](W) writer structure"]
impl crate::Writable for INTERP0_BASE2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP0_BASE2 to value 0"]
impl crate::Resettable for INTERP0_BASE2_SPEC {
    const RESET_VALUE: u32 = 0;
}