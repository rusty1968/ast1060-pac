#[doc = "Register `I3CD070` reader"]
pub type R = crate::R<I3cd070Spec>;
#[doc = "Register `I3CD070` writer"]
pub type W = crate::W<I3cd070Spec>;
#[doc = "Field `SLVPIDDCR` reader - SLV_PID_DCR"]
pub type SlvpiddcrR = crate::BitReader;
#[doc = "Field `SLVPIDDCR` writer - SLV_PID_DCR"]
pub type SlvpiddcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVMIPIMFGID` reader - SLV_MIPI_MFG_ID"]
pub type SlvmipimfgidR = crate::FieldReader<u16>;
#[doc = "Field `SLVMIPIMFGID` writer - SLV_MIPI_MFG_ID"]
pub type SlvmipimfgidW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - SLV_PID_DCR"]
    #[inline(always)]
    pub fn slvpiddcr(&self) -> SlvpiddcrR {
        SlvpiddcrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15 - SLV_MIPI_MFG_ID"]
    #[inline(always)]
    pub fn slvmipimfgid(&self) -> SlvmipimfgidR {
        SlvmipimfgidR::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 16:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - SLV_PID_DCR"]
    #[inline(always)]
    pub fn slvpiddcr(&mut self) -> SlvpiddcrW<I3cd070Spec> {
        SlvpiddcrW::new(self, 0)
    }
    #[doc = "Bits 1:15 - SLV_MIPI_MFG_ID"]
    #[inline(always)]
    pub fn slvmipimfgid(&mut self) -> SlvmipimfgidW<I3cd070Spec> {
        SlvmipimfgidW::new(self, 1)
    }
}
#[doc = "Provisional ID Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd070Spec;
impl crate::RegisterSpec for I3cd070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd070::R`](R) reader structure"]
impl crate::Readable for I3cd070Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd070::W`](W) writer structure"]
impl crate::Writable for I3cd070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD070 to value 0"]
impl crate::Resettable for I3cd070Spec {}
