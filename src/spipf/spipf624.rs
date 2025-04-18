#[doc = "Register `SPIPF624` reader"]
pub type R = crate::R<Spipf624Spec>;
#[doc = "Register `SPIPF624` writer"]
pub type W = crate::W<Spipf624Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf624::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf624::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf624Spec;
impl crate::RegisterSpec for Spipf624Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf624::R`](R) reader structure"]
impl crate::Readable for Spipf624Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf624::W`](W) writer structure"]
impl crate::Writable for Spipf624Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF624 to value 0"]
impl crate::Resettable for Spipf624Spec {}
