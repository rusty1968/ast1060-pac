#[doc = "Register `I2CFILTERTHR20` reader"]
pub type R = crate::R<I2cfilterthr20Spec>;
#[doc = "Register `I2CFILTERTHR20` writer"]
pub type W = crate::W<I2cfilterthr20Spec>;
#[doc = "Field `STATUS` reader - STATUS"]
pub type StatusR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - STATUS"]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(self.bits)
    }
}
impl W {}
#[doc = "I2CFLT\\_THRN\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr20Spec;
impl crate::RegisterSpec for I2cfilterthr20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr20::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr20Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr20::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR20 to value 0"]
impl crate::Resettable for I2cfilterthr20Spec {}
