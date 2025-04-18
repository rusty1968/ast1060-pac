#[doc = "Register `SCU414` reader"]
pub type R = crate::R<Scu414Spec>;
#[doc = "Register `SCU414` writer"]
pub type W = crate::W<Scu414Spec>;
#[doc = "Field `EnblSCL1FnPin` reader - Enable SCL1 function pin"]
pub type EnblScl1fnPinR = crate::BitReader;
#[doc = "Field `EnblSCL1FnPin` writer - Enable SCL1 function pin"]
pub type EnblScl1fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSDA1FnPin` reader - Enable SDA1 function pin"]
pub type EnblSda1fnPinR = crate::BitReader;
#[doc = "Field `EnblSDA1FnPin` writer - Enable SDA1 function pin"]
pub type EnblSda1fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSCL2FnPin` reader - Enable SCL2 function pin"]
pub type EnblScl2fnPinR = crate::BitReader;
#[doc = "Field `EnblSCL2FnPin` writer - Enable SCL2 function pin"]
pub type EnblScl2fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSDA2FnPin` reader - Enable SDA2 function pin"]
pub type EnblSda2fnPinR = crate::BitReader;
#[doc = "Field `EnblSDA2FnPin` writer - Enable SDA2 function pin"]
pub type EnblSda2fnPinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - Enable SCL1 function pin"]
    #[inline(always)]
    pub fn enbl_scl1fn_pin(&self) -> EnblScl1fnPinR {
        EnblScl1fnPinR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable SDA1 function pin"]
    #[inline(always)]
    pub fn enbl_sda1fn_pin(&self) -> EnblSda1fnPinR {
        EnblSda1fnPinR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable SCL2 function pin"]
    #[inline(always)]
    pub fn enbl_scl2fn_pin(&self) -> EnblScl2fnPinR {
        EnblScl2fnPinR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable SDA2 function pin"]
    #[inline(always)]
    pub fn enbl_sda2fn_pin(&self) -> EnblSda2fnPinR {
        EnblSda2fnPinR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Enable SCL1 function pin"]
    #[inline(always)]
    pub fn enbl_scl1fn_pin(&mut self) -> EnblScl1fnPinW<Scu414Spec> {
        EnblScl1fnPinW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable SDA1 function pin"]
    #[inline(always)]
    pub fn enbl_sda1fn_pin(&mut self) -> EnblSda1fnPinW<Scu414Spec> {
        EnblSda1fnPinW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable SCL2 function pin"]
    #[inline(always)]
    pub fn enbl_scl2fn_pin(&mut self) -> EnblScl2fnPinW<Scu414Spec> {
        EnblScl2fnPinW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable SDA2 function pin"]
    #[inline(always)]
    pub fn enbl_sda2fn_pin(&mut self) -> EnblSda2fnPinW<Scu414Spec> {
        EnblSda2fnPinW::new(self, 31)
    }
}
#[doc = "Multi-function Pin Control \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu414::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu414::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu414Spec;
impl crate::RegisterSpec for Scu414Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu414::R`](R) reader structure"]
impl crate::Readable for Scu414Spec {}
#[doc = "`write(|w| ..)` method takes [`scu414::W`](W) writer structure"]
impl crate::Writable for Scu414Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU414 to value 0"]
impl crate::Resettable for Scu414Spec {}
