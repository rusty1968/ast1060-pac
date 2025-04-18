#[doc = "Register `SPIPF584` reader"]
pub type R = crate::R<Spipf584Spec>;
#[doc = "Register `SPIPF584` writer"]
pub type W = crate::W<Spipf584Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf584::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf584::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf584Spec;
impl crate::RegisterSpec for Spipf584Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf584::R`](R) reader structure"]
impl crate::Readable for Spipf584Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf584::W`](W) writer structure"]
impl crate::Writable for Spipf584Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF584 to value 0"]
impl crate::Resettable for Spipf584Spec {}
