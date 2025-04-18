#[doc = "Register `SPIPF4CC` reader"]
pub type R = crate::R<Spipf4ccSpec>;
#[doc = "Register `SPIPF4CC` writer"]
pub type W = crate::W<Spipf4ccSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf4ccSpec;
impl crate::RegisterSpec for Spipf4ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf4cc::R`](R) reader structure"]
impl crate::Readable for Spipf4ccSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf4cc::W`](W) writer structure"]
impl crate::Writable for Spipf4ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF4CC to value 0"]
impl crate::Resettable for Spipf4ccSpec {}
