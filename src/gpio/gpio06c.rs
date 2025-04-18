#[doc = "Register `GPIO06C` reader"]
pub type R = crate::R<Gpio06cSpec>;
#[doc = "Register `GPIO06C` writer"]
pub type W = crate::W<Gpio06cSpec>;
#[doc = "Field `PortGPIOE70CmdSource1` reader - Port GPIOE\\[7:0\\] Command Source 1"]
pub type PortGpioe70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOE70CmdSource1` writer - Port GPIOE\\[7:0\\] Command Source 1"]
pub type PortGpioe70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPIOF70CmdSource1` reader - Port GPIOF\\[7:0\\] Command Source 1"]
pub type PortGpiof70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOF70CmdSource1` writer - Port GPIOF\\[7:0\\] Command Source 1"]
pub type PortGpiof70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOG70CmdSource1` reader - Port GPIOG\\[7:0\\] Command Source 1"]
pub type PortGpiog70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOG70CmdSource1` writer - Port GPIOG\\[7:0\\] Command Source 1"]
pub type PortGpiog70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIOH70CmdSource1` reader - Port GPIOH\\[7:0\\] Command Source 1"]
pub type PortGpioh70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOH70CmdSource1` writer - Port GPIOH\\[7:0\\] Command Source 1"]
pub type PortGpioh70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOE\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioe70cmd_source1(&self) -> PortGpioe70cmdSource1R {
        PortGpioe70cmdSource1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPIOF\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiof70cmd_source1(&self) -> PortGpiof70cmdSource1R {
        PortGpiof70cmdSource1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOG\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiog70cmd_source1(&self) -> PortGpiog70cmdSource1R {
        PortGpiog70cmdSource1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIOH\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioh70cmd_source1(&self) -> PortGpioh70cmdSource1R {
        PortGpioh70cmdSource1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOE\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioe70cmd_source1(&mut self) -> PortGpioe70cmdSource1W<Gpio06cSpec> {
        PortGpioe70cmdSource1W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPIOF\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiof70cmd_source1(&mut self) -> PortGpiof70cmdSource1W<Gpio06cSpec> {
        PortGpiof70cmdSource1W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOG\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiog70cmd_source1(&mut self) -> PortGpiog70cmdSource1W<Gpio06cSpec> {
        PortGpiog70cmdSource1W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIOH\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioh70cmd_source1(&mut self) -> PortGpioh70cmdSource1W<Gpio06cSpec> {
        PortGpioh70cmdSource1W::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio06cSpec;
impl crate::RegisterSpec for Gpio06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio06c::R`](R) reader structure"]
impl crate::Readable for Gpio06cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio06c::W`](W) writer structure"]
impl crate::Writable for Gpio06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO06C to value 0"]
impl crate::Resettable for Gpio06cSpec {}
