#[doc = "Register `SPIPF40C` reader"]
pub type R = crate::R<Spipf40cSpec>;
#[doc = "Register `SPIPF40C` writer"]
pub type W = crate::W<Spipf40cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf40c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf40c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf40cSpec;
impl crate::RegisterSpec for Spipf40cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf40c::R`](R) reader structure"]
impl crate::Readable for Spipf40cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf40c::W`](W) writer structure"]
impl crate::Writable for Spipf40cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF40C to value 0"]
impl crate::Resettable for Spipf40cSpec {}
