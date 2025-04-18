#[doc = "Register `UDMA024` reader"]
pub type R = crate::R<Udma024Spec>;
#[doc = "Register `UDMA024` writer"]
pub type W = crate::W<Udma024Spec>;
#[doc = "Field `UARTRXDMARst` reader - UART RX DMA reset"]
pub type UartrxdmarstR = crate::FieldReader<u16>;
#[doc = "Field `UARTRXDMARst` writer - UART RX DMA reset"]
pub type UartrxdmarstW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UARTRXDMARst1` reader - UART RX DMA reset"]
pub type Uartrxdmarst1R = crate::FieldReader;
#[doc = "Field `UARTRXDMARst1` writer - UART RX DMA reset"]
pub type Uartrxdmarst1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - UART RX DMA reset"]
    #[inline(always)]
    pub fn uartrxdmarst(&self) -> UartrxdmarstR {
        UartrxdmarstR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - UART RX DMA reset"]
    #[inline(always)]
    pub fn uartrxdmarst1(&self) -> Uartrxdmarst1R {
        Uartrxdmarst1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - UART RX DMA reset"]
    #[inline(always)]
    pub fn uartrxdmarst(&mut self) -> UartrxdmarstW<Udma024Spec> {
        UartrxdmarstW::new(self, 0)
    }
    #[doc = "Bits 0:3 - UART RX DMA reset"]
    #[inline(always)]
    pub fn uartrxdmarst1(&mut self) -> Uartrxdmarst1W<Udma024Spec> {
        Uartrxdmarst1W::new(self, 0)
    }
}
#[doc = "UART RX DMA reset\n\nYou can [`read`](crate::Reg::read) this register and get [`udma024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma024Spec;
impl crate::RegisterSpec for Udma024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma024::R`](R) reader structure"]
impl crate::Readable for Udma024Spec {}
#[doc = "`write(|w| ..)` method takes [`udma024::W`](W) writer structure"]
impl crate::Writable for Udma024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA024 to value 0"]
impl crate::Resettable for Udma024Spec {}
