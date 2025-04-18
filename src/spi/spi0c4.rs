#[doc = "Register `SPI0C4` reader"]
pub type R = crate::R<Spi0c4Spec>;
#[doc = "Register `SPI0C4` writer"]
pub type W = crate::W<Spi0c4Spec>;
#[doc = "Field `SegmentsLowerBoundAddrBit27125` reader - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27125R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsLowerBoundAddrBit27125` writer - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27125W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27125` reader - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27125R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27125` writer - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27125W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27125(&self) -> SegmentsLowerBoundAddrBit27125R {
        SegmentsLowerBoundAddrBit27125R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27125(&self) -> SegmentsUpperBoundAddrBit27125R {
        SegmentsUpperBoundAddrBit27125R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27125(
        &mut self,
    ) -> SegmentsLowerBoundAddrBit27125W<Spi0c4Spec> {
        SegmentsLowerBoundAddrBit27125W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27125(
        &mut self,
    ) -> SegmentsUpperBoundAddrBit27125W<Spi0c4Spec> {
        SegmentsUpperBoundAddrBit27125W::new(self, 16)
    }
}
#[doc = "Write Address Filter Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0c4Spec;
impl crate::RegisterSpec for Spi0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0c4::R`](R) reader structure"]
impl crate::Readable for Spi0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`spi0c4::W`](W) writer structure"]
impl crate::Writable for Spi0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0C4 to value 0"]
impl crate::Resettable for Spi0c4Spec {}
