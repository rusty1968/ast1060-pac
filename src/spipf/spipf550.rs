#[doc = "Register `SPIPF550` reader"]
pub type R = crate::R<Spipf550Spec>;
#[doc = "Register `SPIPF550` writer"]
pub type W = crate::W<Spipf550Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf550::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf550::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf550Spec;
impl crate::RegisterSpec for Spipf550Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf550::R`](R) reader structure"]
impl crate::Readable for Spipf550Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf550::W`](W) writer structure"]
impl crate::Writable for Spipf550Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF550 to value 0"]
impl crate::Resettable for Spipf550Spec {}
