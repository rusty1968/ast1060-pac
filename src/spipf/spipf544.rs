#[doc = "Register `SPIPF544` reader"]
pub type R = crate::R<Spipf544Spec>;
#[doc = "Register `SPIPF544` writer"]
pub type W = crate::W<Spipf544Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf544::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf544::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf544Spec;
impl crate::RegisterSpec for Spipf544Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf544::R`](R) reader structure"]
impl crate::Readable for Spipf544Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf544::W`](W) writer structure"]
impl crate::Writable for Spipf544Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF544 to value 0"]
impl crate::Resettable for Spipf544Spec {}
