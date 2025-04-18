#[doc = "Register `GPIO14C` reader"]
pub type R = crate::R<Gpio14cSpec>;
#[doc = "Register `GPIO14C` writer"]
pub type W = crate::W<Gpio14cSpec>;
#[doc = "Field `PortGPIU70INTSensitivityType0Sel` reader - Port GPIU\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiu70intsensitivityType0selR = crate::FieldReader;
#[doc = "Field `PortGPIU70INTSensitivityType0Sel` writer - Port GPIU\\[7:0\\] interrupt sensitivity type 0 selection"]
pub type PortGpiu70intsensitivityType0selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiu70intsensitivity_type0sel(&self) -> PortGpiu70intsensitivityType0selR {
        PortGpiu70intsensitivityType0selR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn port_gpiu70intsensitivity_type0sel(
        &mut self,
    ) -> PortGpiu70intsensitivityType0selW<Gpio14cSpec> {
        PortGpiu70intsensitivityType0selW::new(self, 0)
    }
}
#[doc = "GPIO\\_U Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio14c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio14c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio14cSpec;
impl crate::RegisterSpec for Gpio14cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio14c::R`](R) reader structure"]
impl crate::Readable for Gpio14cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio14c::W`](W) writer structure"]
impl crate::Writable for Gpio14cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO14C to value 0"]
impl crate::Resettable for Gpio14cSpec {}
