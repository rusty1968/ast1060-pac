#[doc = "Register `SPIPF49C` reader"]
pub type R = crate::R<Spipf49cSpec>;
#[doc = "Register `SPIPF49C` writer"]
pub type W = crate::W<Spipf49cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf49c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf49c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf49cSpec;
impl crate::RegisterSpec for Spipf49cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf49c::R`](R) reader structure"]
impl crate::Readable for Spipf49cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf49c::W`](W) writer structure"]
impl crate::Writable for Spipf49cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF49C to value 0"]
impl crate::Resettable for Spipf49cSpec {}
