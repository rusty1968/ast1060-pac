#[doc = "Register `GPIO53C` reader"]
pub type R = crate::R<Gpio53cSpec>;
#[doc = "Register `GPIO53C` writer"]
pub type W = crate::W<Gpio53cSpec>;
#[doc = "Field `PortSerialGPIOI70INTEnbl` reader - Port Serial GPIOI\\[7:0\\] interrupt enable"]
pub type PortSerialGpioi70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOI70INTEnbl` writer - Port Serial GPIOI\\[7:0\\] interrupt enable"]
pub type PortSerialGpioi70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOJ70INTEnbl` reader - Port Serial GPIOJ\\[7:0\\] interrupt enable"]
pub type PortSerialGpioj70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOJ70INTEnbl` writer - Port Serial GPIOJ\\[7:0\\] interrupt enable"]
pub type PortSerialGpioj70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOK70INTEnbl` reader - Port Serial GPIOK\\[7:0\\] interrupt enable"]
pub type PortSerialGpiok70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOK70INTEnbl` writer - Port Serial GPIOK\\[7:0\\] interrupt enable"]
pub type PortSerialGpiok70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOL70INTEnbl` reader - Port Serial GPIOL\\[7:0\\] interrupt enable"]
pub type PortSerialGpiol70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOL70INTEnbl` writer - Port Serial GPIOL\\[7:0\\] interrupt enable"]
pub type PortSerialGpiol70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioi70intenbl(&self) -> PortSerialGpioi70intenblR {
        PortSerialGpioi70intenblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioj70intenbl(&self) -> PortSerialGpioj70intenblR {
        PortSerialGpioj70intenblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiok70intenbl(&self) -> PortSerialGpiok70intenblR {
        PortSerialGpiok70intenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiol70intenbl(&self) -> PortSerialGpiol70intenblR {
        PortSerialGpiol70intenblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioi70intenbl(&mut self) -> PortSerialGpioi70intenblW<Gpio53cSpec> {
        PortSerialGpioi70intenblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioj70intenbl(&mut self) -> PortSerialGpioj70intenblW<Gpio53cSpec> {
        PortSerialGpioj70intenblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiok70intenbl(&mut self) -> PortSerialGpiok70intenblW<Gpio53cSpec> {
        PortSerialGpiok70intenblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiol70intenbl(&mut self) -> PortSerialGpiol70intenblW<Gpio53cSpec> {
        PortSerialGpiol70intenblW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_I/J/K/L 1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio53c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio53c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio53cSpec;
impl crate::RegisterSpec for Gpio53cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio53c::R`](R) reader structure"]
impl crate::Readable for Gpio53cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio53c::W`](W) writer structure"]
impl crate::Writable for Gpio53cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO53C to value 0"]
impl crate::Resettable for Gpio53cSpec {}
