#[doc = "Register `I2CFILTERTHR60` reader"]
pub type R = crate::R<I2cfilterthr60Spec>;
#[doc = "Register `I2CFILTERTHR60` writer"]
pub type W = crate::W<I2cfilterthr60Spec>;
#[doc = "Field `INFO` reader - INFO"]
pub type InfoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - INFO"]
    #[inline(always)]
    pub fn info(&self) -> InfoR {
        InfoR::new(self.bits)
    }
}
impl W {}
#[doc = "I2CFLT\\_THRN\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cfilterthr60Spec;
impl crate::RegisterSpec for I2cfilterthr60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cfilterthr60::R`](R) reader structure"]
impl crate::Readable for I2cfilterthr60Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cfilterthr60::W`](W) writer structure"]
impl crate::Writable for I2cfilterthr60Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CFILTERTHR60 to value 0"]
impl crate::Resettable for I2cfilterthr60Spec {}
