#[doc = "Register `SPIPF23C` reader"]
pub type R = crate::R<Spipf23cSpec>;
#[doc = "Register `SPIPF23C` writer"]
pub type W = crate::W<Spipf23cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf23c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf23c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf23cSpec;
impl crate::RegisterSpec for Spipf23cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf23c::R`](R) reader structure"]
impl crate::Readable for Spipf23cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf23c::W`](W) writer structure"]
impl crate::Writable for Spipf23cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF23C to value 0"]
impl crate::Resettable for Spipf23cSpec {}
