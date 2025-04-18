#[doc = "Register `SPIPF8EC` reader"]
pub type R = crate::R<Spipf8ecSpec>;
#[doc = "Register `SPIPF8EC` writer"]
pub type W = crate::W<Spipf8ecSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf8ecSpec;
impl crate::RegisterSpec for Spipf8ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf8ec::R`](R) reader structure"]
impl crate::Readable for Spipf8ecSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf8ec::W`](W) writer structure"]
impl crate::Writable for Spipf8ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF8EC to value 0"]
impl crate::Resettable for Spipf8ecSpec {}
