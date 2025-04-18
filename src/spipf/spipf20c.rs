#[doc = "Register `SPIPF20C` reader"]
pub type R = crate::R<Spipf20cSpec>;
#[doc = "Register `SPIPF20C` writer"]
pub type W = crate::W<Spipf20cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf20c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf20c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf20cSpec;
impl crate::RegisterSpec for Spipf20cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf20c::R`](R) reader structure"]
impl crate::Readable for Spipf20cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf20c::W`](W) writer structure"]
impl crate::Writable for Spipf20cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF20C to value 0"]
impl crate::Resettable for Spipf20cSpec {}
