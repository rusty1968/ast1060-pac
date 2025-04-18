#[doc = "Register `SPIPF214` reader"]
pub type R = crate::R<Spipf214Spec>;
#[doc = "Register `SPIPF214` writer"]
pub type W = crate::W<Spipf214Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf214::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf214::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf214Spec;
impl crate::RegisterSpec for Spipf214Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf214::R`](R) reader structure"]
impl crate::Readable for Spipf214Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf214::W`](W) writer structure"]
impl crate::Writable for Spipf214Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF214 to value 0"]
impl crate::Resettable for Spipf214Spec {}
