#[doc = "Register `SPI12C` reader"]
pub type R = crate::R<Spi12cSpec>;
#[doc = "Register `SPI12C` writer"]
pub type W = crate::W<Spi12cSpec>;
#[doc = "Field `FullyQualifiedCmd17` reader - Fully qualified Command"]
pub type FullyQualifiedCmd17R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd17` writer - Fully qualified Command"]
pub type FullyQualifiedCmd17W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd8` reader - Fully qualified Command"]
pub type FullyQualifiedCmd8R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd8` writer - Fully qualified Command"]
pub type FullyQualifiedCmd8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::FieldReader<u16>;
#[doc = "Field `EnblEntry8` reader - Enable Entry"]
pub type EnblEntry8R = crate::BitReader;
#[doc = "Field `EnblEntry8` writer - Enable Entry"]
pub type EnblEntry8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd17(&self) -> FullyQualifiedCmd17R {
        FullyQualifiedCmd17R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd8(&self) -> FullyQualifiedCmd8R {
        FullyQualifiedCmd8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry8(&self) -> EnblEntry8R {
        EnblEntry8R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd17(&mut self) -> FullyQualifiedCmd17W<Spi12cSpec> {
        FullyQualifiedCmd17W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd8(&mut self) -> FullyQualifiedCmd8W<Spi12cSpec> {
        FullyQualifiedCmd8W::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry8(&mut self) -> EnblEntry8W<Spi12cSpec> {
        EnblEntry8W::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi12c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi12c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi12cSpec;
impl crate::RegisterSpec for Spi12cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi12c::R`](R) reader structure"]
impl crate::Readable for Spi12cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi12c::W`](W) writer structure"]
impl crate::Writable for Spi12cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI12C to value 0"]
impl crate::Resettable for Spi12cSpec {}
