#[doc = "Register `GPIO02C` reader"]
pub type R = crate::R<Gpio02cSpec>;
#[doc = "Register `GPIO02C` writer"]
pub type W = crate::W<Gpio02cSpec>;
#[doc = "Field `PortGPIOE70INTSensitivityType0Sel` reader - Port GPIOE\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioe70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOE70INTSensitivityType0Sel` writer - Port GPIOE\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioe70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70INTSensitivityType0Sel` reader - Port GPIOF\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiof70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOF70INTSensitivityType0Sel` writer - Port GPIOF\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiof70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70INTSensitivityType0Sel` reader - Port GPIOG\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiog70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOG70INTSensitivityType0Sel` writer - Port GPIOG\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiog70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70INTSensitivityType0Sel` reader - Port GPIOH\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioh70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIOH70INTSensitivityType0Sel` writer - Port GPIOH\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpioh70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioe70intsensitivity_type0sel(&self) -> PortGpioe70intsensitivityType0selR {
        PortGpioe70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiof70intsensitivity_type0sel(&self) -> PortGpiof70intsensitivityType0selR {
        PortGpiof70intsensitivityType0selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiog70intsensitivity_type0sel(&self) -> PortGpiog70intsensitivityType0selR {
        PortGpiog70intsensitivityType0selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioh70intsensitivity_type0sel(&self) -> PortGpioh70intsensitivityType0selR {
        PortGpioh70intsensitivityType0selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioe70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpioe70intsensitivityType0selW<Gpio02cSpec> {
        PortGpioe70intsensitivityType0selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiof70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpiof70intsensitivityType0selW<Gpio02cSpec> {
        PortGpiof70intsensitivityType0selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiog70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpiog70intsensitivityType0selW<Gpio02cSpec> {
        PortGpiog70intsensitivityType0selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpioh70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpioh70intsensitivityType0selW<Gpio02cSpec> {
        PortGpioh70intsensitivityType0selW::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio02cSpec;
impl crate::RegisterSpec for Gpio02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio02c::R`](R) reader structure"]
impl crate::Readable for Gpio02cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio02c::W`](W) writer structure"]
impl crate::Writable for Gpio02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO02C to value 0"]
impl crate::Resettable for Gpio02cSpec {}
