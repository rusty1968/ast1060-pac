#[doc = "Register `SPIPF428` reader"]
pub type R = crate::R<Spipf428Spec>;
#[doc = "Register `SPIPF428` writer"]
pub type W = crate::W<Spipf428Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf428::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf428::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf428Spec;
impl crate::RegisterSpec for Spipf428Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf428::R`](R) reader structure"]
impl crate::Readable for Spipf428Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf428::W`](W) writer structure"]
impl crate::Writable for Spipf428Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF428 to value 0"]
impl crate::Resettable for Spipf428Spec {}
