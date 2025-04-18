#[doc = "Register `SPIPF744` reader"]
pub type R = crate::R<Spipf744Spec>;
#[doc = "Register `SPIPF744` writer"]
pub type W = crate::W<Spipf744Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf744::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf744::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf744Spec;
impl crate::RegisterSpec for Spipf744Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf744::R`](R) reader structure"]
impl crate::Readable for Spipf744Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf744::W`](W) writer structure"]
impl crate::Writable for Spipf744Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF744 to value 0"]
impl crate::Resettable for Spipf744Spec {}
