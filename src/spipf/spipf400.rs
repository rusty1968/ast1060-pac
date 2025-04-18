#[doc = "Register `SPIPF400` reader"]
pub type R = crate::R<Spipf400Spec>;
#[doc = "Register `SPIPF400` writer"]
pub type W = crate::W<Spipf400Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf400::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf400::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf400Spec;
impl crate::RegisterSpec for Spipf400Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf400::R`](R) reader structure"]
impl crate::Readable for Spipf400Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf400::W`](W) writer structure"]
impl crate::Writable for Spipf400Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF400 to value 0"]
impl crate::Resettable for Spipf400Spec {}
