#[doc = "Register `SPIPF7AC` reader"]
pub type R = crate::R<Spipf7acSpec>;
#[doc = "Register `SPIPF7AC` writer"]
pub type W = crate::W<Spipf7acSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf7acSpec;
impl crate::RegisterSpec for Spipf7acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf7ac::R`](R) reader structure"]
impl crate::Readable for Spipf7acSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf7ac::W`](W) writer structure"]
impl crate::Writable for Spipf7acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF7AC to value 0"]
impl crate::Resettable for Spipf7acSpec {}
