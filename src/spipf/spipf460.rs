#[doc = "Register `SPIPF460` reader"]
pub type R = crate::R<Spipf460Spec>;
#[doc = "Register `SPIPF460` writer"]
pub type W = crate::W<Spipf460Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf460::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf460::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf460Spec;
impl crate::RegisterSpec for Spipf460Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf460::R`](R) reader structure"]
impl crate::Readable for Spipf460Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf460::W`](W) writer structure"]
impl crate::Writable for Spipf460Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF460 to value 0"]
impl crate::Resettable for Spipf460Spec {}
