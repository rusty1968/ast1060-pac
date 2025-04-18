#[doc = "Register `SPIPF81C` reader"]
pub type R = crate::R<Spipf81cSpec>;
#[doc = "Register `SPIPF81C` writer"]
pub type W = crate::W<Spipf81cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf81c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf81c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf81cSpec;
impl crate::RegisterSpec for Spipf81cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf81c::R`](R) reader structure"]
impl crate::Readable for Spipf81cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf81c::W`](W) writer structure"]
impl crate::Writable for Spipf81cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF81C to value 0"]
impl crate::Resettable for Spipf81cSpec {}
