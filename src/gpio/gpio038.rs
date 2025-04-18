#[doc = "Register `GPIO038` reader"]
pub type R = crate::R<Gpio038Spec>;
#[doc = "Register `GPIO038` writer"]
pub type W = crate::W<Gpio038Spec>;
#[doc = "Field `PortGPIOE70INTStsReg` reader - Port GPIOE\\[7:0\\] interrupt status register"]
pub type PortGpioe70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOE70INTStsReg` writer - Port GPIOE\\[7:0\\] interrupt status register"]
pub type PortGpioe70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70INTStsReg` reader - Port GPIOF\\[7:0\\] interrupt status register"]
pub type PortGpiof70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOF70INTStsReg` writer - Port GPIOF\\[7:0\\] interrupt status register"]
pub type PortGpiof70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70INTStsReg` reader - Port GPIOG\\[7:0\\] interrupt status register"]
pub type PortGpiog70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOG70INTStsReg` writer - Port GPIOG\\[7:0\\] interrupt status register"]
pub type PortGpiog70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70INTStsReg` reader - Port GPIOH\\[7:0\\] interrupt status register"]
pub type PortGpioh70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIOH70INTStsReg` writer - Port GPIOH\\[7:0\\] interrupt status register"]
pub type PortGpioh70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioe70intsts_reg(&self) -> PortGpioe70intstsRegR {
        PortGpioe70intstsRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiof70intsts_reg(&self) -> PortGpiof70intstsRegR {
        PortGpiof70intstsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiog70intsts_reg(&self) -> PortGpiog70intstsRegR {
        PortGpiog70intstsRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioh70intsts_reg(&self) -> PortGpioh70intstsRegR {
        PortGpioh70intstsRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioe70intsts_reg(&mut self) -> PortGpioe70intstsRegW<Gpio038Spec> {
        PortGpioe70intstsRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiof70intsts_reg(&mut self) -> PortGpiof70intstsRegW<Gpio038Spec> {
        PortGpiof70intstsRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiog70intsts_reg(&mut self) -> PortGpiog70intstsRegW<Gpio038Spec> {
        PortGpiog70intstsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpioh70intsts_reg(&mut self) -> PortGpioh70intstsRegW<Gpio038Spec> {
        PortGpioh70intstsRegW::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio038Spec;
impl crate::RegisterSpec for Gpio038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio038::R`](R) reader structure"]
impl crate::Readable for Gpio038Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio038::W`](W) writer structure"]
impl crate::Writable for Gpio038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO038 to value 0"]
impl crate::Resettable for Gpio038Spec {}
