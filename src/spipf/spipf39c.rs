#[doc = "Register `SPIPF39C` reader"]
pub type R = crate::R<Spipf39cSpec>;
#[doc = "Register `SPIPF39C` writer"]
pub type W = crate::W<Spipf39cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf39c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf39c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf39cSpec;
impl crate::RegisterSpec for Spipf39cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf39c::R`](R) reader structure"]
impl crate::Readable for Spipf39cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf39c::W`](W) writer structure"]
impl crate::Writable for Spipf39cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF39C to value 0"]
impl crate::Resettable for Spipf39cSpec {}
