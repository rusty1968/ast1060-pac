#[doc = "Register `SPIPF208` reader"]
pub type R = crate::R<Spipf208Spec>;
#[doc = "Register `SPIPF208` writer"]
pub type W = crate::W<Spipf208Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf208::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf208::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf208Spec;
impl crate::RegisterSpec for Spipf208Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf208::R`](R) reader structure"]
impl crate::Readable for Spipf208Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf208::W`](W) writer structure"]
impl crate::Writable for Spipf208Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF208 to value 0"]
impl crate::Resettable for Spipf208Spec {}
