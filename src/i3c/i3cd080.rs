#[doc = "Register `I3CD080` reader"]
pub type R = crate::R<I3cd080Spec>;
#[doc = "Register `I3CD080` writer"]
pub type W = crate::W<I3cd080Spec>;
#[doc = "Field `MXDSMAXRDTURN` reader - MXDS_MAX_RD_TURN"]
pub type MxdsmaxrdturnR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - MXDS_MAX_RD_TURN"]
    #[inline(always)]
    pub fn mxdsmaxrdturn(&self) -> MxdsmaxrdturnR {
        MxdsmaxrdturnR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "MXDS Maximum Read Turnaround Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd080Spec;
impl crate::RegisterSpec for I3cd080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd080::R`](R) reader structure"]
impl crate::Readable for I3cd080Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd080::W`](W) writer structure"]
impl crate::Writable for I3cd080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD080 to value 0"]
impl crate::Resettable for I3cd080Spec {}
