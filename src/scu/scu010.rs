#[doc = "Register `SCU010` reader"]
pub type R = crate::R<Scu010Spec>;
#[doc = "Register `SCU010` writer"]
pub type W = crate::W<Scu010Spec>;
#[doc = "Field `ThisRegIsDesignedToProtectSCURegFromUnpredictableUpdates` reader - This register is designed to protect SCU registers from unpredictable updates,"]
pub type ThisRegIsDesignedToProtectScuregFromUnpredictableUpdatesR = crate::FieldReader<u32>;
#[doc = "Field `ThisRegIsDesignedToProtectSCURegFromUnpredictableUpdates` writer - This register is designed to protect SCU registers from unpredictable updates,"]
pub type ThisRegIsDesignedToProtectScuregFromUnpredictableUpdatesW<'a, REG> =
    crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is designed to protect SCU registers from unpredictable updates,"]
    #[inline(always)]
    pub fn this_reg_is_designed_to_protect_scureg_from_unpredictable_updates(
        &self,
    ) -> ThisRegIsDesignedToProtectScuregFromUnpredictableUpdatesR {
        ThisRegIsDesignedToProtectScuregFromUnpredictableUpdatesR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is designed to protect SCU registers from unpredictable updates,"]
    #[inline(always)]
    pub fn this_reg_is_designed_to_protect_scureg_from_unpredictable_updates(
        &mut self,
    ) -> ThisRegIsDesignedToProtectScuregFromUnpredictableUpdatesW<Scu010Spec> {
        ThisRegIsDesignedToProtectScuregFromUnpredictableUpdatesW::new(self, 0)
    }
}
#[doc = "Protection Key Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu010Spec;
impl crate::RegisterSpec for Scu010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu010::R`](R) reader structure"]
impl crate::Readable for Scu010Spec {}
#[doc = "`write(|w| ..)` method takes [`scu010::W`](W) writer structure"]
impl crate::Writable for Scu010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU010 to value 0"]
impl crate::Resettable for Scu010Spec {}
