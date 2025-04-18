#[doc = "Register `SPIPF240` reader"]
pub type R = crate::R<Spipf240Spec>;
#[doc = "Register `SPIPF240` writer"]
pub type W = crate::W<Spipf240Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf240::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf240::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf240Spec;
impl crate::RegisterSpec for Spipf240Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf240::R`](R) reader structure"]
impl crate::Readable for Spipf240Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf240::W`](W) writer structure"]
impl crate::Writable for Spipf240Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF240 to value 0"]
impl crate::Resettable for Spipf240Spec {}
