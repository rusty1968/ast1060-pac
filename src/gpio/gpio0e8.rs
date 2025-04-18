#[doc = "Register `GPIO0E8` reader"]
pub type R = crate::R<Gpio0e8Spec>;
#[doc = "Register `GPIO0E8` writer"]
pub type W = crate::W<Gpio0e8Spec>;
#[doc = "Field `PortGPIOM70INTEnbl` reader - Port GPIOM\\[7:0\\] interrupt enable"]
pub type PortGpiom70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOM70INTEnbl` writer - Port GPIOM\\[7:0\\] interrupt enable"]
pub type PortGpiom70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70INTEnbl` reader - Port GPION\\[7:0\\] interrupt enable"]
pub type PortGpion70intenblR = crate::FieldReader;
#[doc = "Field `PortGPION70INTEnbl` writer - Port GPION\\[7:0\\] interrupt enable"]
pub type PortGpion70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70INTEnbl` reader - Port GPIOO\\[7:0\\] interrupt enable"]
pub type PortGpioo70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOO70INTEnbl` writer - Port GPIOO\\[7:0\\] interrupt enable"]
pub type PortGpioo70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70INTEnbl` reader - Port GPIOP\\[7:0\\] interrupt enable"]
pub type PortGpiop70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOP70INTEnbl` writer - Port GPIOP\\[7:0\\] interrupt enable"]
pub type PortGpiop70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiom70intenbl(&self) -> PortGpiom70intenblR {
        PortGpiom70intenblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpion70intenbl(&self) -> PortGpion70intenblR {
        PortGpion70intenblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioo70intenbl(&self) -> PortGpioo70intenblR {
        PortGpioo70intenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiop70intenbl(&self) -> PortGpiop70intenblR {
        PortGpiop70intenblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiom70intenbl(&mut self) -> PortGpiom70intenblW<Gpio0e8Spec> {
        PortGpiom70intenblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpion70intenbl(&mut self) -> PortGpion70intenblW<Gpio0e8Spec> {
        PortGpion70intenblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioo70intenbl(&mut self) -> PortGpioo70intenblW<Gpio0e8Spec> {
        PortGpioo70intenblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiop70intenbl(&mut self) -> PortGpiop70intenblW<Gpio0e8Spec> {
        PortGpiop70intenblW::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0e8Spec;
impl crate::RegisterSpec for Gpio0e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0e8::R`](R) reader structure"]
impl crate::Readable for Gpio0e8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0e8::W`](W) writer structure"]
impl crate::Writable for Gpio0e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0E8 to value 0"]
impl crate::Resettable for Gpio0e8Spec {}
