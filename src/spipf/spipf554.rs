#[doc = "Register `SPIPF554` reader"]
pub type R = crate::R<Spipf554Spec>;
#[doc = "Register `SPIPF554` writer"]
pub type W = crate::W<Spipf554Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf554::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf554::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf554Spec;
impl crate::RegisterSpec for Spipf554Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf554::R`](R) reader structure"]
impl crate::Readable for Spipf554Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf554::W`](W) writer structure"]
impl crate::Writable for Spipf554Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF554 to value 0"]
impl crate::Resettable for Spipf554Spec {}
