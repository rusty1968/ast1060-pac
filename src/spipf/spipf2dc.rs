#[doc = "Register `SPIPF2DC` reader"]
pub type R = crate::R<Spipf2dcSpec>;
#[doc = "Register `SPIPF2DC` writer"]
pub type W = crate::W<Spipf2dcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf2dcSpec;
impl crate::RegisterSpec for Spipf2dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf2dc::R`](R) reader structure"]
impl crate::Readable for Spipf2dcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf2dc::W`](W) writer structure"]
impl crate::Writable for Spipf2dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF2DC to value 0"]
impl crate::Resettable for Spipf2dcSpec {}
