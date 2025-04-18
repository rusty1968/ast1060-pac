#[doc = "Register `SPI034` reader"]
pub type R = crate::R<Spi034Spec>;
#[doc = "Register `SPI034` writer"]
pub type W = crate::W<Spi034Spec>;
#[doc = "Field `StartAddrA31161` reader - Start address A\\[31:16\\]"]
pub type StartAddrA31161R = crate::FieldReader<u16>;
#[doc = "Field `StartAddrA31161` writer - Start address A\\[31:16\\]"]
pub type StartAddrA31161W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EndAddrA31161` reader - End address A\\[31:16\\]"]
pub type EndAddrA31161R = crate::FieldReader<u16>;
#[doc = "Field `EndAddrA31161` writer - End address A\\[31:16\\]"]
pub type EndAddrA31161W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Start address A\\[31:16\\]"]
    #[inline(always)]
    pub fn start_addr_a31161(&self) -> StartAddrA31161R {
        StartAddrA31161R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - End address A\\[31:16\\]"]
    #[inline(always)]
    pub fn end_addr_a31161(&self) -> EndAddrA31161R {
        EndAddrA31161R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start address A\\[31:16\\]"]
    #[inline(always)]
    pub fn start_addr_a31161(&mut self) -> StartAddrA31161W<Spi034Spec> {
        StartAddrA31161W::new(self, 0)
    }
    #[doc = "Bits 16:31 - End address A\\[31:16\\]"]
    #[inline(always)]
    pub fn end_addr_a31161(&mut self) -> EndAddrA31161W<Spi034Spec> {
        EndAddrA31161W::new(self, 16)
    }
}
#[doc = "CE1 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi034Spec;
impl crate::RegisterSpec for Spi034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi034::R`](R) reader structure"]
impl crate::Readable for Spi034Spec {}
#[doc = "`write(|w| ..)` method takes [`spi034::W`](W) writer structure"]
impl crate::Writable for Spi034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI034 to value 0"]
impl crate::Resettable for Spi034Spec {}
