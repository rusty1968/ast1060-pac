#[doc = "Register `GPIO0E0` reader"]
pub type R = crate::R<Gpio0e0Spec>;
#[doc = "Register `GPIO0E0` writer"]
pub type W = crate::W<Gpio0e0Spec>;
#[doc = "Field `PortGPIOM70CmdSource0` reader - Port GPIOM\\[7:0\\] Command Source 0"]
pub type PortGpiom70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOM70CmdSource0` writer - Port GPIOM\\[7:0\\] Command Source 0"]
pub type PortGpiom70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPION70CmdSource0` reader - Port GPION\\[7:0\\] Command Source 0"]
pub type PortGpion70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPION70CmdSource0` writer - Port GPION\\[7:0\\] Command Source 0"]
pub type PortGpion70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOO70CmdSource0` reader - Port GPIOO\\[7:0\\] Command Source 0"]
pub type PortGpioo70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOO70CmdSource0` writer - Port GPIOO\\[7:0\\] Command Source 0"]
pub type PortGpioo70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIOP70CmdSource0` reader - Port GPIOP\\[7:0\\] Command Source 0"]
pub type PortGpiop70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOP70CmdSource0` writer - Port GPIOP\\[7:0\\] Command Source 0"]
pub type PortGpiop70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOM\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiom70cmd_source0(&self) -> PortGpiom70cmdSource0R {
        PortGpiom70cmdSource0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPION\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpion70cmd_source0(&self) -> PortGpion70cmdSource0R {
        PortGpion70cmdSource0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOO\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioo70cmd_source0(&self) -> PortGpioo70cmdSource0R {
        PortGpioo70cmdSource0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIOP\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiop70cmd_source0(&self) -> PortGpiop70cmdSource0R {
        PortGpiop70cmdSource0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOM\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiom70cmd_source0(&mut self) -> PortGpiom70cmdSource0W<Gpio0e0Spec> {
        PortGpiom70cmdSource0W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPION\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpion70cmd_source0(&mut self) -> PortGpion70cmdSource0W<Gpio0e0Spec> {
        PortGpion70cmdSource0W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOO\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioo70cmd_source0(&mut self) -> PortGpioo70cmdSource0W<Gpio0e0Spec> {
        PortGpioo70cmdSource0W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIOP\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiop70cmd_source0(&mut self) -> PortGpiop70cmdSource0W<Gpio0e0Spec> {
        PortGpiop70cmdSource0W::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0e0Spec;
impl crate::RegisterSpec for Gpio0e0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0e0::R`](R) reader structure"]
impl crate::Readable for Gpio0e0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0e0::W`](W) writer structure"]
impl crate::Writable for Gpio0e0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0E0 to value 0"]
impl crate::Resettable for Gpio0e0Spec {}
