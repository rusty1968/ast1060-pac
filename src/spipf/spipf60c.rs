#[doc = "Register `SPIPF60C` reader"]
pub type R = crate::R<Spipf60cSpec>;
#[doc = "Register `SPIPF60C` writer"]
pub type W = crate::W<Spipf60cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf60c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf60c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf60cSpec;
impl crate::RegisterSpec for Spipf60cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf60c::R`](R) reader structure"]
impl crate::Readable for Spipf60cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf60c::W`](W) writer structure"]
impl crate::Writable for Spipf60cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF60C to value 0"]
impl crate::Resettable for Spipf60cSpec {}
