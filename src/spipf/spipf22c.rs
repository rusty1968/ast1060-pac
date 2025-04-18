#[doc = "Register `SPIPF22C` reader"]
pub type R = crate::R<Spipf22cSpec>;
#[doc = "Register `SPIPF22C` writer"]
pub type W = crate::W<Spipf22cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf22c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf22c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf22cSpec;
impl crate::RegisterSpec for Spipf22cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf22c::R`](R) reader structure"]
impl crate::Readable for Spipf22cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf22c::W`](W) writer structure"]
impl crate::Writable for Spipf22cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF22C to value 0"]
impl crate::Resettable for Spipf22cSpec {}
