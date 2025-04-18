#[doc = "Register `SPIPF640` reader"]
pub type R = crate::R<Spipf640Spec>;
#[doc = "Register `SPIPF640` writer"]
pub type W = crate::W<Spipf640Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf640::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf640::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf640Spec;
impl crate::RegisterSpec for Spipf640Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf640::R`](R) reader structure"]
impl crate::Readable for Spipf640Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf640::W`](W) writer structure"]
impl crate::Writable for Spipf640Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF640 to value 0"]
impl crate::Resettable for Spipf640Spec {}
