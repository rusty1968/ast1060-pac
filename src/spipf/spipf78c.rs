#[doc = "Register `SPIPF78C` reader"]
pub type R = crate::R<Spipf78cSpec>;
#[doc = "Register `SPIPF78C` writer"]
pub type W = crate::W<Spipf78cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf78c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf78c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf78cSpec;
impl crate::RegisterSpec for Spipf78cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf78c::R`](R) reader structure"]
impl crate::Readable for Spipf78cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf78c::W`](W) writer structure"]
impl crate::Writable for Spipf78cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF78C to value 0"]
impl crate::Resettable for Spipf78cSpec {}
