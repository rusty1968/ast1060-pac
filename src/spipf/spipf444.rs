#[doc = "Register `SPIPF444` reader"]
pub type R = crate::R<Spipf444Spec>;
#[doc = "Register `SPIPF444` writer"]
pub type W = crate::W<Spipf444Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf444::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf444::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf444Spec;
impl crate::RegisterSpec for Spipf444Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf444::R`](R) reader structure"]
impl crate::Readable for Spipf444Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf444::W`](W) writer structure"]
impl crate::Writable for Spipf444Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF444 to value 0"]
impl crate::Resettable for Spipf444Spec {}
