#[doc = "Register `SPIPF618` reader"]
pub type R = crate::R<Spipf618Spec>;
#[doc = "Register `SPIPF618` writer"]
pub type W = crate::W<Spipf618Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf618::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf618::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf618Spec;
impl crate::RegisterSpec for Spipf618Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf618::R`](R) reader structure"]
impl crate::Readable for Spipf618Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf618::W`](W) writer structure"]
impl crate::Writable for Spipf618Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF618 to value 0"]
impl crate::Resettable for Spipf618Spec {}
