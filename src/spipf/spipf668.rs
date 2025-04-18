#[doc = "Register `SPIPF668` reader"]
pub type R = crate::R<Spipf668Spec>;
#[doc = "Register `SPIPF668` writer"]
pub type W = crate::W<Spipf668Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf668::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf668::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf668Spec;
impl crate::RegisterSpec for Spipf668Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf668::R`](R) reader structure"]
impl crate::Readable for Spipf668Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf668::W`](W) writer structure"]
impl crate::Writable for Spipf668Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF668 to value 0"]
impl crate::Resettable for Spipf668Spec {}
