#[doc = "Register `UDMA158` reader"]
pub type R = crate::R<Udma158Spec>;
#[doc = "Register `UDMA158` writer"]
pub type W = crate::W<Udma158Spec>;
#[doc = "Field `UART9TXBufBaseAddr` reader - UART9 TX buffer base address"]
pub type Uart9txbufBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `UART9TXBufBaseAddr` writer - UART9 TX buffer base address"]
pub type Uart9txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART9 TX buffer base address"]
    #[inline(always)]
    pub fn uart9txbuf_base_addr(&self) -> Uart9txbufBaseAddrR {
        Uart9txbufBaseAddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART9 TX buffer base address"]
    #[inline(always)]
    pub fn uart9txbuf_base_addr(&mut self) -> Uart9txbufBaseAddrW<Udma158Spec> {
        Uart9txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART9 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma158::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma158::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma158Spec;
impl crate::RegisterSpec for Udma158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma158::R`](R) reader structure"]
impl crate::Readable for Udma158Spec {}
#[doc = "`write(|w| ..)` method takes [`udma158::W`](W) writer structure"]
impl crate::Writable for Udma158Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA158 to value 0"]
impl crate::Resettable for Udma158Spec {}
