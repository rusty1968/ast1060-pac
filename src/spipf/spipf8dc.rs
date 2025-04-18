#[doc = "Register `SPIPF8DC` reader"]
pub type R = crate::R<Spipf8dcSpec>;
#[doc = "Register `SPIPF8DC` writer"]
pub type W = crate::W<Spipf8dcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf8dcSpec;
impl crate::RegisterSpec for Spipf8dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf8dc::R`](R) reader structure"]
impl crate::Readable for Spipf8dcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf8dc::W`](W) writer structure"]
impl crate::Writable for Spipf8dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF8DC to value 0"]
impl crate::Resettable for Spipf8dcSpec {}
