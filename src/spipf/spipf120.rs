#[doc = "Register `SPIPF120` reader"]
pub type R = crate::R<Spipf120Spec>;
#[doc = "Register `SPIPF120` writer"]
pub type W = crate::W<Spipf120Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf120::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf120::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf120Spec;
impl crate::RegisterSpec for Spipf120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf120::R`](R) reader structure"]
impl crate::Readable for Spipf120Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf120::W`](W) writer structure"]
impl crate::Writable for Spipf120Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF120 to value 0"]
impl crate::Resettable for Spipf120Spec {}
