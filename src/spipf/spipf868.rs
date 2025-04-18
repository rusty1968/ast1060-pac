#[doc = "Register `SPIPF868` reader"]
pub type R = crate::R<Spipf868Spec>;
#[doc = "Register `SPIPF868` writer"]
pub type W = crate::W<Spipf868Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf868::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf868::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf868Spec;
impl crate::RegisterSpec for Spipf868Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf868::R`](R) reader structure"]
impl crate::Readable for Spipf868Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf868::W`](W) writer structure"]
impl crate::Writable for Spipf868Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF868 to value 0"]
impl crate::Resettable for Spipf868Spec {}
