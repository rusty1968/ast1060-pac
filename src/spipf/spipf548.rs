#[doc = "Register `SPIPF548` reader"]
pub type R = crate::R<Spipf548Spec>;
#[doc = "Register `SPIPF548` writer"]
pub type W = crate::W<Spipf548Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf548::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf548::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf548Spec;
impl crate::RegisterSpec for Spipf548Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf548::R`](R) reader structure"]
impl crate::Readable for Spipf548Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf548::W`](W) writer structure"]
impl crate::Writable for Spipf548Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF548 to value 0"]
impl crate::Resettable for Spipf548Spec {}
