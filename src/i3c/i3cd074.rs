#[doc = "Register `I3CD074` reader"]
pub type R = crate::R<I3cd074Spec>;
#[doc = "Register `I3CD074` writer"]
pub type W = crate::W<I3cd074Spec>;
#[doc = "Field `SLVPIDDCR` reader - SLV_PID_DCR"]
pub type SlvpiddcrR = crate::FieldReader<u16>;
#[doc = "Field `SLVINSTID` reader - SLV_INST_ID"]
pub type SlvinstidR = crate::FieldReader;
#[doc = "Field `SLVINSTID` writer - SLV_INST_ID"]
pub type SlvinstidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SLVPARTID` reader - SLV_PART_ID"]
pub type SlvpartidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - SLV_PID_DCR"]
    #[inline(always)]
    pub fn slvpiddcr(&self) -> SlvpiddcrR {
        SlvpiddcrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - SLV_INST_ID"]
    #[inline(always)]
    pub fn slvinstid(&self) -> SlvinstidR {
        SlvinstidR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - SLV_PART_ID"]
    #[inline(always)]
    pub fn slvpartid(&self) -> SlvpartidR {
        SlvpartidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:15 - SLV_INST_ID"]
    #[inline(always)]
    pub fn slvinstid(&mut self) -> SlvinstidW<I3cd074Spec> {
        SlvinstidW::new(self, 12)
    }
}
#[doc = "Provisional ID Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd074Spec;
impl crate::RegisterSpec for I3cd074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd074::R`](R) reader structure"]
impl crate::Readable for I3cd074Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd074::W`](W) writer structure"]
impl crate::Writable for I3cd074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD074 to value 0"]
impl crate::Resettable for I3cd074Spec {}
