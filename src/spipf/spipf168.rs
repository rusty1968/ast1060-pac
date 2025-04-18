#[doc = "Register `SPIPF168` reader"]
pub type R = crate::R<Spipf168Spec>;
#[doc = "Register `SPIPF168` writer"]
pub type W = crate::W<Spipf168Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf168::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf168::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf168Spec;
impl crate::RegisterSpec for Spipf168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf168::R`](R) reader structure"]
impl crate::Readable for Spipf168Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf168::W`](W) writer structure"]
impl crate::Writable for Spipf168Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF168 to value 0"]
impl crate::Resettable for Spipf168Spec {}
