#[doc = "Register `SPIPF88C` reader"]
pub type R = crate::R<Spipf88cSpec>;
#[doc = "Register `SPIPF88C` writer"]
pub type W = crate::W<Spipf88cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf88c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf88c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf88cSpec;
impl crate::RegisterSpec for Spipf88cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf88c::R`](R) reader structure"]
impl crate::Readable for Spipf88cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf88c::W`](W) writer structure"]
impl crate::Writable for Spipf88cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF88C to value 0"]
impl crate::Resettable for Spipf88cSpec {}
