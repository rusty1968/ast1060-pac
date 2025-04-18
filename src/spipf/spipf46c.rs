#[doc = "Register `SPIPF46C` reader"]
pub type R = crate::R<Spipf46cSpec>;
#[doc = "Register `SPIPF46C` writer"]
pub type W = crate::W<Spipf46cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf46c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf46c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf46cSpec;
impl crate::RegisterSpec for Spipf46cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf46c::R`](R) reader structure"]
impl crate::Readable for Spipf46cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf46c::W`](W) writer structure"]
impl crate::Writable for Spipf46cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF46C to value 0"]
impl crate::Resettable for Spipf46cSpec {}
