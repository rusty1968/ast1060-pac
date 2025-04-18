#[doc = "Register `GPIO014` reader"]
pub type R = crate::R<Gpio014Spec>;
#[doc = "Register `GPIO014` writer"]
pub type W = crate::W<Gpio014Spec>;
#[doc = "Field `PortGPIOA70INTSensitivityType2Sel` reader - Port GPIOA\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioa70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOA70INTSensitivityType2Sel` writer - Port GPIOA\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioa70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70INTSensitivityType2Sel` reader - Port GPIOB\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiob70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOB70INTSensitivityType2Sel` writer - Port GPIOB\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiob70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70INTSensitivityType2Sel` reader - Port GPIOC\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioc70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOC70INTSensitivityType2Sel` writer - Port GPIOC\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpioc70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70INTSensitivityType2Sel` reader - Port GPIOD\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiod70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIOD70INTSensitivityType2Sel` writer - Port GPIOD\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiod70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioa70intsensitivity_type2sel(&self) -> PortGpioa70intsensitivityType2selR {
        PortGpioa70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiob70intsensitivity_type2sel(&self) -> PortGpiob70intsensitivityType2selR {
        PortGpiob70intsensitivityType2selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioc70intsensitivity_type2sel(&self) -> PortGpioc70intsensitivityType2selR {
        PortGpioc70intsensitivityType2selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiod70intsensitivity_type2sel(&self) -> PortGpiod70intsensitivityType2selR {
        PortGpiod70intsensitivityType2selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioa70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpioa70intsensitivityType2selW<Gpio014Spec> {
        PortGpioa70intsensitivityType2selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiob70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpiob70intsensitivityType2selW<Gpio014Spec> {
        PortGpiob70intsensitivityType2selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpioc70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpioc70intsensitivityType2selW<Gpio014Spec> {
        PortGpioc70intsensitivityType2selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiod70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpiod70intsensitivityType2selW<Gpio014Spec> {
        PortGpiod70intsensitivityType2selW::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio014Spec;
impl crate::RegisterSpec for Gpio014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio014::R`](R) reader structure"]
impl crate::Readable for Gpio014Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio014::W`](W) writer structure"]
impl crate::Writable for Gpio014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO014 to value 0"]
impl crate::Resettable for Gpio014Spec {}
