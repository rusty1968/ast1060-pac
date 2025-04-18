#[doc = "Register `SPIPF614` reader"]
pub type R = crate::R<Spipf614Spec>;
#[doc = "Register `SPIPF614` writer"]
pub type W = crate::W<Spipf614Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf614::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf614::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf614Spec;
impl crate::RegisterSpec for Spipf614Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf614::R`](R) reader structure"]
impl crate::Readable for Spipf614Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf614::W`](W) writer structure"]
impl crate::Writable for Spipf614Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF614 to value 0"]
impl crate::Resettable for Spipf614Spec {}
