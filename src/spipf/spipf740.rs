#[doc = "Register `SPIPF740` reader"]
pub type R = crate::R<Spipf740Spec>;
#[doc = "Register `SPIPF740` writer"]
pub type W = crate::W<Spipf740Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf740::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf740::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf740Spec;
impl crate::RegisterSpec for Spipf740Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf740::R`](R) reader structure"]
impl crate::Readable for Spipf740Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf740::W`](W) writer structure"]
impl crate::Writable for Spipf740Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF740 to value 0"]
impl crate::Resettable for Spipf740Spec {}
