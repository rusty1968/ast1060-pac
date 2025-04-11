#[doc = "Register `SPIPF574` reader"]
pub type R = crate::R<Spipf574Spec>;
#[doc = "Register `SPIPF574` writer"]
pub type W = crate::W<Spipf574Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf574::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf574::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf574Spec;
impl crate::RegisterSpec for Spipf574Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf574::R`](R) reader structure"]
impl crate::Readable for Spipf574Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf574::W`](W) writer structure"]
impl crate::Writable for Spipf574Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF574 to value 0"]
impl crate::Resettable for Spipf574Spec {}
