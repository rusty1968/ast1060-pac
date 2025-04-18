#[doc = "Register `SPIPF12C` reader"]
pub type R = crate::R<Spipf12cSpec>;
#[doc = "Register `SPIPF12C` writer"]
pub type W = crate::W<Spipf12cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf12c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf12c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf12cSpec;
impl crate::RegisterSpec for Spipf12cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf12c::R`](R) reader structure"]
impl crate::Readable for Spipf12cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf12c::W`](W) writer structure"]
impl crate::Writable for Spipf12cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF12C to value 0"]
impl crate::Resettable for Spipf12cSpec {}
