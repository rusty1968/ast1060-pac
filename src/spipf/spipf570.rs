#[doc = "Register `SPIPF570` reader"]
pub type R = crate::R<Spipf570Spec>;
#[doc = "Register `SPIPF570` writer"]
pub type W = crate::W<Spipf570Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf570::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf570::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf570Spec;
impl crate::RegisterSpec for Spipf570Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf570::R`](R) reader structure"]
impl crate::Readable for Spipf570Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf570::W`](W) writer structure"]
impl crate::Writable for Spipf570Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF570 to value 0"]
impl crate::Resettable for Spipf570Spec {}
