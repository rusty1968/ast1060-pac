#[doc = "Register `SPIPF1C8` reader"]
pub type R = crate::R<Spipf1c8Spec>;
#[doc = "Register `SPIPF1C8` writer"]
pub type W = crate::W<Spipf1c8Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf1c8Spec;
impl crate::RegisterSpec for Spipf1c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf1c8::R`](R) reader structure"]
impl crate::Readable for Spipf1c8Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf1c8::W`](W) writer structure"]
impl crate::Writable for Spipf1c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF1C8 to value 0"]
impl crate::Resettable for Spipf1c8Spec {}
