#[doc = "Register `SPIPF160` reader"]
pub type R = crate::R<Spipf160Spec>;
#[doc = "Register `SPIPF160` writer"]
pub type W = crate::W<Spipf160Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf160Spec;
impl crate::RegisterSpec for Spipf160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf160::R`](R) reader structure"]
impl crate::Readable for Spipf160Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf160::W`](W) writer structure"]
impl crate::Writable for Spipf160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF160 to value 0"]
impl crate::Resettable for Spipf160Spec {}
