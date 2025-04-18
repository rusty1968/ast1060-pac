#[doc = "Register `GPIO598` reader"]
pub type R = crate::R<Gpio598Spec>;
#[doc = "Register `GPIO598` writer"]
pub type W = crate::W<Gpio598Spec>;
#[doc = "Field `PortSerialGPIOM70INTSensitivityType0Sel` reader - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiom70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOM70INTSensitivityType0Sel` writer - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiom70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPION70INTSensitivityType0Sel` reader - Port Serial GPION\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpion70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPION70INTSensitivityType0Sel` writer - Port Serial GPION\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpion70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOO70INTSensitivityType0Sel` reader - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioo70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOO70INTSensitivityType0Sel` writer - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpioo70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOP70INTSensitivityType0Sel` reader - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiop70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOP70INTSensitivityType0Sel` writer - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortSerialGpiop70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiom70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpiom70intsensitivityType0selR {
        PortSerialGpiom70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpion70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpion70intsensitivityType0selR {
        PortSerialGpion70intsensitivityType0selR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioo70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpioo70intsensitivityType0selR {
        PortSerialGpioo70intsensitivityType0selR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiop70intsensitivity_type0sel(
        &self,
    ) -> PortSerialGpiop70intsensitivityType0selR {
        PortSerialGpiop70intsensitivityType0selR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiom70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpiom70intsensitivityType0selW<Gpio598Spec> {
        PortSerialGpiom70intsensitivityType0selW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpion70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpion70intsensitivityType0selW<Gpio598Spec> {
        PortSerialGpion70intsensitivityType0selW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpioo70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpioo70intsensitivityType0selW<Gpio598Spec> {
        PortSerialGpioo70intsensitivityType0selW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_serial_gpiop70intsensitivity_type0sel(
        &mut self,
    ) -> PortSerialGpiop70intsensitivityType0selW<Gpio598Spec> {
        PortSerialGpiop70intsensitivityType0selW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio598::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio598::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio598Spec;
impl crate::RegisterSpec for Gpio598Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio598::R`](R) reader structure"]
impl crate::Readable for Gpio598Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio598::W`](W) writer structure"]
impl crate::Writable for Gpio598Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO598 to value 0"]
impl crate::Resettable for Gpio598Spec {}
