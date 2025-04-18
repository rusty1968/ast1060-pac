#[doc = "Register `SPIPF394` reader"]
pub type R = crate::R<Spipf394Spec>;
#[doc = "Register `SPIPF394` writer"]
pub type W = crate::W<Spipf394Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf394::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf394::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf394Spec;
impl crate::RegisterSpec for Spipf394Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf394::R`](R) reader structure"]
impl crate::Readable for Spipf394Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf394::W`](W) writer structure"]
impl crate::Writable for Spipf394Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF394 to value 0"]
impl crate::Resettable for Spipf394Spec {}
