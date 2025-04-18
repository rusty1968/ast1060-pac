#[doc = "Register `SECURE92C` reader"]
pub type R = crate::R<Secure92cSpec>;
#[doc = "Register `SECURE92C` writer"]
pub type W = crate::W<Secure92cSpec>;
#[doc = "Field `SecBootSecondVaultKey3Reg` reader - Secure Boot Second Vault Key 3 Register"]
pub type SecBootSecondVaultKey3regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootSecondVaultKey3Reg` writer - Secure Boot Second Vault Key 3 Register"]
pub type SecBootSecondVaultKey3regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 3 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key3reg(&self) -> SecBootSecondVaultKey3regR {
        SecBootSecondVaultKey3regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 3 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key3reg(&mut self) -> SecBootSecondVaultKey3regW<Secure92cSpec> {
        SecBootSecondVaultKey3regW::new(self, 0)
    }
}
#[doc = "Secure Boot Second Vault Key 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure92c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure92c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure92cSpec;
impl crate::RegisterSpec for Secure92cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure92c::R`](R) reader structure"]
impl crate::Readable for Secure92cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure92c::W`](W) writer structure"]
impl crate::Writable for Secure92cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE92C to value 0"]
impl crate::Resettable for Secure92cSpec {}
