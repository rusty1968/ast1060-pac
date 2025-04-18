#[doc = "Register `SPIPF5BC` reader"]
pub type R = crate::R<Spipf5bcSpec>;
#[doc = "Register `SPIPF5BC` writer"]
pub type W = crate::W<Spipf5bcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf5bcSpec;
impl crate::RegisterSpec for Spipf5bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf5bc::R`](R) reader structure"]
impl crate::Readable for Spipf5bcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf5bc::W`](W) writer structure"]
impl crate::Writable for Spipf5bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF5BC to value 0"]
impl crate::Resettable for Spipf5bcSpec {}
