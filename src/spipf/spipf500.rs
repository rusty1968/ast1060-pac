#[doc = "Register `SPIPF500` reader"]
pub type R = crate::R<Spipf500Spec>;
#[doc = "Register `SPIPF500` writer"]
pub type W = crate::W<Spipf500Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf500::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf500::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf500Spec;
impl crate::RegisterSpec for Spipf500Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf500::R`](R) reader structure"]
impl crate::Readable for Spipf500Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf500::W`](W) writer structure"]
impl crate::Writable for Spipf500Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF500 to value 0"]
impl crate::Resettable for Spipf500Spec {}
