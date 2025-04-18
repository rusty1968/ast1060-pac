#[doc = "Register `SPIPF504` reader"]
pub type R = crate::R<Spipf504Spec>;
#[doc = "Register `SPIPF504` writer"]
pub type W = crate::W<Spipf504Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf504::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf504::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf504Spec;
impl crate::RegisterSpec for Spipf504Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf504::R`](R) reader structure"]
impl crate::Readable for Spipf504Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf504::W`](W) writer structure"]
impl crate::Writable for Spipf504Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF504 to value 0"]
impl crate::Resettable for Spipf504Spec {}
