#[doc = "Register `SPIPF7CC` reader"]
pub type R = crate::R<Spipf7ccSpec>;
#[doc = "Register `SPIPF7CC` writer"]
pub type W = crate::W<Spipf7ccSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf7ccSpec;
impl crate::RegisterSpec for Spipf7ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf7cc::R`](R) reader structure"]
impl crate::Readable for Spipf7ccSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf7cc::W`](W) writer structure"]
impl crate::Writable for Spipf7ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF7CC to value 0"]
impl crate::Resettable for Spipf7ccSpec {}
