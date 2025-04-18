#[doc = "Register `SPIPF770` reader"]
pub type R = crate::R<Spipf770Spec>;
#[doc = "Register `SPIPF770` writer"]
pub type W = crate::W<Spipf770Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf770::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf770::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf770Spec;
impl crate::RegisterSpec for Spipf770Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf770::R`](R) reader structure"]
impl crate::Readable for Spipf770Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf770::W`](W) writer structure"]
impl crate::Writable for Spipf770Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF770 to value 0"]
impl crate::Resettable for Spipf770Spec {}
