#[doc = "Register `GPIO0A4` reader"]
pub type R = crate::R<Gpio0a4Spec>;
#[doc = "Register `GPIO0A4` writer"]
pub type W = crate::W<Gpio0a4Spec>;
#[doc = "Field `PortGPIOI70INTSensitivityType2Sel` reader - Port GPIOI\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioi70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOI70INTSensitivityType2Sel` writer - Port GPIOI\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioi70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70INTSensitivityType2Sel` reader - Port GPIOJ\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioj70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOJ70INTSensitivityType2Sel` writer - Port GPIOJ\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioj70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70INTSensitivityType2Sel` reader - Port GPIOK\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiok70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOK70INTSensitivityType2Sel` writer - Port GPIOK\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiok70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70INTSensitivityType2Sel` reader - Port GPIOL\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiol70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOL70INTSensitivityType2Sel` writer - Port GPIOL\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiol70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioi70intsensitivity_type2sel(&self) -> PortGpioi70intsensitivityType2selR {
        PortGpioi70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioj70intsensitivity_type2sel(&self) -> PortGpioj70intsensitivityType2selR {
        PortGpioj70intsensitivityType2selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiok70intsensitivity_type2sel(&self) -> PortGpiok70intsensitivityType2selR {
        PortGpiok70intsensitivityType2selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiol70intsensitivity_type2sel(&self) -> PortGpiol70intsensitivityType2selR {
        PortGpiol70intsensitivityType2selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioi70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpioi70intsensitivityType2selW<Gpio0a4Spec> {
        PortGpioi70intsensitivityType2selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioj70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpioj70intsensitivityType2selW<Gpio0a4Spec> {
        PortGpioj70intsensitivityType2selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiok70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpiok70intsensitivityType2selW<Gpio0a4Spec> {
        PortGpiok70intsensitivityType2selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiol70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpiol70intsensitivityType2selW<Gpio0a4Spec> {
        PortGpiol70intsensitivityType2selW::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0a4Spec;
impl crate::RegisterSpec for Gpio0a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0a4::R`](R) reader structure"]
impl crate::Readable for Gpio0a4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0a4::W`](W) writer structure"]
impl crate::Writable for Gpio0a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0A4 to value 0"]
impl crate::Resettable for Gpio0a4Spec {}
