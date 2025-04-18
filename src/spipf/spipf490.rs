#[doc = "Register `SPIPF490` reader"]
pub type R = crate::R<Spipf490Spec>;
#[doc = "Register `SPIPF490` writer"]
pub type W = crate::W<Spipf490Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf490::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf490::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf490Spec;
impl crate::RegisterSpec for Spipf490Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf490::R`](R) reader structure"]
impl crate::Readable for Spipf490Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf490::W`](W) writer structure"]
impl crate::Writable for Spipf490Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF490 to value 0"]
impl crate::Resettable for Spipf490Spec {}
