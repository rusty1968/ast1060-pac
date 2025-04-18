#[doc = "Register `SPIPF87C` reader"]
pub type R = crate::R<Spipf87cSpec>;
#[doc = "Register `SPIPF87C` writer"]
pub type W = crate::W<Spipf87cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf87c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf87c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf87cSpec;
impl crate::RegisterSpec for Spipf87cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf87c::R`](R) reader structure"]
impl crate::Readable for Spipf87cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf87c::W`](W) writer structure"]
impl crate::Writable for Spipf87cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF87C to value 0"]
impl crate::Resettable for Spipf87cSpec {}
