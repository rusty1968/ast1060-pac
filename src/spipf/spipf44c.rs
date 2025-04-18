#[doc = "Register `SPIPF44C` reader"]
pub type R = crate::R<Spipf44cSpec>;
#[doc = "Register `SPIPF44C` writer"]
pub type W = crate::W<Spipf44cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf44c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf44c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf44cSpec;
impl crate::RegisterSpec for Spipf44cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf44c::R`](R) reader structure"]
impl crate::Readable for Spipf44cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf44c::W`](W) writer structure"]
impl crate::Writable for Spipf44cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF44C to value 0"]
impl crate::Resettable for Spipf44cSpec {}
