#[doc = "Register `GPIO060` reader"]
pub type R = crate::R<Gpio060Spec>;
#[doc = "Register `GPIO060` writer"]
pub type W = crate::W<Gpio060Spec>;
#[doc = "Field `PortGPIOA70CmdSource0` reader - Port GPIOA\\[7:0\\] Command Source 0"]
pub type PortGpioa70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOA70CmdSource0` writer - Port GPIOA\\[7:0\\] Command Source 0"]
pub type PortGpioa70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPIOB70CmdSource0` reader - Port GPIOB\\[7:0\\] Command Source 0"]
pub type PortGpiob70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOB70CmdSource0` writer - Port GPIOB\\[7:0\\] Command Source 0"]
pub type PortGpiob70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOC70CmdSource0` reader - Port GPIOC\\[7:0\\] Command Source 0"]
pub type PortGpioc70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOC70CmdSource0` writer - Port GPIOC\\[7:0\\] Command Source 0"]
pub type PortGpioc70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIOD70CmdSource0` reader - Port GPIOD\\[7:0\\] Command Source 0"]
pub type PortGpiod70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOD70CmdSource0` writer - Port GPIOD\\[7:0\\] Command Source 0"]
pub type PortGpiod70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOA\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioa70cmd_source0(&self) -> PortGpioa70cmdSource0R {
        PortGpioa70cmdSource0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPIOB\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiob70cmd_source0(&self) -> PortGpiob70cmdSource0R {
        PortGpiob70cmdSource0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOC\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioc70cmd_source0(&self) -> PortGpioc70cmdSource0R {
        PortGpioc70cmdSource0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIOD\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiod70cmd_source0(&self) -> PortGpiod70cmdSource0R {
        PortGpiod70cmdSource0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOA\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioa70cmd_source0(&mut self) -> PortGpioa70cmdSource0W<Gpio060Spec> {
        PortGpioa70cmdSource0W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPIOB\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiob70cmd_source0(&mut self) -> PortGpiob70cmdSource0W<Gpio060Spec> {
        PortGpiob70cmdSource0W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOC\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioc70cmd_source0(&mut self) -> PortGpioc70cmdSource0W<Gpio060Spec> {
        PortGpioc70cmdSource0W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIOD\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiod70cmd_source0(&mut self) -> PortGpiod70cmdSource0W<Gpio060Spec> {
        PortGpiod70cmdSource0W::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio060Spec;
impl crate::RegisterSpec for Gpio060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio060::R`](R) reader structure"]
impl crate::Readable for Gpio060Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio060::W`](W) writer structure"]
impl crate::Writable for Gpio060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO060 to value 0"]
impl crate::Resettable for Gpio060Spec {}
