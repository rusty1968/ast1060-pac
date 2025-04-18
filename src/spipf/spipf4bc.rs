#[doc = "Register `SPIPF4BC` reader"]
pub type R = crate::R<Spipf4bcSpec>;
#[doc = "Register `SPIPF4BC` writer"]
pub type W = crate::W<Spipf4bcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf4bcSpec;
impl crate::RegisterSpec for Spipf4bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf4bc::R`](R) reader structure"]
impl crate::Readable for Spipf4bcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf4bc::W`](W) writer structure"]
impl crate::Writable for Spipf4bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF4BC to value 0"]
impl crate::Resettable for Spipf4bcSpec {}
