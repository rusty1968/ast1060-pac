#[doc = "Register `SPIPF7B0` reader"]
pub type R = crate::R<Spipf7b0Spec>;
#[doc = "Register `SPIPF7B0` writer"]
pub type W = crate::W<Spipf7b0Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf7b0Spec;
impl crate::RegisterSpec for Spipf7b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf7b0::R`](R) reader structure"]
impl crate::Readable for Spipf7b0Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf7b0::W`](W) writer structure"]
impl crate::Writable for Spipf7b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF7B0 to value 0"]
impl crate::Resettable for Spipf7b0Spec {}
