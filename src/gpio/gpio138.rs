#[doc = "Register `GPIO138` reader"]
pub type R = crate::R<Gpio138Spec>;
#[doc = "Register `GPIO138` writer"]
pub type W = crate::W<Gpio138Spec>;
#[doc = "Field `PortGPIOQ70InputMask` reader - Port GPIOQ\\[7:0\\] input mask"]
pub type PortGpioq70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOQ70InputMask` writer - Port GPIOQ\\[7:0\\] input mask"]
pub type PortGpioq70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70InputMask` reader - Port GPIOR\\[7:0\\] input mask"]
pub type PortGpior70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOR70InputMask` writer - Port GPIOR\\[7:0\\] input mask"]
pub type PortGpior70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70InputMask` reader - Port GPIOS\\[7:0\\] input mask"]
pub type PortGpios70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOS70InputMask` writer - Port GPIOS\\[7:0\\] input mask"]
pub type PortGpios70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIT70InputMask` reader - Port GPIT\\[7:0\\] input mask"]
pub type PortGpit70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIT70InputMask` writer - Port GPIT\\[7:0\\] input mask"]
pub type PortGpit70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioq70input_mask(&self) -> PortGpioq70inputMaskR {
        PortGpioq70inputMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpior70input_mask(&self) -> PortGpior70inputMaskR {
        PortGpior70inputMaskR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpios70input_mask(&self) -> PortGpios70inputMaskR {
        PortGpios70inputMaskR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpit70input_mask(&self) -> PortGpit70inputMaskR {
        PortGpit70inputMaskR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioq70input_mask(&mut self) -> PortGpioq70inputMaskW<Gpio138Spec> {
        PortGpioq70inputMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpior70input_mask(&mut self) -> PortGpior70inputMaskW<Gpio138Spec> {
        PortGpior70inputMaskW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpios70input_mask(&mut self) -> PortGpios70inputMaskW<Gpio138Spec> {
        PortGpios70inputMaskW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpit70input_mask(&mut self) -> PortGpit70inputMaskW<Gpio138Spec> {
        PortGpit70inputMaskW::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio138::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio138::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio138Spec;
impl crate::RegisterSpec for Gpio138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio138::R`](R) reader structure"]
impl crate::Readable for Gpio138Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio138::W`](W) writer structure"]
impl crate::Writable for Gpio138Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO138 to value 0"]
impl crate::Resettable for Gpio138Spec {}
