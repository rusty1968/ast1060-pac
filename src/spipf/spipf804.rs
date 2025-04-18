#[doc = "Register `SPIPF804` reader"]
pub type R = crate::R<Spipf804Spec>;
#[doc = "Register `SPIPF804` writer"]
pub type W = crate::W<Spipf804Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf804::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf804::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf804Spec;
impl crate::RegisterSpec for Spipf804Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf804::R`](R) reader structure"]
impl crate::Readable for Spipf804Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf804::W`](W) writer structure"]
impl crate::Writable for Spipf804Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF804 to value 0"]
impl crate::Resettable for Spipf804Spec {}
