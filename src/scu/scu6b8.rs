#[doc = "Register `SCU6B8` reader"]
pub type R = crate::R<Scu6b8Spec>;
#[doc = "Register `SCU6B8` writer"]
pub type W = crate::W<Scu6b8Spec>;
impl W {}
#[doc = "Multi-function Pin Control \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu6b8Spec;
impl crate::RegisterSpec for Scu6b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu6b8::R`](R) reader structure"]
impl crate::Readable for Scu6b8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu6b8::W`](W) writer structure"]
impl crate::Writable for Scu6b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU6B8 to value 0"]
impl crate::Resettable for Scu6b8Spec {}
