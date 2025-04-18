#[doc = "Register `SPIPF364` reader"]
pub type R = crate::R<Spipf364Spec>;
#[doc = "Register `SPIPF364` writer"]
pub type W = crate::W<Spipf364Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf364::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf364::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf364Spec;
impl crate::RegisterSpec for Spipf364Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf364::R`](R) reader structure"]
impl crate::Readable for Spipf364Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf364::W`](W) writer structure"]
impl crate::Writable for Spipf364Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF364 to value 0"]
impl crate::Resettable for Spipf364Spec {}
