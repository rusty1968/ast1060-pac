#[doc = "Register `SPIPF1EC` reader"]
pub type R = crate::R<Spipf1ecSpec>;
#[doc = "Register `SPIPF1EC` writer"]
pub type W = crate::W<Spipf1ecSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf1ecSpec;
impl crate::RegisterSpec for Spipf1ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf1ec::R`](R) reader structure"]
impl crate::Readable for Spipf1ecSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf1ec::W`](W) writer structure"]
impl crate::Writable for Spipf1ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF1EC to value 0"]
impl crate::Resettable for Spipf1ecSpec {}
