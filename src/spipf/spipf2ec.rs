#[doc = "Register `SPIPF2EC` reader"]
pub type R = crate::R<Spipf2ecSpec>;
#[doc = "Register `SPIPF2EC` writer"]
pub type W = crate::W<Spipf2ecSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf2ecSpec;
impl crate::RegisterSpec for Spipf2ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf2ec::R`](R) reader structure"]
impl crate::Readable for Spipf2ecSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf2ec::W`](W) writer structure"]
impl crate::Writable for Spipf2ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF2EC to value 0"]
impl crate::Resettable for Spipf2ecSpec {}
