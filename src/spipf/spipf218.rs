#[doc = "Register `SPIPF218` reader"]
pub type R = crate::R<Spipf218Spec>;
#[doc = "Register `SPIPF218` writer"]
pub type W = crate::W<Spipf218Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf218::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf218::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf218Spec;
impl crate::RegisterSpec for Spipf218Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf218::R`](R) reader structure"]
impl crate::Readable for Spipf218Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf218::W`](W) writer structure"]
impl crate::Writable for Spipf218Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF218 to value 0"]
impl crate::Resettable for Spipf218Spec {}
