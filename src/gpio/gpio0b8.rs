#[doc = "Register `GPIO0B8` reader"]
pub type R = crate::R<Gpio0b8Spec>;
#[doc = "Register `GPIO0B8` writer"]
pub type W = crate::W<Gpio0b8Spec>;
#[doc = "Field `PortGPIOI70InputMask` reader - Port GPIOI\\[7:0\\] input mask"]
pub type PortGpioi70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOI70InputMask` writer - Port GPIOI\\[7:0\\] input mask"]
pub type PortGpioi70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70InputMask` reader - Port GPIOJ\\[7:0\\] input mask"]
pub type PortGpioj70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOJ70InputMask` writer - Port GPIOJ\\[7:0\\] input mask"]
pub type PortGpioj70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70InputMask` reader - Port GPIOK\\[7:0\\] input mask"]
pub type PortGpiok70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOK70InputMask` writer - Port GPIOK\\[7:0\\] input mask"]
pub type PortGpiok70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70InputMask` reader - Port GPIOL\\[7:0\\] input mask"]
pub type PortGpiol70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOL70InputMask` writer - Port GPIOL\\[7:0\\] input mask"]
pub type PortGpiol70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioi70input_mask(&self) -> PortGpioi70inputMaskR {
        PortGpioi70inputMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioj70input_mask(&self) -> PortGpioj70inputMaskR {
        PortGpioj70inputMaskR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiok70input_mask(&self) -> PortGpiok70inputMaskR {
        PortGpiok70inputMaskR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiol70input_mask(&self) -> PortGpiol70inputMaskR {
        PortGpiol70inputMaskR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioi70input_mask(&mut self) -> PortGpioi70inputMaskW<Gpio0b8Spec> {
        PortGpioi70inputMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioj70input_mask(&mut self) -> PortGpioj70inputMaskW<Gpio0b8Spec> {
        PortGpioj70inputMaskW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiok70input_mask(&mut self) -> PortGpiok70inputMaskW<Gpio0b8Spec> {
        PortGpiok70inputMaskW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiol70input_mask(&mut self) -> PortGpiol70inputMaskW<Gpio0b8Spec> {
        PortGpiol70inputMaskW::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0b8Spec;
impl crate::RegisterSpec for Gpio0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0b8::R`](R) reader structure"]
impl crate::Readable for Gpio0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0b8::W`](W) writer structure"]
impl crate::Writable for Gpio0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0B8 to value 0"]
impl crate::Resettable for Gpio0b8Spec {}
