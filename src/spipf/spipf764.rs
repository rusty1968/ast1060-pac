#[doc = "Register `SPIPF764` reader"]
pub type R = crate::R<Spipf764Spec>;
#[doc = "Register `SPIPF764` writer"]
pub type W = crate::W<Spipf764Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf764::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf764::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf764Spec;
impl crate::RegisterSpec for Spipf764Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf764::R`](R) reader structure"]
impl crate::Readable for Spipf764Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf764::W`](W) writer structure"]
impl crate::Writable for Spipf764Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF764 to value 0"]
impl crate::Resettable for Spipf764Spec {}
