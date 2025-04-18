#[doc = "Register `SPIPF494` reader"]
pub type R = crate::R<Spipf494Spec>;
#[doc = "Register `SPIPF494` writer"]
pub type W = crate::W<Spipf494Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf494::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf494::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf494Spec;
impl crate::RegisterSpec for Spipf494Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf494::R`](R) reader structure"]
impl crate::Readable for Spipf494Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf494::W`](W) writer structure"]
impl crate::Writable for Spipf494Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF494 to value 0"]
impl crate::Resettable for Spipf494Spec {}
