#[doc = "Register `SPIPF6BC` reader"]
pub type R = crate::R<Spipf6bcSpec>;
#[doc = "Register `SPIPF6BC` writer"]
pub type W = crate::W<Spipf6bcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf6bcSpec;
impl crate::RegisterSpec for Spipf6bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf6bc::R`](R) reader structure"]
impl crate::Readable for Spipf6bcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf6bc::W`](W) writer structure"]
impl crate::Writable for Spipf6bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF6BC to value 0"]
impl crate::Resettable for Spipf6bcSpec {}
