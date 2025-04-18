#[doc = "Register `SECURE094` reader"]
pub type R = crate::R<Secure094Spec>;
#[doc = "Register `SECURE094` writer"]
pub type W = crate::W<Secure094Spec>;
#[doc = "Field `Reserved01` reader - Reserved(0)"]
pub type Reserved01R = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Secure Boot Counter 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure094Spec;
impl crate::RegisterSpec for Secure094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure094::R`](R) reader structure"]
impl crate::Readable for Secure094Spec {}
#[doc = "`write(|w| ..)` method takes [`secure094::W`](W) writer structure"]
impl crate::Writable for Secure094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE094 to value 0"]
impl crate::Resettable for Secure094Spec {}
