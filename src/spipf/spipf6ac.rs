#[doc = "Register `SPIPF6AC` reader"]
pub type R = crate::R<Spipf6acSpec>;
#[doc = "Register `SPIPF6AC` writer"]
pub type W = crate::W<Spipf6acSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf6acSpec;
impl crate::RegisterSpec for Spipf6acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf6ac::R`](R) reader structure"]
impl crate::Readable for Spipf6acSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf6ac::W`](W) writer structure"]
impl crate::Writable for Spipf6acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF6AC to value 0"]
impl crate::Resettable for Spipf6acSpec {}
