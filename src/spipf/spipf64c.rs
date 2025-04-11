#[doc = "Register `SPIPF64C` reader"]
pub type R = crate::R<Spipf64cSpec>;
#[doc = "Register `SPIPF64C` writer"]
pub type W = crate::W<Spipf64cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf64c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf64c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf64cSpec;
impl crate::RegisterSpec for Spipf64cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf64c::R`](R) reader structure"]
impl crate::Readable for Spipf64cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf64c::W`](W) writer structure"]
impl crate::Writable for Spipf64cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF64C to value 0"]
impl crate::Resettable for Spipf64cSpec {}
