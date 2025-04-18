#[doc = "Register `SPIPF34C` reader"]
pub type R = crate::R<Spipf34cSpec>;
#[doc = "Register `SPIPF34C` writer"]
pub type W = crate::W<Spipf34cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf34c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf34c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf34cSpec;
impl crate::RegisterSpec for Spipf34cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf34c::R`](R) reader structure"]
impl crate::Readable for Spipf34cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf34c::W`](W) writer structure"]
impl crate::Writable for Spipf34cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF34C to value 0"]
impl crate::Resettable for Spipf34cSpec {}
