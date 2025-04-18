#[doc = "Register `SPIPF45C` reader"]
pub type R = crate::R<Spipf45cSpec>;
#[doc = "Register `SPIPF45C` writer"]
pub type W = crate::W<Spipf45cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf45c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf45c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf45cSpec;
impl crate::RegisterSpec for Spipf45cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf45c::R`](R) reader structure"]
impl crate::Readable for Spipf45cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf45c::W`](W) writer structure"]
impl crate::Writable for Spipf45cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF45C to value 0"]
impl crate::Resettable for Spipf45cSpec {}
