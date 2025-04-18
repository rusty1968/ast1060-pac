#[doc = "Register `SPIPF164` reader"]
pub type R = crate::R<Spipf164Spec>;
#[doc = "Register `SPIPF164` writer"]
pub type W = crate::W<Spipf164Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf164::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf164::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf164Spec;
impl crate::RegisterSpec for Spipf164Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf164::R`](R) reader structure"]
impl crate::Readable for Spipf164Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf164::W`](W) writer structure"]
impl crate::Writable for Spipf164Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF164 to value 0"]
impl crate::Resettable for Spipf164Spec {}
