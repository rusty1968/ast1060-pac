#[doc = "Register `SPI29C` reader"]
pub type R = crate::R<Spi29cSpec>;
#[doc = "Register `SPI29C` writer"]
pub type W = crate::W<Spi29cSpec>;
#[doc = "Field `DMABufferReadwrPort` reader - DMA Buffer read/write port"]
pub type DmabufferReadwrPortR = crate::FieldReader<u32>;
#[doc = "Field `DMABufferReadwrPort` writer - DMA Buffer read/write port"]
pub type DmabufferReadwrPortW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer read/write port"]
    #[inline(always)]
    pub fn dmabuffer_readwr_port(&self) -> DmabufferReadwrPortR {
        DmabufferReadwrPortR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Buffer read/write port"]
    #[inline(always)]
    pub fn dmabuffer_readwr_port(&mut self) -> DmabufferReadwrPortW<Spi29cSpec> {
        DmabufferReadwrPortW::new(self, 0)
    }
}
#[doc = "DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi29c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi29c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi29cSpec;
impl crate::RegisterSpec for Spi29cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi29c::R`](R) reader structure"]
impl crate::Readable for Spi29cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi29c::W`](W) writer structure"]
impl crate::Writable for Spi29cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI29C to value 0"]
impl crate::Resettable for Spi29cSpec {}
