#[doc = "Register `SPIPF69C` reader"]
pub type R = crate::R<Spipf69cSpec>;
#[doc = "Register `SPIPF69C` writer"]
pub type W = crate::W<Spipf69cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf69c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf69c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf69cSpec;
impl crate::RegisterSpec for Spipf69cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf69c::R`](R) reader structure"]
impl crate::Readable for Spipf69cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf69c::W`](W) writer structure"]
impl crate::Writable for Spipf69cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF69C to value 0"]
impl crate::Resettable for Spipf69cSpec {}
