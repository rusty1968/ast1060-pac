#[doc = "Register `SPI054` reader"]
pub type R = crate::R<Spi054Spec>;
#[doc = "Register `SPI054` writer"]
pub type W = crate::W<Spi054Spec>;
#[doc = "Field `SPIDummyCycleOutputData` reader - SPI dummy cycle output data"]
pub type SpidummyCycleOutputDataR = crate::FieldReader;
#[doc = "Field `SPIDummyCycleOutputData` writer - SPI dummy cycle output data"]
pub type SpidummyCycleOutputDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - SPI dummy cycle output data"]
    #[inline(always)]
    pub fn spidummy_cycle_output_data(&self) -> SpidummyCycleOutputDataR {
        SpidummyCycleOutputDataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI dummy cycle output data"]
    #[inline(always)]
    pub fn spidummy_cycle_output_data(&mut self) -> SpidummyCycleOutputDataW<Spi054Spec> {
        SpidummyCycleOutputDataW::new(self, 0)
    }
}
#[doc = "SPI Dummy Cycle Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi054Spec;
impl crate::RegisterSpec for Spi054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi054::R`](R) reader structure"]
impl crate::Readable for Spi054Spec {}
#[doc = "`write(|w| ..)` method takes [`spi054::W`](W) writer structure"]
impl crate::Writable for Spi054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI054 to value 0"]
impl crate::Resettable for Spi054Spec {}
