#[doc = "Register `GPIO5A0` reader"]
pub type R = crate::R<Gpio5a0Spec>;
#[doc = "Register `GPIO5A0` writer"]
pub type W = crate::W<Gpio5a0Spec>;
#[doc = "Field `PortSerialGPIOM70INTSensitivityType2Sel` reader - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiom70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOM70INTSensitivityType2Sel` writer - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiom70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPION70INTSensitivityType2Sel` reader - Port Serial GPION\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpion70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPION70INTSensitivityType2Sel` writer - Port Serial GPION\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpion70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOO70INTSensitivityType2Sel` reader - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioo70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOO70INTSensitivityType2Sel` writer - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpioo70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOP70INTSensitivityType2Sel` reader - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiop70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOP70INTSensitivityType2Sel` writer - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortSerialGpiop70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiom70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpiom70intsensitivityType2selR {
        PortSerialGpiom70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpion70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpion70intsensitivityType2selR {
        PortSerialGpion70intsensitivityType2selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioo70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpioo70intsensitivityType2selR {
        PortSerialGpioo70intsensitivityType2selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiop70intsensitivity_type2sel(
        &self,
    ) -> PortSerialGpiop70intsensitivityType2selR {
        PortSerialGpiop70intsensitivityType2selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiom70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpiom70intsensitivityType2selW<Gpio5a0Spec> {
        PortSerialGpiom70intsensitivityType2selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpion70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpion70intsensitivityType2selW<Gpio5a0Spec> {
        PortSerialGpion70intsensitivityType2selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpioo70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpioo70intsensitivityType2selW<Gpio5a0Spec> {
        PortSerialGpioo70intsensitivityType2selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_serial_gpiop70intsensitivity_type2sel(
        &mut self,
    ) -> PortSerialGpiop70intsensitivityType2selW<Gpio5a0Spec> {
        PortSerialGpiop70intsensitivityType2selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio5a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio5a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio5a0Spec;
impl crate::RegisterSpec for Gpio5a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio5a0::R`](R) reader structure"]
impl crate::Readable for Gpio5a0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio5a0::W`](W) writer structure"]
impl crate::Writable for Gpio5a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO5A0 to value 0"]
impl crate::Resettable for Gpio5a0Spec {}
