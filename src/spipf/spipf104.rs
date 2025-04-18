#[doc = "Register `SPIPF104` reader"]
pub type R = crate::R<Spipf104Spec>;
#[doc = "Register `SPIPF104` writer"]
pub type W = crate::W<Spipf104Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf104Spec;
impl crate::RegisterSpec for Spipf104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf104::R`](R) reader structure"]
impl crate::Readable for Spipf104Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf104::W`](W) writer structure"]
impl crate::Writable for Spipf104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF104 to value 0"]
impl crate::Resettable for Spipf104Spec {}
