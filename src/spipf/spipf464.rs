#[doc = "Register `SPIPF464` reader"]
pub type R = crate::R<Spipf464Spec>;
#[doc = "Register `SPIPF464` writer"]
pub type W = crate::W<Spipf464Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf464::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf464::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf464Spec;
impl crate::RegisterSpec for Spipf464Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf464::R`](R) reader structure"]
impl crate::Readable for Spipf464Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf464::W`](W) writer structure"]
impl crate::Writable for Spipf464Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF464 to value 0"]
impl crate::Resettable for Spipf464Spec {}
