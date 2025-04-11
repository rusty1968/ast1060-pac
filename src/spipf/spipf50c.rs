#[doc = "Register `SPIPF50C` reader"]
pub type R = crate::R<Spipf50cSpec>;
#[doc = "Register `SPIPF50C` writer"]
pub type W = crate::W<Spipf50cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf50c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf50c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf50cSpec;
impl crate::RegisterSpec for Spipf50cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf50c::R`](R) reader structure"]
impl crate::Readable for Spipf50cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf50c::W`](W) writer structure"]
impl crate::Writable for Spipf50cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF50C to value 0"]
impl crate::Resettable for Spipf50cSpec {}
