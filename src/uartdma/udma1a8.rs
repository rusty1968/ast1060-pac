#[doc = "Register `UDMA1A8` reader"]
pub type R = crate::R<Udma1a8Spec>;
#[doc = "Register `UDMA1A8` writer"]
pub type W = crate::W<Udma1a8Spec>;
#[doc = "Field `UART12TXBufBaseAddr` reader - UART12 TX buffer base address"]
pub type Uart12txbufBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `UART12TXBufBaseAddr` writer - UART12 TX buffer base address"]
pub type Uart12txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART12 TX buffer base address"]
    #[inline(always)]
    pub fn uart12txbuf_base_addr(&self) -> Uart12txbufBaseAddrR {
        Uart12txbufBaseAddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART12 TX buffer base address"]
    #[inline(always)]
    pub fn uart12txbuf_base_addr(&mut self) -> Uart12txbufBaseAddrW<Udma1a8Spec> {
        Uart12txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART12 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1a8Spec;
impl crate::RegisterSpec for Udma1a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1a8::R`](R) reader structure"]
impl crate::Readable for Udma1a8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1a8::W`](W) writer structure"]
impl crate::Writable for Udma1a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1A8 to value 0"]
impl crate::Resettable for Udma1a8Spec {}
