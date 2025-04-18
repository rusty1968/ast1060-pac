#[doc = "Register `GPIO110` reader"]
pub type R = crate::R<Gpio110Spec>;
#[doc = "Register `GPIO110` writer"]
pub type W = crate::W<Gpio110Spec>;
#[doc = "Field `PortGPIOQ70CmdSource0` reader - Port GPIOQ\\[7:0\\] Command Source 0"]
pub type PortGpioq70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOQ70CmdSource0` writer - Port GPIOQ\\[7:0\\] Command Source 0"]
pub type PortGpioq70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPIOR70CmdSource0` reader - Port GPIOR\\[7:0\\] Command Source 0"]
pub type PortGpior70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOR70CmdSource0` writer - Port GPIOR\\[7:0\\] Command Source 0"]
pub type PortGpior70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOS70CmdSource0` reader - Port GPIOS\\[7:0\\] Command Source 0"]
pub type PortGpios70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOS70CmdSource0` writer - Port GPIOS\\[7:0\\] Command Source 0"]
pub type PortGpios70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIT70CmdSource0` reader - Port GPIT\\[7:0\\] Command Source 0"]
pub type PortGpit70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIT70CmdSource0` writer - Port GPIT\\[7:0\\] Command Source 0"]
pub type PortGpit70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOQ\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioq70cmd_source0(&self) -> PortGpioq70cmdSource0R {
        PortGpioq70cmdSource0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPIOR\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpior70cmd_source0(&self) -> PortGpior70cmdSource0R {
        PortGpior70cmdSource0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOS\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpios70cmd_source0(&self) -> PortGpios70cmdSource0R {
        PortGpios70cmdSource0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIT\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpit70cmd_source0(&self) -> PortGpit70cmdSource0R {
        PortGpit70cmdSource0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOQ\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioq70cmd_source0(&mut self) -> PortGpioq70cmdSource0W<Gpio110Spec> {
        PortGpioq70cmdSource0W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPIOR\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpior70cmd_source0(&mut self) -> PortGpior70cmdSource0W<Gpio110Spec> {
        PortGpior70cmdSource0W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOS\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpios70cmd_source0(&mut self) -> PortGpios70cmdSource0W<Gpio110Spec> {
        PortGpios70cmdSource0W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIT\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpit70cmd_source0(&mut self) -> PortGpit70cmdSource0W<Gpio110Spec> {
        PortGpit70cmdSource0W::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio110Spec;
impl crate::RegisterSpec for Gpio110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio110::R`](R) reader structure"]
impl crate::Readable for Gpio110Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio110::W`](W) writer structure"]
impl crate::Writable for Gpio110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO110 to value 0"]
impl crate::Resettable for Gpio110Spec {}
