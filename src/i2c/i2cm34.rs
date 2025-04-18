#[doc = "Register `I2CM34` reader"]
pub type R = crate::R<I2cm34Spec>;
#[doc = "Register `I2CM34` writer"]
pub type W = crate::W<I2cm34Spec>;
#[doc = "Field `SDRAMDMABufferBaseAddr1` reader - SDRAM DMA Buffer Base Address"]
pub type SdramdmabufferBaseAddr1R = crate::FieldReader<u32>;
#[doc = "Field `SDRAMDMABufferBaseAddr1` writer - SDRAM DMA Buffer Base Address"]
pub type SdramdmabufferBaseAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `Reserved0x11` reader - Reserved (0x1)"]
pub type Reserved0x11R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - SDRAM DMA Buffer Base Address"]
    #[inline(always)]
    pub fn sdramdmabuffer_base_addr1(&self) -> SdramdmabufferBaseAddr1R {
        SdramdmabufferBaseAddr1R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0x1)"]
    #[inline(always)]
    pub fn reserved0x11(&self) -> Reserved0x11R {
        Reserved0x11R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - SDRAM DMA Buffer Base Address"]
    #[inline(always)]
    pub fn sdramdmabuffer_base_addr1(&mut self) -> SdramdmabufferBaseAddr1W<I2cm34Spec> {
        SdramdmabufferBaseAddr1W::new(self, 0)
    }
}
#[doc = "\newver{Master DMA Mode Rx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cm34Spec;
impl crate::RegisterSpec for I2cm34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cm34::R`](R) reader structure"]
impl crate::Readable for I2cm34Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cm34::W`](W) writer structure"]
impl crate::Writable for I2cm34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CM34 to value 0"]
impl crate::Resettable for I2cm34Spec {}
