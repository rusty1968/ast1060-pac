#[doc = "Register `I3CD07C` reader"]
pub type R = crate::R<I3cd07cSpec>;
#[doc = "Register `I3CD07C` writer"]
pub type W = crate::W<I3cd07cSpec>;
#[doc = "Field `MWL` reader - MWL"]
pub type MwlR = crate::FieldReader<u16>;
#[doc = "Field `MRL` reader - MRL"]
pub type MrlR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - MWL"]
    #[inline(always)]
    pub fn mwl(&self) -> MwlR {
        MwlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MRL"]
    #[inline(always)]
    pub fn mrl(&self) -> MrlR {
        MrlR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "I3C Max Write/Read Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd07cSpec;
impl crate::RegisterSpec for I3cd07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd07c::R`](R) reader structure"]
impl crate::Readable for I3cd07cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd07c::W`](W) writer structure"]
impl crate::Writable for I3cd07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD07C to value 0"]
impl crate::Resettable for I3cd07cSpec {}
