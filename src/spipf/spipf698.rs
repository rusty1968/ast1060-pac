#[doc = "Register `SPIPF698` reader"]
pub type R = crate::R<Spipf698Spec>;
#[doc = "Register `SPIPF698` writer"]
pub type W = crate::W<Spipf698Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf698::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf698::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf698Spec;
impl crate::RegisterSpec for Spipf698Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf698::R`](R) reader structure"]
impl crate::Readable for Spipf698Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf698::W`](W) writer structure"]
impl crate::Writable for Spipf698Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF698 to value 0"]
impl crate::Resettable for Spipf698Spec {}
