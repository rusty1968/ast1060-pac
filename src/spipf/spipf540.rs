#[doc = "Register `SPIPF540` reader"]
pub type R = crate::R<Spipf540Spec>;
#[doc = "Register `SPIPF540` writer"]
pub type W = crate::W<Spipf540Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf540::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf540::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf540Spec;
impl crate::RegisterSpec for Spipf540Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf540::R`](R) reader structure"]
impl crate::Readable for Spipf540Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf540::W`](W) writer structure"]
impl crate::Writable for Spipf540Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF540 to value 0"]
impl crate::Resettable for Spipf540Spec {}
