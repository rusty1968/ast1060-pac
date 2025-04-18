#[doc = "Register `I2CC50` reader"]
pub type R = crate::R<I2cc50Spec>;
#[doc = "Register `I2CC50` writer"]
pub type W = crate::W<I2cc50Spec>;
#[doc = "Field `CurDMAOperatingAddrCounter` reader - Current DMA Operating Address Counter"]
pub type CurDmaoperatingAddrCounterR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current DMA Operating Address Counter"]
    #[inline(always)]
    pub fn cur_dmaoperating_addr_counter(&self) -> CurDmaoperatingAddrCounterR {
        CurDmaoperatingAddrCounterR::new(self.bits)
    }
}
impl W {}
#[doc = "Current DMA Operating Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cc50Spec;
impl crate::RegisterSpec for I2cc50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cc50::R`](R) reader structure"]
impl crate::Readable for I2cc50Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cc50::W`](W) writer structure"]
impl crate::Writable for I2cc50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CC50 to value 0"]
impl crate::Resettable for I2cc50Spec {}
