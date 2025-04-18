#[doc = "Register `SPIPF2FC` reader"]
pub type R = crate::R<Spipf2fcSpec>;
#[doc = "Register `SPIPF2FC` writer"]
pub type W = crate::W<Spipf2fcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf2fcSpec;
impl crate::RegisterSpec for Spipf2fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf2fc::R`](R) reader structure"]
impl crate::Readable for Spipf2fcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf2fc::W`](W) writer structure"]
impl crate::Writable for Spipf2fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF2FC to value 0"]
impl crate::Resettable for Spipf2fcSpec {}
