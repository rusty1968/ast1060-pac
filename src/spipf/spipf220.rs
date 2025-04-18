#[doc = "Register `SPIPF220` reader"]
pub type R = crate::R<Spipf220Spec>;
#[doc = "Register `SPIPF220` writer"]
pub type W = crate::W<Spipf220Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf220::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf220::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf220Spec;
impl crate::RegisterSpec for Spipf220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf220::R`](R) reader structure"]
impl crate::Readable for Spipf220Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf220::W`](W) writer structure"]
impl crate::Writable for Spipf220Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF220 to value 0"]
impl crate::Resettable for Spipf220Spec {}
