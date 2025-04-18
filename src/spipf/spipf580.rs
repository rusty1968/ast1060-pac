#[doc = "Register `SPIPF580` reader"]
pub type R = crate::R<Spipf580Spec>;
#[doc = "Register `SPIPF580` writer"]
pub type W = crate::W<Spipf580Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf580::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf580::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf580Spec;
impl crate::RegisterSpec for Spipf580Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf580::R`](R) reader structure"]
impl crate::Readable for Spipf580Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf580::W`](W) writer structure"]
impl crate::Writable for Spipf580Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF580 to value 0"]
impl crate::Resettable for Spipf580Spec {}
