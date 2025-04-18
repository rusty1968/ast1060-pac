#[doc = "Register `SPIPF894` reader"]
pub type R = crate::R<Spipf894Spec>;
#[doc = "Register `SPIPF894` writer"]
pub type W = crate::W<Spipf894Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf894::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf894::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf894Spec;
impl crate::RegisterSpec for Spipf894Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf894::R`](R) reader structure"]
impl crate::Readable for Spipf894Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf894::W`](W) writer structure"]
impl crate::Writable for Spipf894Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF894 to value 0"]
impl crate::Resettable for Spipf894Spec {}
