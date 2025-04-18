#[doc = "Register `SPIPF604` reader"]
pub type R = crate::R<Spipf604Spec>;
#[doc = "Register `SPIPF604` writer"]
pub type W = crate::W<Spipf604Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf604::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf604::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf604Spec;
impl crate::RegisterSpec for Spipf604Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf604::R`](R) reader structure"]
impl crate::Readable for Spipf604Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf604::W`](W) writer structure"]
impl crate::Writable for Spipf604Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF604 to value 0"]
impl crate::Resettable for Spipf604Spec {}
