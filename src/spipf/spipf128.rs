#[doc = "Register `SPIPF128` reader"]
pub type R = crate::R<Spipf128Spec>;
#[doc = "Register `SPIPF128` writer"]
pub type W = crate::W<Spipf128Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf128::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf128::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf128Spec;
impl crate::RegisterSpec for Spipf128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf128::R`](R) reader structure"]
impl crate::Readable for Spipf128Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf128::W`](W) writer structure"]
impl crate::Writable for Spipf128Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF128 to value 0"]
impl crate::Resettable for Spipf128Spec {}
