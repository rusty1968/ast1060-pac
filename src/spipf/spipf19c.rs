#[doc = "Register `SPIPF19C` reader"]
pub type R = crate::R<Spipf19cSpec>;
#[doc = "Register `SPIPF19C` writer"]
pub type W = crate::W<Spipf19cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf19c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf19c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf19cSpec;
impl crate::RegisterSpec for Spipf19cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf19c::R`](R) reader structure"]
impl crate::Readable for Spipf19cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf19c::W`](W) writer structure"]
impl crate::Writable for Spipf19cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF19C to value 0"]
impl crate::Resettable for Spipf19cSpec {}
