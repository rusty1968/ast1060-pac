#[doc = "Register `SPIPF190` reader"]
pub type R = crate::R<Spipf190Spec>;
#[doc = "Register `SPIPF190` writer"]
pub type W = crate::W<Spipf190Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf190::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf190::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf190Spec;
impl crate::RegisterSpec for Spipf190Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf190::R`](R) reader structure"]
impl crate::Readable for Spipf190Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf190::W`](W) writer structure"]
impl crate::Writable for Spipf190Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF190 to value 0"]
impl crate::Resettable for Spipf190Spec {}
