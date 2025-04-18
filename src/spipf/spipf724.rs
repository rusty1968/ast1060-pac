#[doc = "Register `SPIPF724` reader"]
pub type R = crate::R<Spipf724Spec>;
#[doc = "Register `SPIPF724` writer"]
pub type W = crate::W<Spipf724Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf724::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf724::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf724Spec;
impl crate::RegisterSpec for Spipf724Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf724::R`](R) reader structure"]
impl crate::Readable for Spipf724Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf724::W`](W) writer structure"]
impl crate::Writable for Spipf724Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF724 to value 0"]
impl crate::Resettable for Spipf724Spec {}
