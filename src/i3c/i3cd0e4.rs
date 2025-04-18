#[doc = "Register `I3CD0E4` reader"]
pub type R = crate::R<I3cd0e4Spec>;
#[doc = "Register `I3CD0E4` writer"]
pub type W = crate::W<I3cd0e4Spec>;
#[doc = "Field `I3CVERTYPE` reader - I3C_VER_TYPE"]
pub type I3cvertypeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - I3C_VER_TYPE"]
    #[inline(always)]
    pub fn i3cvertype(&self) -> I3cvertypeR {
        I3cvertypeR::new(self.bits)
    }
}
impl W {}
#[doc = "I3C Version TYPE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0e4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0e4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0e4Spec;
impl crate::RegisterSpec for I3cd0e4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0e4::R`](R) reader structure"]
impl crate::Readable for I3cd0e4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0e4::W`](W) writer structure"]
impl crate::Writable for I3cd0e4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0E4 to value 0"]
impl crate::Resettable for I3cd0e4Spec {}
