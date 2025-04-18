#[doc = "Register `SECURE08C` reader"]
pub type R = crate::R<Secure08cSpec>;
#[doc = "Register `SECURE08C` writer"]
pub type W = crate::W<Secure08cSpec>;
#[doc = "Field `SecBootEngIntCtrlRegs` reader - Secure Boot Engine Internal Controller Registers"]
pub type SecBootEngIntCtrlRegsR = crate::FieldReader<u32>;
#[doc = "Field `SecBootEngIntCtrlRegs` writer - Secure Boot Engine Internal Controller Registers"]
pub type SecBootEngIntCtrlRegsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Engine Internal Controller Registers"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_regs(&self) -> SecBootEngIntCtrlRegsR {
        SecBootEngIntCtrlRegsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Engine Internal Controller Registers"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_regs(&mut self) -> SecBootEngIntCtrlRegsW<Secure08cSpec> {
        SecBootEngIntCtrlRegsW::new(self, 0)
    }
}
#[doc = "Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure08cSpec;
impl crate::RegisterSpec for Secure08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure08c::R`](R) reader structure"]
impl crate::Readable for Secure08cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure08c::W`](W) writer structure"]
impl crate::Writable for Secure08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE08C to value 0"]
impl crate::Resettable for Secure08cSpec {}
