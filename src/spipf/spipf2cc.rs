#[doc = "Register `SPIPF2CC` reader"]
pub type R = crate::R<Spipf2ccSpec>;
#[doc = "Register `SPIPF2CC` writer"]
pub type W = crate::W<Spipf2ccSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf2ccSpec;
impl crate::RegisterSpec for Spipf2ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf2cc::R`](R) reader structure"]
impl crate::Readable for Spipf2ccSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf2cc::W`](W) writer structure"]
impl crate::Writable for Spipf2ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF2CC to value 0"]
impl crate::Resettable for Spipf2ccSpec {}
