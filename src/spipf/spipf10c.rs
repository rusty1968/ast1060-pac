#[doc = "Register `SPIPF10C` reader"]
pub type R = crate::R<Spipf10cSpec>;
#[doc = "Register `SPIPF10C` writer"]
pub type W = crate::W<Spipf10cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf10c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf10c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf10cSpec;
impl crate::RegisterSpec for Spipf10cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf10c::R`](R) reader structure"]
impl crate::Readable for Spipf10cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf10c::W`](W) writer structure"]
impl crate::Writable for Spipf10cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF10C to value 0"]
impl crate::Resettable for Spipf10cSpec {}
