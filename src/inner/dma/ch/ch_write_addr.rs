#[doc = "Register `CH_WRITE_ADDR` reader"]
pub type R = crate::R<CH_WRITE_ADDR_SPEC>;
#[doc = "Register `CH_WRITE_ADDR` writer"]
pub type W = crate::W<CH_WRITE_ADDR_SPEC>;
#[doc = "Field `CH0_WRITE_ADDR` reader - This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub type CH0_WRITE_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CH0_WRITE_ADDR` writer - This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
pub type CH0_WRITE_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    #[inline(always)]
    pub fn ch0_write_addr(&self) -> CH0_WRITE_ADDR_R {
        CH0_WRITE_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_write_addr(&mut self) -> CH0_WRITE_ADDR_W<CH_WRITE_ADDR_SPEC> {
        CH0_WRITE_ADDR_W::new(self, 0)
    }
}
#[doc = "DMA Channel 0 Write Address pointer  

You can [`read`](crate::Reg::read) this register and get [`ch_write_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_write_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_WRITE_ADDR_SPEC;
impl crate::RegisterSpec for CH_WRITE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_write_addr::R`](R) reader structure"]
impl crate::Readable for CH_WRITE_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_write_addr::W`](W) writer structure"]
impl crate::Writable for CH_WRITE_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_WRITE_ADDR to value 0"]
impl crate::Resettable for CH_WRITE_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
