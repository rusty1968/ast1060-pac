#[doc = "Register `SPIPF6DC` reader"]
pub type R = crate::R<Spipf6dcSpec>;
#[doc = "Register `SPIPF6DC` writer"]
pub type W = crate::W<Spipf6dcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf6dcSpec;
impl crate::RegisterSpec for Spipf6dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf6dc::R`](R) reader structure"]
impl crate::Readable for Spipf6dcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf6dc::W`](W) writer structure"]
impl crate::Writable for Spipf6dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF6DC to value 0"]
impl crate::Resettable for Spipf6dcSpec {}
