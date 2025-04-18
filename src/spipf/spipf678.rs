#[doc = "Register `SPIPF678` reader"]
pub type R = crate::R<Spipf678Spec>;
#[doc = "Register `SPIPF678` writer"]
pub type W = crate::W<Spipf678Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf678::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf678::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf678Spec;
impl crate::RegisterSpec for Spipf678Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf678::R`](R) reader structure"]
impl crate::Readable for Spipf678Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf678::W`](W) writer structure"]
impl crate::Writable for Spipf678Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF678 to value 0"]
impl crate::Resettable for Spipf678Spec {}
