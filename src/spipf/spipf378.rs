#[doc = "Register `SPIPF378` reader"]
pub type R = crate::R<Spipf378Spec>;
#[doc = "Register `SPIPF378` writer"]
pub type W = crate::W<Spipf378Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf378::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf378::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf378Spec;
impl crate::RegisterSpec for Spipf378Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf378::R`](R) reader structure"]
impl crate::Readable for Spipf378Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf378::W`](W) writer structure"]
impl crate::Writable for Spipf378Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF378 to value 0"]
impl crate::Resettable for Spipf378Spec {}
