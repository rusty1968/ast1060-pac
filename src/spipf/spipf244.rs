#[doc = "Register `SPIPF244` reader"]
pub type R = crate::R<Spipf244Spec>;
#[doc = "Register `SPIPF244` writer"]
pub type W = crate::W<Spipf244Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf244::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf244::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf244Spec;
impl crate::RegisterSpec for Spipf244Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf244::R`](R) reader structure"]
impl crate::Readable for Spipf244Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf244::W`](W) writer structure"]
impl crate::Writable for Spipf244Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF244 to value 0"]
impl crate::Resettable for Spipf244Spec {}
