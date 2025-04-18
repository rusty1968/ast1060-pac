#[doc = "Register `SPIPF144` reader"]
pub type R = crate::R<Spipf144Spec>;
#[doc = "Register `SPIPF144` writer"]
pub type W = crate::W<Spipf144Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf144::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf144::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf144Spec;
impl crate::RegisterSpec for Spipf144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf144::R`](R) reader structure"]
impl crate::Readable for Spipf144Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf144::W`](W) writer structure"]
impl crate::Writable for Spipf144Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF144 to value 0"]
impl crate::Resettable for Spipf144Spec {}
