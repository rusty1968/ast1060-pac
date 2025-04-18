#[doc = "Register `I3CD0CC` reader"]
pub type R = crate::R<I3cd0ccSpec>;
#[doc = "Register `I3CD0CC` writer"]
pub type W = crate::W<I3cd0ccSpec>;
#[doc = "Field `I3CEXTTERMNLCNT` reader - I3C_EXT_TERMN_LCNT"]
pub type I3cexttermnlcntR = crate::FieldReader;
#[doc = "Field `I3CEXTTERMNLCNT` writer - I3C_EXT_TERMN_LCNT"]
pub type I3cexttermnlcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSVD1` reader - reserved"]
pub type Rsvd1R = crate::FieldReader<u16>;
#[doc = "Field `I3CTSSKEWCNT` reader - I3C_TS_SKEW_CNT"]
pub type I3ctsskewcntR = crate::FieldReader;
#[doc = "Field `I3CTSSKEWCNT` writer - I3C_TS_SKEW_CNT"]
pub type I3ctsskewcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - I3C_EXT_TERMN_LCNT"]
    #[inline(always)]
    pub fn i3cexttermnlcnt(&self) -> I3cexttermnlcntR {
        I3cexttermnlcntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - I3C_TS_SKEW_CNT"]
    #[inline(always)]
    pub fn i3ctsskewcnt(&self) -> I3ctsskewcntR {
        I3ctsskewcntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - I3C_EXT_TERMN_LCNT"]
    #[inline(always)]
    pub fn i3cexttermnlcnt(&mut self) -> I3cexttermnlcntW<I3cd0ccSpec> {
        I3cexttermnlcntW::new(self, 0)
    }
    #[doc = "Bits 16:19 - I3C_TS_SKEW_CNT"]
    #[inline(always)]
    pub fn i3ctsskewcnt(&mut self) -> I3ctsskewcntW<I3cd0ccSpec> {
        I3ctsskewcntW::new(self, 16)
    }
}
#[doc = "SCL Termination Bit Low count Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0ccSpec;
impl crate::RegisterSpec for I3cd0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0cc::R`](R) reader structure"]
impl crate::Readable for I3cd0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0cc::W`](W) writer structure"]
impl crate::Writable for I3cd0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0CC to value 0"]
impl crate::Resettable for I3cd0ccSpec {}
