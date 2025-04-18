#[doc = "Register `SCU314` reader"]
pub type R = crate::R<Scu314Spec>;
#[doc = "Register `SCU314` writer"]
pub type W = crate::W<Scu314Spec>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `I3CHCLKSel` reader - I3CHCLK selection"]
pub type I3chclkselR = crate::FieldReader;
#[doc = "Field `I3CHCLKSel` writer - I3CHCLK selection"]
pub type I3chclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `HCLKDividerSel` reader - HCLK divider selection"]
pub type HclkdividerSelR = crate::FieldReader;
#[doc = "Field `HCLKDividerSel` writer - HCLK divider selection"]
pub type HclkdividerSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:12 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:14 - I3CHCLK selection"]
    #[inline(always)]
    pub fn i3chclksel(&self) -> I3chclkselR {
        I3chclkselR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:27 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 0x1fff) as u16)
    }
    #[doc = "Bits 28:30 - HCLK divider selection"]
    #[inline(always)]
    pub fn hclkdivider_sel(&self) -> HclkdividerSelR {
        HclkdividerSelR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu314Spec> {
        Reserved2W::new(self, 0)
    }
    #[doc = "Bits 13:14 - I3CHCLK selection"]
    #[inline(always)]
    pub fn i3chclksel(&mut self) -> I3chclkselW<Scu314Spec> {
        I3chclkselW::new(self, 13)
    }
    #[doc = "Bits 15:27 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu314Spec> {
        Reserved1W::new(self, 15)
    }
    #[doc = "Bits 28:30 - HCLK divider selection"]
    #[inline(always)]
    pub fn hclkdivider_sel(&mut self) -> HclkdividerSelW<Scu314Spec> {
        HclkdividerSelW::new(self, 28)
    }
}
#[doc = "Clock Selection Register Set 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu314::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu314::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu314Spec;
impl crate::RegisterSpec for Scu314Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu314::R`](R) reader structure"]
impl crate::Readable for Scu314Spec {}
#[doc = "`write(|w| ..)` method takes [`scu314::W`](W) writer structure"]
impl crate::Writable for Scu314Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU314 to value 0x4000_0000"]
impl crate::Resettable for Scu314Spec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
