#[doc = "Register `GPIO558` reader"]
pub type R = crate::R<Gpio558Spec>;
#[doc = "Register `GPIO558` writer"]
pub type W = crate::W<Gpio558Spec>;
#[doc = "Field `PortGPIOA70InputMask` reader - Port GPIOA\\[7:0\\] input mask"]
pub type PortGpioa70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOA70InputMask` writer - Port GPIOA\\[7:0\\] input mask"]
pub type PortGpioa70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70InputMask` reader - Port GPIOB\\[7:0\\] input mask"]
pub type PortGpiob70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOB70InputMask` writer - Port GPIOB\\[7:0\\] input mask"]
pub type PortGpiob70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70InputMask` reader - Port GPIOC\\[7:0\\] input mask"]
pub type PortGpioc70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOC70InputMask` writer - Port GPIOC\\[7:0\\] input mask"]
pub type PortGpioc70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70InputMask` reader - Port GPIOD\\[7:0\\] input mask"]
pub type PortGpiod70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOD70InputMask` writer - Port GPIOD\\[7:0\\] input mask"]
pub type PortGpiod70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioa70input_mask(&self) -> PortGpioa70inputMaskR {
        PortGpioa70inputMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiob70input_mask(&self) -> PortGpiob70inputMaskR {
        PortGpiob70inputMaskR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioc70input_mask(&self) -> PortGpioc70inputMaskR {
        PortGpioc70inputMaskR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiod70input_mask(&self) -> PortGpiod70inputMaskR {
        PortGpiod70inputMaskR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioa70input_mask(&mut self) -> PortGpioa70inputMaskW<Gpio558Spec> {
        PortGpioa70inputMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiob70input_mask(&mut self) -> PortGpiob70inputMaskW<Gpio558Spec> {
        PortGpiob70inputMaskW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioc70input_mask(&mut self) -> PortGpioc70inputMaskW<Gpio558Spec> {
        PortGpioc70inputMaskW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiod70input_mask(&mut self) -> PortGpiod70inputMaskW<Gpio558Spec> {
        PortGpiod70inputMaskW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_A/B/C/D Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio558::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio558::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio558Spec;
impl crate::RegisterSpec for Gpio558Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio558::R`](R) reader structure"]
impl crate::Readable for Gpio558Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio558::W`](W) writer structure"]
impl crate::Writable for Gpio558Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO558 to value 0"]
impl crate::Resettable for Gpio558Spec {}
