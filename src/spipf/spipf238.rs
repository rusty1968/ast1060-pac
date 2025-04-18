#[doc = "Register `SPIPF238` reader"]
pub type R = crate::R<Spipf238Spec>;
#[doc = "Register `SPIPF238` writer"]
pub type W = crate::W<Spipf238Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf238::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf238::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf238Spec;
impl crate::RegisterSpec for Spipf238Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf238::R`](R) reader structure"]
impl crate::Readable for Spipf238Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf238::W`](W) writer structure"]
impl crate::Writable for Spipf238Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF238 to value 0"]
impl crate::Resettable for Spipf238Spec {}
