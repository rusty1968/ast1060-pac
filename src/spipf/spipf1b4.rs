#[doc = "Register `SPIPF1B4` reader"]
pub type R = crate::R<Spipf1b4Spec>;
#[doc = "Register `SPIPF1B4` writer"]
pub type W = crate::W<Spipf1b4Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf1b4Spec;
impl crate::RegisterSpec for Spipf1b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf1b4::R`](R) reader structure"]
impl crate::Readable for Spipf1b4Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf1b4::W`](W) writer structure"]
impl crate::Writable for Spipf1b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF1B4 to value 0"]
impl crate::Resettable for Spipf1b4Spec {}
