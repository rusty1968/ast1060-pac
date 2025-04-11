#[doc = "Register `SPIPF8CC` reader"]
pub type R = crate::R<Spipf8ccSpec>;
#[doc = "Register `SPIPF8CC` writer"]
pub type W = crate::W<Spipf8ccSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf8ccSpec;
impl crate::RegisterSpec for Spipf8ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf8cc::R`](R) reader structure"]
impl crate::Readable for Spipf8ccSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf8cc::W`](W) writer structure"]
impl crate::Writable for Spipf8ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF8CC to value 0"]
impl crate::Resettable for Spipf8ccSpec {}
