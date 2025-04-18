#[doc = "Register `GPIO00C` reader"]
pub type R = crate::R<Gpio00cSpec>;
#[doc = "Register `GPIO00C` writer"]
pub type W = crate::W<Gpio00cSpec>;
#[doc = "Field `PortGPIOA70INTSensitivityType0Sel` reader - Port GPIOA\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioa70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOA70INTSensitivityType0Sel` writer - Port GPIOA\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioa70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70INTSensitivityType0Sel` reader - Port GPIOB\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiob70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOB70INTSensitivityType0Sel` writer - Port GPIOB\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiob70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70INTSensitivityType0Sel` reader - Port GPIOC\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioc70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOC70INTSensitivityType0Sel` writer - Port GPIOC\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioc70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70INTSensitivityType0Sel` reader - Port GPIOD\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiod70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOD70INTSensitivityType0Sel` writer - Port GPIOD\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiod70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioa70intsensitivity_type0sel(&self) -> PortGpioa70intsensitivityType0selR {
        PortGpioa70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiob70intsensitivity_type0sel(&self) -> PortGpiob70intsensitivityType0selR {
        PortGpiob70intsensitivityType0selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioc70intsensitivity_type0sel(&self) -> PortGpioc70intsensitivityType0selR {
        PortGpioc70intsensitivityType0selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiod70intsensitivity_type0sel(&self) -> PortGpiod70intsensitivityType0selR {
        PortGpiod70intsensitivityType0selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioa70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpioa70intsensitivityType0selW<Gpio00cSpec> {
        PortGpioa70intsensitivityType0selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiob70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpiob70intsensitivityType0selW<Gpio00cSpec> {
        PortGpiob70intsensitivityType0selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioc70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpioc70intsensitivityType0selW<Gpio00cSpec> {
        PortGpioc70intsensitivityType0selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiod70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpiod70intsensitivityType0selW<Gpio00cSpec> {
        PortGpiod70intsensitivityType0selW::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio00cSpec;
impl crate::RegisterSpec for Gpio00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio00c::R`](R) reader structure"]
impl crate::Readable for Gpio00cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio00c::W`](W) writer structure"]
impl crate::Writable for Gpio00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO00C to value 0"]
impl crate::Resettable for Gpio00cSpec {}
