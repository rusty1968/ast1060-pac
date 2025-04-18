#[doc = "Register `UDMA038` reader"]
pub type R = crate::R<Udma038Spec>;
#[doc = "Register `UDMA038` writer"]
pub type W = crate::W<Udma038Spec>;
#[doc = "Field `UARTRXDMAINTEnbl` reader - UART RX DMA interrupt enable"]
pub type UartrxdmaintenblR = crate::FieldReader<u16>;
#[doc = "Field `UARTRXDMAINTEnbl` writer - UART RX DMA interrupt enable"]
pub type UartrxdmaintenblW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UARTRXDMAINTEnbl1` reader - UART RX DMA interrupt enable"]
pub type Uartrxdmaintenbl1R = crate::FieldReader;
#[doc = "Field `UARTRXDMAINTEnbl1` writer - UART RX DMA interrupt enable"]
pub type Uartrxdmaintenbl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - UART RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uartrxdmaintenbl(&self) -> UartrxdmaintenblR {
        UartrxdmaintenblR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - UART RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uartrxdmaintenbl1(&self) -> Uartrxdmaintenbl1R {
        Uartrxdmaintenbl1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - UART RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uartrxdmaintenbl(&mut self) -> UartrxdmaintenblW<Udma038Spec> {
        UartrxdmaintenblW::new(self, 0)
    }
    #[doc = "Bits 0:3 - UART RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uartrxdmaintenbl1(&mut self) -> Uartrxdmaintenbl1W<Udma038Spec> {
        Uartrxdmaintenbl1W::new(self, 0)
    }
}
#[doc = "UART RX DMA interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma038Spec;
impl crate::RegisterSpec for Udma038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma038::R`](R) reader structure"]
impl crate::Readable for Udma038Spec {}
#[doc = "`write(|w| ..)` method takes [`udma038::W`](W) writer structure"]
impl crate::Writable for Udma038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA038 to value 0"]
impl crate::Resettable for Udma038Spec {}
