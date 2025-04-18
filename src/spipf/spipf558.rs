#[doc = "Register `SPIPF558` reader"]
pub type R = crate::R<Spipf558Spec>;
#[doc = "Register `SPIPF558` writer"]
pub type W = crate::W<Spipf558Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf558::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf558::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf558Spec;
impl crate::RegisterSpec for Spipf558Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf558::R`](R) reader structure"]
impl crate::Readable for Spipf558Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf558::W`](W) writer structure"]
impl crate::Writable for Spipf558Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF558 to value 0"]
impl crate::Resettable for Spipf558Spec {}
