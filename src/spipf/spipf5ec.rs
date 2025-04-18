#[doc = "Register `SPIPF5EC` reader"]
pub type R = crate::R<Spipf5ecSpec>;
#[doc = "Register `SPIPF5EC` writer"]
pub type W = crate::W<Spipf5ecSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf5ecSpec;
impl crate::RegisterSpec for Spipf5ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf5ec::R`](R) reader structure"]
impl crate::Readable for Spipf5ecSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf5ec::W`](W) writer structure"]
impl crate::Writable for Spipf5ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF5EC to value 0"]
impl crate::Resettable for Spipf5ecSpec {}
