#[doc = "Register `SPIPF188` reader"]
pub type R = crate::R<Spipf188Spec>;
#[doc = "Register `SPIPF188` writer"]
pub type W = crate::W<Spipf188Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf188::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf188::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf188Spec;
impl crate::RegisterSpec for Spipf188Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf188::R`](R) reader structure"]
impl crate::Readable for Spipf188Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf188::W`](W) writer structure"]
impl crate::Writable for Spipf188Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF188 to value 0"]
impl crate::Resettable for Spipf188Spec {}
