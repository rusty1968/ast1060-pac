#[doc = "Register `SPIPF610` reader"]
pub type R = crate::R<Spipf610Spec>;
#[doc = "Register `SPIPF610` writer"]
pub type W = crate::W<Spipf610Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf610::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf610::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf610Spec;
impl crate::RegisterSpec for Spipf610Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf610::R`](R) reader structure"]
impl crate::Readable for Spipf610Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf610::W`](W) writer structure"]
impl crate::Writable for Spipf610Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF610 to value 0"]
impl crate::Resettable for Spipf610Spec {}
