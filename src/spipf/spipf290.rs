#[doc = "Register `SPIPF290` reader"]
pub type R = crate::R<Spipf290Spec>;
#[doc = "Register `SPIPF290` writer"]
pub type W = crate::W<Spipf290Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf290::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf290::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf290Spec;
impl crate::RegisterSpec for Spipf290Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf290::R`](R) reader structure"]
impl crate::Readable for Spipf290Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf290::W`](W) writer structure"]
impl crate::Writable for Spipf290Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF290 to value 0"]
impl crate::Resettable for Spipf290Spec {}
