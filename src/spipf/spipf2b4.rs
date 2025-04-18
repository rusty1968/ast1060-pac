#[doc = "Register `SPIPF2B4` reader"]
pub type R = crate::R<Spipf2b4Spec>;
#[doc = "Register `SPIPF2B4` writer"]
pub type W = crate::W<Spipf2b4Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf2b4Spec;
impl crate::RegisterSpec for Spipf2b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf2b4::R`](R) reader structure"]
impl crate::Readable for Spipf2b4Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf2b4::W`](W) writer structure"]
impl crate::Writable for Spipf2b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF2B4 to value 0"]
impl crate::Resettable for Spipf2b4Spec {}
