#[doc = "Register `SPIPF858` reader"]
pub type R = crate::R<Spipf858Spec>;
#[doc = "Register `SPIPF858` writer"]
pub type W = crate::W<Spipf858Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf858::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf858::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf858Spec;
impl crate::RegisterSpec for Spipf858Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf858::R`](R) reader structure"]
impl crate::Readable for Spipf858Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf858::W`](W) writer structure"]
impl crate::Writable for Spipf858Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF858 to value 0"]
impl crate::Resettable for Spipf858Spec {}
