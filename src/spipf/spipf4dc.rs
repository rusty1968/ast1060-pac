#[doc = "Register `SPIPF4DC` reader"]
pub type R = crate::R<Spipf4dcSpec>;
#[doc = "Register `SPIPF4DC` writer"]
pub type W = crate::W<Spipf4dcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf4dcSpec;
impl crate::RegisterSpec for Spipf4dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf4dc::R`](R) reader structure"]
impl crate::Readable for Spipf4dcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf4dc::W`](W) writer structure"]
impl crate::Writable for Spipf4dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF4DC to value 0"]
impl crate::Resettable for Spipf4dcSpec {}
