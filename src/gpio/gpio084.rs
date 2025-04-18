#[doc = "Register `GPIO084` reader"]
pub type R = crate::R<Gpio084Spec>;
#[doc = "Register `GPIO084` writer"]
pub type W = crate::W<Gpio084Spec>;
#[doc = "Field `PortGPIOQ70DirionCtrl` reader - Port GPIOQ\\[7:0\\] direction control"]
pub type PortGpioq70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOQ70DirionCtrl` writer - Port GPIOQ\\[7:0\\] direction control"]
pub type PortGpioq70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70DirionCtrl` reader - Port GPIOR\\[7:0\\] direction control"]
pub type PortGpior70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOR70DirionCtrl` writer - Port GPIOR\\[7:0\\] direction control"]
pub type PortGpior70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70DirionCtrl` reader - Port GPIOS\\[7:0\\] direction control"]
pub type PortGpios70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOS70DirionCtrl` writer - Port GPIOS\\[7:0\\] direction control"]
pub type PortGpios70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioq70dirion_ctrl(&self) -> PortGpioq70dirionCtrlR {
        PortGpioq70dirionCtrlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpior70dirion_ctrl(&self) -> PortGpior70dirionCtrlR {
        PortGpior70dirionCtrlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpios70dirion_ctrl(&self) -> PortGpios70dirionCtrlR {
        PortGpios70dirionCtrlR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioq70dirion_ctrl(&mut self) -> PortGpioq70dirionCtrlW<Gpio084Spec> {
        PortGpioq70dirionCtrlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpior70dirion_ctrl(&mut self) -> PortGpior70dirionCtrlW<Gpio084Spec> {
        PortGpior70dirionCtrlW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpios70dirion_ctrl(&mut self) -> PortGpios70dirionCtrlW<Gpio084Spec> {
        PortGpios70dirionCtrlW::new(self, 16)
    }
}
#[doc = "GPIO\\_Q/R/S/T Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio084Spec;
impl crate::RegisterSpec for Gpio084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio084::R`](R) reader structure"]
impl crate::Readable for Gpio084Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio084::W`](W) writer structure"]
impl crate::Writable for Gpio084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO084 to value 0"]
impl crate::Resettable for Gpio084Spec {}
