#[doc = "Register `GPIO030` reader"]
pub type R = crate::R<Gpio030Spec>;
#[doc = "Register `GPIO030` writer"]
pub type W = crate::W<Gpio030Spec>;
#[doc = "Field `PortGPIOE70INTSensitivityType1Sel` reader - Port GPIOE\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioe70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOE70INTSensitivityType1Sel` writer - Port GPIOE\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioe70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70INTSensitivityType1Sel` reader - Port GPIOF\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiof70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOF70INTSensitivityType1Sel` writer - Port GPIOF\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiof70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70INTSensitivityType1Sel` reader - Port GPIOG\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiog70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOG70INTSensitivityType1Sel` writer - Port GPIOG\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiog70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70INTSensitivityType1Sel` reader - Port GPIOH\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioh70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIOH70INTSensitivityType1Sel` writer - Port GPIOH\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpioh70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioe70intsensitivity_type1sel(&self) -> PortGpioe70intsensitivityType1selR {
        PortGpioe70intsensitivityType1selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiof70intsensitivity_type1sel(&self) -> PortGpiof70intsensitivityType1selR {
        PortGpiof70intsensitivityType1selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiog70intsensitivity_type1sel(&self) -> PortGpiog70intsensitivityType1selR {
        PortGpiog70intsensitivityType1selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioh70intsensitivity_type1sel(&self) -> PortGpioh70intsensitivityType1selR {
        PortGpioh70intsensitivityType1selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioe70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpioe70intsensitivityType1selW<Gpio030Spec> {
        PortGpioe70intsensitivityType1selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiof70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpiof70intsensitivityType1selW<Gpio030Spec> {
        PortGpiof70intsensitivityType1selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiog70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpiog70intsensitivityType1selW<Gpio030Spec> {
        PortGpiog70intsensitivityType1selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpioh70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpioh70intsensitivityType1selW<Gpio030Spec> {
        PortGpioh70intsensitivityType1selW::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio030Spec;
impl crate::RegisterSpec for Gpio030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio030::R`](R) reader structure"]
impl crate::Readable for Gpio030Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio030::W`](W) writer structure"]
impl crate::Writable for Gpio030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO030 to value 0"]
impl crate::Resettable for Gpio030Spec {}
