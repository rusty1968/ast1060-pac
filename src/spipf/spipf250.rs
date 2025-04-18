#[doc = "Register `SPIPF250` reader"]
pub type R = crate::R<Spipf250Spec>;
#[doc = "Register `SPIPF250` writer"]
pub type W = crate::W<Spipf250Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf250::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf250::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf250Spec;
impl crate::RegisterSpec for Spipf250Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf250::R`](R) reader structure"]
impl crate::Readable for Spipf250Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf250::W`](W) writer structure"]
impl crate::Writable for Spipf250Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF250 to value 0"]
impl crate::Resettable for Spipf250Spec {}
