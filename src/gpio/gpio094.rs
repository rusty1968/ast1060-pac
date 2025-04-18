#[doc = "Register `GPIO094` reader"]
pub type R = crate::R<Gpio094Spec>;
#[doc = "Register `GPIO094` writer"]
pub type W = crate::W<Gpio094Spec>;
#[doc = "Field `PortGPIOI70CmdSource1` reader - Port GPIOI\\[7:0\\] Command Source 1"]
pub type PortGpioi70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOI70CmdSource1` writer - Port GPIOI\\[7:0\\] Command Source 1"]
pub type PortGpioi70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPIOJ70CmdSource1` reader - Port GPIOJ\\[7:0\\] Command Source 1"]
pub type PortGpioj70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOJ70CmdSource1` writer - Port GPIOJ\\[7:0\\] Command Source 1"]
pub type PortGpioj70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOK70CmdSource1` reader - Port GPIOK\\[7:0\\] Command Source 1"]
pub type PortGpiok70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOK70CmdSource1` writer - Port GPIOK\\[7:0\\] Command Source 1"]
pub type PortGpiok70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIOL70CmdSource1` reader - Port GPIOL\\[7:0\\] Command Source 1"]
pub type PortGpiol70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOL70CmdSource1` writer - Port GPIOL\\[7:0\\] Command Source 1"]
pub type PortGpiol70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOI\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioi70cmd_source1(&self) -> PortGpioi70cmdSource1R {
        PortGpioi70cmdSource1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPIOJ\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioj70cmd_source1(&self) -> PortGpioj70cmdSource1R {
        PortGpioj70cmdSource1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOK\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiok70cmd_source1(&self) -> PortGpiok70cmdSource1R {
        PortGpiok70cmdSource1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIOL\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiol70cmd_source1(&self) -> PortGpiol70cmdSource1R {
        PortGpiol70cmdSource1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOI\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioi70cmd_source1(&mut self) -> PortGpioi70cmdSource1W<Gpio094Spec> {
        PortGpioi70cmdSource1W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPIOJ\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioj70cmd_source1(&mut self) -> PortGpioj70cmdSource1W<Gpio094Spec> {
        PortGpioj70cmdSource1W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOK\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiok70cmd_source1(&mut self) -> PortGpiok70cmdSource1W<Gpio094Spec> {
        PortGpiok70cmdSource1W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIOL\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiol70cmd_source1(&mut self) -> PortGpiol70cmdSource1W<Gpio094Spec> {
        PortGpiol70cmdSource1W::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio094Spec;
impl crate::RegisterSpec for Gpio094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio094::R`](R) reader structure"]
impl crate::Readable for Gpio094Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio094::W`](W) writer structure"]
impl crate::Writable for Gpio094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO094 to value 0"]
impl crate::Resettable for Gpio094Spec {}
