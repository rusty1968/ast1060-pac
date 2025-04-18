#[doc = "Register `SPIPF73C` reader"]
pub type R = crate::R<Spipf73cSpec>;
#[doc = "Register `SPIPF73C` writer"]
pub type W = crate::W<Spipf73cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf73c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf73c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf73cSpec;
impl crate::RegisterSpec for Spipf73cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf73c::R`](R) reader structure"]
impl crate::Readable for Spipf73cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf73c::W`](W) writer structure"]
impl crate::Writable for Spipf73cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF73C to value 0"]
impl crate::Resettable for Spipf73cSpec {}
