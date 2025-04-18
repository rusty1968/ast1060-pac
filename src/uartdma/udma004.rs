#[doc = "Register `UDMA004` reader"]
pub type R = crate::R<Udma004Spec>;
#[doc = "Register `UDMA004` writer"]
pub type W = crate::W<Udma004Spec>;
#[doc = "Field `UARTRXDMAEnblReg` reader - UART RX DMA enable register"]
pub type UartrxdmaenblRegR = crate::FieldReader<u16>;
#[doc = "Field `UARTRXDMAEnblReg` writer - UART RX DMA enable register"]
pub type UartrxdmaenblRegW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UARTRXDMAEnblReg1` reader - UART RX DMA enable register"]
pub type UartrxdmaenblReg1R = crate::FieldReader;
#[doc = "Field `UARTRXDMAEnblReg1` writer - UART RX DMA enable register"]
pub type UartrxdmaenblReg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - UART RX DMA enable register"]
    #[inline(always)]
    pub fn uartrxdmaenbl_reg(&self) -> UartrxdmaenblRegR {
        UartrxdmaenblRegR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - UART RX DMA enable register"]
    #[inline(always)]
    pub fn uartrxdmaenbl_reg1(&self) -> UartrxdmaenblReg1R {
        UartrxdmaenblReg1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - UART RX DMA enable register"]
    #[inline(always)]
    pub fn uartrxdmaenbl_reg(&mut self) -> UartrxdmaenblRegW<Udma004Spec> {
        UartrxdmaenblRegW::new(self, 0)
    }
    #[doc = "Bits 0:3 - UART RX DMA enable register"]
    #[inline(always)]
    pub fn uartrxdmaenbl_reg1(&mut self) -> UartrxdmaenblReg1W<Udma004Spec> {
        UartrxdmaenblReg1W::new(self, 0)
    }
}
#[doc = "UART RX DMA enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma004Spec;
impl crate::RegisterSpec for Udma004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma004::R`](R) reader structure"]
impl crate::Readable for Udma004Spec {}
#[doc = "`write(|w| ..)` method takes [`udma004::W`](W) writer structure"]
impl crate::Writable for Udma004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA004 to value 0"]
impl crate::Resettable for Udma004Spec {}
