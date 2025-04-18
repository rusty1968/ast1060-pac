#[doc = "Register `SPIPF67C` reader"]
pub type R = crate::R<Spipf67cSpec>;
#[doc = "Register `SPIPF67C` writer"]
pub type W = crate::W<Spipf67cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf67c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf67c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf67cSpec;
impl crate::RegisterSpec for Spipf67cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf67c::R`](R) reader structure"]
impl crate::Readable for Spipf67cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf67c::W`](W) writer structure"]
impl crate::Writable for Spipf67cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF67C to value 0"]
impl crate::Resettable for Spipf67cSpec {}
