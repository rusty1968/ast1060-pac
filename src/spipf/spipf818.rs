#[doc = "Register `SPIPF818` reader"]
pub type R = crate::R<Spipf818Spec>;
#[doc = "Register `SPIPF818` writer"]
pub type W = crate::W<Spipf818Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf818::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf818::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf818Spec;
impl crate::RegisterSpec for Spipf818Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf818::R`](R) reader structure"]
impl crate::Readable for Spipf818Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf818::W`](W) writer structure"]
impl crate::Writable for Spipf818Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF818 to value 0"]
impl crate::Resettable for Spipf818Spec {}
