#[doc = "Register `SPIPF16C` reader"]
pub type R = crate::R<Spipf16cSpec>;
#[doc = "Register `SPIPF16C` writer"]
pub type W = crate::W<Spipf16cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf16c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf16c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf16cSpec;
impl crate::RegisterSpec for Spipf16cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf16c::R`](R) reader structure"]
impl crate::Readable for Spipf16cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf16c::W`](W) writer structure"]
impl crate::Writable for Spipf16cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF16C to value 0"]
impl crate::Resettable for Spipf16cSpec {}
