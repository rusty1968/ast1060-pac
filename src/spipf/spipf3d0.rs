#[doc = "Register `SPIPF3D0` reader"]
pub type R = crate::R<Spipf3d0Spec>;
#[doc = "Register `SPIPF3D0` writer"]
pub type W = crate::W<Spipf3d0Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf3d0Spec;
impl crate::RegisterSpec for Spipf3d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf3d0::R`](R) reader structure"]
impl crate::Readable for Spipf3d0Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf3d0::W`](W) writer structure"]
impl crate::Writable for Spipf3d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF3D0 to value 0"]
impl crate::Resettable for Spipf3d0Spec {}
