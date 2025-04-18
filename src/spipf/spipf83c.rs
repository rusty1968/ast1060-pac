#[doc = "Register `SPIPF83C` reader"]
pub type R = crate::R<Spipf83cSpec>;
#[doc = "Register `SPIPF83C` writer"]
pub type W = crate::W<Spipf83cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf83c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf83c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf83cSpec;
impl crate::RegisterSpec for Spipf83cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf83c::R`](R) reader structure"]
impl crate::Readable for Spipf83cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf83c::W`](W) writer structure"]
impl crate::Writable for Spipf83cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF83C to value 0"]
impl crate::Resettable for Spipf83cSpec {}
