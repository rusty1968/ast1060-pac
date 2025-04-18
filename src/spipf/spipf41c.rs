#[doc = "Register `SPIPF41C` reader"]
pub type R = crate::R<Spipf41cSpec>;
#[doc = "Register `SPIPF41C` writer"]
pub type W = crate::W<Spipf41cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf41c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf41c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf41cSpec;
impl crate::RegisterSpec for Spipf41cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf41c::R`](R) reader structure"]
impl crate::Readable for Spipf41cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf41c::W`](W) writer structure"]
impl crate::Writable for Spipf41cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF41C to value 0"]
impl crate::Resettable for Spipf41cSpec {}
