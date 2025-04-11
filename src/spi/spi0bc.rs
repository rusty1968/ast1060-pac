#[doc = "Register `SPI0BC` reader"]
pub type R = crate::R<Spi0bcSpec>;
#[doc = "Register `SPI0BC` writer"]
pub type W = crate::W<Spi0bcSpec>;
#[doc = "Field `SegmentsLowerBoundAddrBit27123` reader - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27123R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsLowerBoundAddrBit27123` writer - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27123W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27123` reader - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27123R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27123` writer - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27123W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27123(&self) -> SegmentsLowerBoundAddrBit27123R {
        SegmentsLowerBoundAddrBit27123R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27123(&self) -> SegmentsUpperBoundAddrBit27123R {
        SegmentsUpperBoundAddrBit27123R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27123(
        &mut self,
    ) -> SegmentsLowerBoundAddrBit27123W<Spi0bcSpec> {
        SegmentsLowerBoundAddrBit27123W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27123(
        &mut self,
    ) -> SegmentsUpperBoundAddrBit27123W<Spi0bcSpec> {
        SegmentsUpperBoundAddrBit27123W::new(self, 16)
    }
}
#[doc = "Write Address Filter Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0bcSpec;
impl crate::RegisterSpec for Spi0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0bc::R`](R) reader structure"]
impl crate::Readable for Spi0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`spi0bc::W`](W) writer structure"]
impl crate::Writable for Spi0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0BC to value 0"]
impl crate::Resettable for Spi0bcSpec {}
