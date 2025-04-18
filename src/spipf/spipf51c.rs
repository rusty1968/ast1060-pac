#[doc = "Register `SPIPF51C` reader"]
pub type R = crate::R<Spipf51cSpec>;
#[doc = "Register `SPIPF51C` writer"]
pub type W = crate::W<Spipf51cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf51c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf51c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf51cSpec;
impl crate::RegisterSpec for Spipf51cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf51c::R`](R) reader structure"]
impl crate::Readable for Spipf51cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf51c::W`](W) writer structure"]
impl crate::Writable for Spipf51cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF51C to value 0"]
impl crate::Resettable for Spipf51cSpec {}
