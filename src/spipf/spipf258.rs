#[doc = "Register `SPIPF258` reader"]
pub type R = crate::R<Spipf258Spec>;
#[doc = "Register `SPIPF258` writer"]
pub type W = crate::W<Spipf258Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf258::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf258::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf258Spec;
impl crate::RegisterSpec for Spipf258Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf258::R`](R) reader structure"]
impl crate::Readable for Spipf258Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf258::W`](W) writer structure"]
impl crate::Writable for Spipf258Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF258 to value 0"]
impl crate::Resettable for Spipf258Spec {}
