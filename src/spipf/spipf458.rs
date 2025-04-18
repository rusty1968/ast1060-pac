#[doc = "Register `SPIPF458` reader"]
pub type R = crate::R<Spipf458Spec>;
#[doc = "Register `SPIPF458` writer"]
pub type W = crate::W<Spipf458Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf458::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf458::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf458Spec;
impl crate::RegisterSpec for Spipf458Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf458::R`](R) reader structure"]
impl crate::Readable for Spipf458Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf458::W`](W) writer structure"]
impl crate::Writable for Spipf458Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF458 to value 0"]
impl crate::Resettable for Spipf458Spec {}
