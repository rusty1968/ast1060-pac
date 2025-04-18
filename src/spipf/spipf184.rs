#[doc = "Register `SPIPF184` reader"]
pub type R = crate::R<Spipf184Spec>;
#[doc = "Register `SPIPF184` writer"]
pub type W = crate::W<Spipf184Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf184::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf184::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf184Spec;
impl crate::RegisterSpec for Spipf184Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf184::R`](R) reader structure"]
impl crate::Readable for Spipf184Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf184::W`](W) writer structure"]
impl crate::Writable for Spipf184Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF184 to value 0"]
impl crate::Resettable for Spipf184Spec {}
