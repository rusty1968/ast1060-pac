#[doc = "Register `SPIPF268` reader"]
pub type R = crate::R<Spipf268Spec>;
#[doc = "Register `SPIPF268` writer"]
pub type W = crate::W<Spipf268Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf268::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf268::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf268Spec;
impl crate::RegisterSpec for Spipf268Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf268::R`](R) reader structure"]
impl crate::Readable for Spipf268Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf268::W`](W) writer structure"]
impl crate::Writable for Spipf268Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF268 to value 0"]
impl crate::Resettable for Spipf268Spec {}
