#[doc = "Register `SPIPF18C` reader"]
pub type R = crate::R<Spipf18cSpec>;
#[doc = "Register `SPIPF18C` writer"]
pub type W = crate::W<Spipf18cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf18c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf18c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf18cSpec;
impl crate::RegisterSpec for Spipf18cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf18c::R`](R) reader structure"]
impl crate::Readable for Spipf18cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf18c::W`](W) writer structure"]
impl crate::Writable for Spipf18cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF18C to value 0"]
impl crate::Resettable for Spipf18cSpec {}
