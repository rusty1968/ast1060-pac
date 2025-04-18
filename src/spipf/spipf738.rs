#[doc = "Register `SPIPF738` reader"]
pub type R = crate::R<Spipf738Spec>;
#[doc = "Register `SPIPF738` writer"]
pub type W = crate::W<Spipf738Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf738::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf738::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf738Spec;
impl crate::RegisterSpec for Spipf738Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf738::R`](R) reader structure"]
impl crate::Readable for Spipf738Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf738::W`](W) writer structure"]
impl crate::Writable for Spipf738Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF738 to value 0"]
impl crate::Resettable for Spipf738Spec {}
