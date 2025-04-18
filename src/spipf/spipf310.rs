#[doc = "Register `SPIPF310` reader"]
pub type R = crate::R<Spipf310Spec>;
#[doc = "Register `SPIPF310` writer"]
pub type W = crate::W<Spipf310Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf310::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf310::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf310Spec;
impl crate::RegisterSpec for Spipf310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf310::R`](R) reader structure"]
impl crate::Readable for Spipf310Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf310::W`](W) writer structure"]
impl crate::Writable for Spipf310Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF310 to value 0"]
impl crate::Resettable for Spipf310Spec {}
