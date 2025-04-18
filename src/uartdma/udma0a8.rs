#[doc = "Register `UDMA0A8` reader"]
pub type R = crate::R<Udma0a8Spec>;
#[doc = "Register `UDMA0A8` writer"]
pub type W = crate::W<Udma0a8Spec>;
#[doc = "Field `UART4TXBufBaseAddr` reader - UART4 TX buffer base address"]
pub type Uart4txbufBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `UART4TXBufBaseAddr` writer - UART4 TX buffer base address"]
pub type Uart4txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART4 TX buffer base address"]
    #[inline(always)]
    pub fn uart4txbuf_base_addr(&self) -> Uart4txbufBaseAddrR {
        Uart4txbufBaseAddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART4 TX buffer base address"]
    #[inline(always)]
    pub fn uart4txbuf_base_addr(&mut self) -> Uart4txbufBaseAddrW<Udma0a8Spec> {
        Uart4txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART4 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0a8Spec;
impl crate::RegisterSpec for Udma0a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0a8::R`](R) reader structure"]
impl crate::Readable for Udma0a8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0a8::W`](W) writer structure"]
impl crate::Writable for Udma0a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0A8 to value 0"]
impl crate::Resettable for Udma0a8Spec {}
