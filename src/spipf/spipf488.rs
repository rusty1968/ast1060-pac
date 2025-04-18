#[doc = "Register `SPIPF488` reader"]
pub type R = crate::R<Spipf488Spec>;
#[doc = "Register `SPIPF488` writer"]
pub type W = crate::W<Spipf488Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf488::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf488::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf488Spec;
impl crate::RegisterSpec for Spipf488Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf488::R`](R) reader structure"]
impl crate::Readable for Spipf488Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf488::W`](W) writer structure"]
impl crate::Writable for Spipf488Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF488 to value 0"]
impl crate::Resettable for Spipf488Spec {}
