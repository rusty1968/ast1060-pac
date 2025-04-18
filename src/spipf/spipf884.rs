#[doc = "Register `SPIPF884` reader"]
pub type R = crate::R<Spipf884Spec>;
#[doc = "Register `SPIPF884` writer"]
pub type W = crate::W<Spipf884Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf884::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf884::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf884Spec;
impl crate::RegisterSpec for Spipf884Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf884::R`](R) reader structure"]
impl crate::Readable for Spipf884Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf884::W`](W) writer structure"]
impl crate::Writable for Spipf884Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF884 to value 0"]
impl crate::Resettable for Spipf884Spec {}
