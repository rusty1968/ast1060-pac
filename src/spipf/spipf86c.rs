#[doc = "Register `SPIPF86C` reader"]
pub type R = crate::R<Spipf86cSpec>;
#[doc = "Register `SPIPF86C` writer"]
pub type W = crate::W<Spipf86cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf86c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf86c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf86cSpec;
impl crate::RegisterSpec for Spipf86cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf86c::R`](R) reader structure"]
impl crate::Readable for Spipf86cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf86c::W`](W) writer structure"]
impl crate::Writable for Spipf86cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF86C to value 0"]
impl crate::Resettable for Spipf86cSpec {}
