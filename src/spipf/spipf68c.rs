#[doc = "Register `SPIPF68C` reader"]
pub type R = crate::R<Spipf68cSpec>;
#[doc = "Register `SPIPF68C` writer"]
pub type W = crate::W<Spipf68cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf68c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf68c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf68cSpec;
impl crate::RegisterSpec for Spipf68cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf68c::R`](R) reader structure"]
impl crate::Readable for Spipf68cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf68c::W`](W) writer structure"]
impl crate::Writable for Spipf68cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF68C to value 0"]
impl crate::Resettable for Spipf68cSpec {}
