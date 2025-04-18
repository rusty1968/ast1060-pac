#[doc = "Register `SPIPF36C` reader"]
pub type R = crate::R<Spipf36cSpec>;
#[doc = "Register `SPIPF36C` writer"]
pub type W = crate::W<Spipf36cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf36c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf36c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf36cSpec;
impl crate::RegisterSpec for Spipf36cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf36c::R`](R) reader structure"]
impl crate::Readable for Spipf36cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf36c::W`](W) writer structure"]
impl crate::Writable for Spipf36cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF36C to value 0"]
impl crate::Resettable for Spipf36cSpec {}
