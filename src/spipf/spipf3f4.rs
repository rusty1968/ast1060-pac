#[doc = "Register `SPIPF3F4` reader"]
pub type R = crate::R<Spipf3f4Spec>;
#[doc = "Register `SPIPF3F4` writer"]
pub type W = crate::W<Spipf3f4Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf3f4Spec;
impl crate::RegisterSpec for Spipf3f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf3f4::R`](R) reader structure"]
impl crate::Readable for Spipf3f4Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf3f4::W`](W) writer structure"]
impl crate::Writable for Spipf3f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF3F4 to value 0"]
impl crate::Resettable for Spipf3f4Spec {}
