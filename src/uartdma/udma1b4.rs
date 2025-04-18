#[doc = "Register `UDMA1B4` reader"]
pub type R = crate::R<Udma1b4Spec>;
#[doc = "Register `UDMA1B4` writer"]
pub type W = crate::W<Udma1b4Spec>;
#[doc = "Field `UART12RXWrPointer` reader - UART12 RX write pointer"]
pub type Uart12rxwrPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART12 RX write pointer"]
    #[inline(always)]
    pub fn uart12rxwr_pointer(&self) -> Uart12rxwrPointerR {
        Uart12rxwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART12 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1b4Spec;
impl crate::RegisterSpec for Udma1b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1b4::R`](R) reader structure"]
impl crate::Readable for Udma1b4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1b4::W`](W) writer structure"]
impl crate::Writable for Udma1b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1B4 to value 0"]
impl crate::Resettable for Udma1b4Spec {}
