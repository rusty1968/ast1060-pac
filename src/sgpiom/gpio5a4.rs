#[doc = "Register `GPIO5A4` reader"]
pub type R = crate::R<Gpio5a4Spec>;
#[doc = "Register `GPIO5A4` writer"]
pub type W = crate::W<Gpio5a4Spec>;
#[doc = "Field `PortSerialGPIOM70INTStsReg` reader - Port Serial GPIOM\\[7:0\\] interrupt status register"]
pub type PortSerialGpiom70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOM70INTStsReg` writer - Port Serial GPIOM\\[7:0\\] interrupt status register"]
pub type PortSerialGpiom70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPION70INTStsReg` reader - Port Serial GPION\\[7:0\\] interrupt status register"]
pub type PortSerialGpion70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPION70INTStsReg` writer - Port Serial GPION\\[7:0\\] interrupt status register"]
pub type PortSerialGpion70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOO70INTStsReg` reader - Port Serial GPIOO\\[7:0\\] interrupt status register"]
pub type PortSerialGpioo70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOO70INTStsReg` writer - Port Serial GPIOO\\[7:0\\] interrupt status register"]
pub type PortSerialGpioo70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOP70INTStsReg` reader - Port Serial GPIOP\\[7:0\\] interrupt status register"]
pub type PortSerialGpiop70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOP70INTStsReg` writer - Port Serial GPIOP\\[7:0\\] interrupt status register"]
pub type PortSerialGpiop70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiom70intsts_reg(&self) -> PortSerialGpiom70intstsRegR {
        PortSerialGpiom70intstsRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpion70intsts_reg(&self) -> PortSerialGpion70intstsRegR {
        PortSerialGpion70intstsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioo70intsts_reg(&self) -> PortSerialGpioo70intstsRegR {
        PortSerialGpioo70intstsRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiop70intsts_reg(&self) -> PortSerialGpiop70intstsRegR {
        PortSerialGpiop70intstsRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiom70intsts_reg(&mut self) -> PortSerialGpiom70intstsRegW<Gpio5a4Spec> {
        PortSerialGpiom70intstsRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpion70intsts_reg(&mut self) -> PortSerialGpion70intstsRegW<Gpio5a4Spec> {
        PortSerialGpion70intstsRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioo70intsts_reg(&mut self) -> PortSerialGpioo70intstsRegW<Gpio5a4Spec> {
        PortSerialGpioo70intstsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiop70intsts_reg(&mut self) -> PortSerialGpiop70intstsRegW<Gpio5a4Spec> {
        PortSerialGpiop70intstsRegW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio5a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio5a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio5a4Spec;
impl crate::RegisterSpec for Gpio5a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio5a4::R`](R) reader structure"]
impl crate::Readable for Gpio5a4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio5a4::W`](W) writer structure"]
impl crate::Writable for Gpio5a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO5A4 to value 0"]
impl crate::Resettable for Gpio5a4Spec {}
