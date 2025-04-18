#[doc = "Register `SPIPF340` reader"]
pub type R = crate::R<Spipf340Spec>;
#[doc = "Register `SPIPF340` writer"]
pub type W = crate::W<Spipf340Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf340::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf340::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf340Spec;
impl crate::RegisterSpec for Spipf340Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf340::R`](R) reader structure"]
impl crate::Readable for Spipf340Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf340::W`](W) writer structure"]
impl crate::Writable for Spipf340Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF340 to value 0"]
impl crate::Resettable for Spipf340Spec {}
