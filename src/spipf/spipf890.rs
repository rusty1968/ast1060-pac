#[doc = "Register `SPIPF890` reader"]
pub type R = crate::R<Spipf890Spec>;
#[doc = "Register `SPIPF890` writer"]
pub type W = crate::W<Spipf890Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf890::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf890::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf890Spec;
impl crate::RegisterSpec for Spipf890Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf890::R`](R) reader structure"]
impl crate::Readable for Spipf890Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf890::W`](W) writer structure"]
impl crate::Writable for Spipf890Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF890 to value 0"]
impl crate::Resettable for Spipf890Spec {}
