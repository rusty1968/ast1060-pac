#[doc = "Register `UDMA000` reader"]
pub type R = crate::R<Udma000Spec>;
#[doc = "Register `UDMA000` writer"]
pub type W = crate::W<Udma000Spec>;
#[doc = "Field `UARTTXDMAEnblReg` reader - UART TX DMA enable register"]
pub type UarttxdmaenblRegR = crate::FieldReader<u16>;
#[doc = "Field `UARTTXDMAEnblReg` writer - UART TX DMA enable register"]
pub type UarttxdmaenblRegW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UARTTXDMAEnblReg1` reader - UART TX DMA enable register"]
pub type UarttxdmaenblReg1R = crate::FieldReader;
#[doc = "Field `UARTTXDMAEnblReg1` writer - UART TX DMA enable register"]
pub type UarttxdmaenblReg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - UART TX DMA enable register"]
    #[inline(always)]
    pub fn uarttxdmaenbl_reg(&self) -> UarttxdmaenblRegR {
        UarttxdmaenblRegR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - UART TX DMA enable register"]
    #[inline(always)]
    pub fn uarttxdmaenbl_reg1(&self) -> UarttxdmaenblReg1R {
        UarttxdmaenblReg1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - UART TX DMA enable register"]
    #[inline(always)]
    pub fn uarttxdmaenbl_reg(&mut self) -> UarttxdmaenblRegW<Udma000Spec> {
        UarttxdmaenblRegW::new(self, 0)
    }
    #[doc = "Bits 0:3 - UART TX DMA enable register"]
    #[inline(always)]
    pub fn uarttxdmaenbl_reg1(&mut self) -> UarttxdmaenblReg1W<Udma000Spec> {
        UarttxdmaenblReg1W::new(self, 0)
    }
}
#[doc = "UART TX DMA enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma000Spec;
impl crate::RegisterSpec for Udma000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma000::R`](R) reader structure"]
impl crate::Readable for Udma000Spec {}
#[doc = "`write(|w| ..)` method takes [`udma000::W`](W) writer structure"]
impl crate::Writable for Udma000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA000 to value 0"]
impl crate::Resettable for Udma000Spec {}
