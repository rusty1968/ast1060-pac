#[doc = "Register `I3CD090` reader"]
pub type R = crate::R<I3cd090Spec>;
#[doc = "Register `I3CD090` writer"]
pub type W = crate::W<I3cd090Spec>;
#[doc = "Field `SLVTSXSYMBLCNT` reader - SLV_TSX_SYMBL_CNT"]
pub type SlvtsxsymblcntR = crate::FieldReader;
#[doc = "Field `SLVTSXSYMBLCNT` writer - SLV_TSX_SYMBL_CNT"]
pub type SlvtsxsymblcntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RSVD316` reader - These bits in TSX symbol timing register is reserved."]
pub type Rsvd316R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - SLV_TSX_SYMBL_CNT"]
    #[inline(always)]
    pub fn slvtsxsymblcnt(&self) -> SlvtsxsymblcntR {
        SlvtsxsymblcntR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - These bits in TSX symbol timing register is reserved."]
    #[inline(always)]
    pub fn rsvd316(&self) -> Rsvd316R {
        Rsvd316R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - SLV_TSX_SYMBL_CNT"]
    #[inline(always)]
    pub fn slvtsxsymblcnt(&mut self) -> SlvtsxsymblcntW<I3cd090Spec> {
        SlvtsxsymblcntW::new(self, 0)
    }
}
#[doc = "TSP/TSL Symbol Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd090Spec;
impl crate::RegisterSpec for I3cd090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd090::R`](R) reader structure"]
impl crate::Readable for I3cd090Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd090::W`](W) writer structure"]
impl crate::Writable for I3cd090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD090 to value 0"]
impl crate::Resettable for I3cd090Spec {}
