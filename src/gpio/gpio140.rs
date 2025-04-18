#[doc = "Register `GPIO140` reader"]
pub type R = crate::R<Gpio140Spec>;
#[doc = "Register `GPIO140` writer"]
pub type W = crate::W<Gpio140Spec>;
#[doc = "Field `PortGPIU70CmdSource0` reader - Port GPIU\\[7:0\\] Command Source 0"]
pub type PortGpiu70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIU70CmdSource0` writer - Port GPIU\\[7:0\\] Command Source 0"]
pub type PortGpiu70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIU\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiu70cmd_source0(&self) -> PortGpiu70cmdSource0R {
        PortGpiu70cmdSource0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIU\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiu70cmd_source0(&mut self) -> PortGpiu70cmdSource0W<Gpio140Spec> {
        PortGpiu70cmdSource0W::new(self, 0)
    }
}
#[doc = "GPIO\\_U Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio140::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio140::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio140Spec;
impl crate::RegisterSpec for Gpio140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio140::R`](R) reader structure"]
impl crate::Readable for Gpio140Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio140::W`](W) writer structure"]
impl crate::Writable for Gpio140Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO140 to value 0"]
impl crate::Resettable for Gpio140Spec {}
