#[doc = "Register `SPIPF53C` reader"]
pub type R = crate::R<Spipf53cSpec>;
#[doc = "Register `SPIPF53C` writer"]
pub type W = crate::W<Spipf53cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf53c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf53c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf53cSpec;
impl crate::RegisterSpec for Spipf53cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf53c::R`](R) reader structure"]
impl crate::Readable for Spipf53cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf53c::W`](W) writer structure"]
impl crate::Writable for Spipf53cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF53C to value 0"]
impl crate::Resettable for Spipf53cSpec {}
