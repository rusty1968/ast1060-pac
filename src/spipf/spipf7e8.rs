#[doc = "Register `SPIPF7E8` reader"]
pub type R = crate::R<Spipf7e8Spec>;
#[doc = "Register `SPIPF7E8` writer"]
pub type W = crate::W<Spipf7e8Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf7e8Spec;
impl crate::RegisterSpec for Spipf7e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf7e8::R`](R) reader structure"]
impl crate::Readable for Spipf7e8Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf7e8::W`](W) writer structure"]
impl crate::Writable for Spipf7e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF7E8 to value 0"]
impl crate::Resettable for Spipf7e8Spec {}
