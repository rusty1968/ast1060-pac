#[doc = "Register `SPIPF2BC` reader"]
pub type R = crate::R<Spipf2bcSpec>;
#[doc = "Register `SPIPF2BC` writer"]
pub type W = crate::W<Spipf2bcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf2bcSpec;
impl crate::RegisterSpec for Spipf2bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf2bc::R`](R) reader structure"]
impl crate::Readable for Spipf2bcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf2bc::W`](W) writer structure"]
impl crate::Writable for Spipf2bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF2BC to value 0"]
impl crate::Resettable for Spipf2bcSpec {}
