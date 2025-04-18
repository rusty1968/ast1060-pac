#[doc = "Register `SPIPF524` reader"]
pub type R = crate::R<Spipf524Spec>;
#[doc = "Register `SPIPF524` writer"]
pub type W = crate::W<Spipf524Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf524::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf524::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf524Spec;
impl crate::RegisterSpec for Spipf524Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf524::R`](R) reader structure"]
impl crate::Readable for Spipf524Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf524::W`](W) writer structure"]
impl crate::Writable for Spipf524Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF524 to value 0"]
impl crate::Resettable for Spipf524Spec {}
