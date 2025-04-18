#[doc = "Register `SPIPF1BC` reader"]
pub type R = crate::R<Spipf1bcSpec>;
#[doc = "Register `SPIPF1BC` writer"]
pub type W = crate::W<Spipf1bcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf1bcSpec;
impl crate::RegisterSpec for Spipf1bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf1bc::R`](R) reader structure"]
impl crate::Readable for Spipf1bcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf1bc::W`](W) writer structure"]
impl crate::Writable for Spipf1bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF1BC to value 0"]
impl crate::Resettable for Spipf1bcSpec {}
