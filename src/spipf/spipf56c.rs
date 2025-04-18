#[doc = "Register `SPIPF56C` reader"]
pub type R = crate::R<Spipf56cSpec>;
#[doc = "Register `SPIPF56C` writer"]
pub type W = crate::W<Spipf56cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf56c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf56c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf56cSpec;
impl crate::RegisterSpec for Spipf56cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf56c::R`](R) reader structure"]
impl crate::Readable for Spipf56cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf56c::W`](W) writer structure"]
impl crate::Writable for Spipf56cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF56C to value 0"]
impl crate::Resettable for Spipf56cSpec {}
