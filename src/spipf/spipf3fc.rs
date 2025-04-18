#[doc = "Register `SPIPF3FC` reader"]
pub type R = crate::R<Spipf3fcSpec>;
#[doc = "Register `SPIPF3FC` writer"]
pub type W = crate::W<Spipf3fcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf3fcSpec;
impl crate::RegisterSpec for Spipf3fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf3fc::R`](R) reader structure"]
impl crate::Readable for Spipf3fcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf3fc::W`](W) writer structure"]
impl crate::Writable for Spipf3fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF3FC to value 0"]
impl crate::Resettable for Spipf3fcSpec {}
