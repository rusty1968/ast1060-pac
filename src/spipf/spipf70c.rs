#[doc = "Register `SPIPF70C` reader"]
pub type R = crate::R<Spipf70cSpec>;
#[doc = "Register `SPIPF70C` writer"]
pub type W = crate::W<Spipf70cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf70c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf70c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf70cSpec;
impl crate::RegisterSpec for Spipf70cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf70c::R`](R) reader structure"]
impl crate::Readable for Spipf70cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf70c::W`](W) writer structure"]
impl crate::Writable for Spipf70cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF70C to value 0"]
impl crate::Resettable for Spipf70cSpec {}
