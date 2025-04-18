#[doc = "Register `GPIO500` reader"]
pub type R = crate::R<Gpio500Spec>;
#[doc = "Register `GPIO500` writer"]
pub type W = crate::W<Gpio500Spec>;
#[doc = "Field `PortSerialGPIOA70DataReg` reader - Port Serial GPIOA\\[7:0\\] data register"]
pub type PortSerialGpioa70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOA70DataReg` writer - Port Serial GPIOA\\[7:0\\] data register"]
pub type PortSerialGpioa70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOB70DataReg` reader - Port Serial GPIOB\\[7:0\\] data register"]
pub type PortSerialGpiob70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOB70DataReg` writer - Port Serial GPIOB\\[7:0\\] data register"]
pub type PortSerialGpiob70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOC70DataReg` reader - Port Serial GPIOC\\[7:0\\] data register"]
pub type PortSerialGpioc70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOC70DataReg` writer - Port Serial GPIOC\\[7:0\\] data register"]
pub type PortSerialGpioc70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOD70DataReg` reader - Port Serial GPIOD\\[7:0\\] data register"]
pub type PortSerialGpiod70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOD70DataReg` writer - Port Serial GPIOD\\[7:0\\] data register"]
pub type PortSerialGpiod70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioa70data_reg(&self) -> PortSerialGpioa70dataRegR {
        PortSerialGpioa70dataRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiob70data_reg(&self) -> PortSerialGpiob70dataRegR {
        PortSerialGpiob70dataRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioc70data_reg(&self) -> PortSerialGpioc70dataRegR {
        PortSerialGpioc70dataRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiod70data_reg(&self) -> PortSerialGpiod70dataRegR {
        PortSerialGpiod70dataRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioa70data_reg(&mut self) -> PortSerialGpioa70dataRegW<Gpio500Spec> {
        PortSerialGpioa70dataRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiob70data_reg(&mut self) -> PortSerialGpiob70dataRegW<Gpio500Spec> {
        PortSerialGpiob70dataRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioc70data_reg(&mut self) -> PortSerialGpioc70dataRegW<Gpio500Spec> {
        PortSerialGpioc70dataRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiod70data_reg(&mut self) -> PortSerialGpiod70dataRegW<Gpio500Spec> {
        PortSerialGpiod70dataRegW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_A/B/C/D 1 Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio500::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio500::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio500Spec;
impl crate::RegisterSpec for Gpio500Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio500::R`](R) reader structure"]
impl crate::Readable for Gpio500Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio500::W`](W) writer structure"]
impl crate::Writable for Gpio500Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO500 to value 0"]
impl crate::Resettable for Gpio500Spec {}
