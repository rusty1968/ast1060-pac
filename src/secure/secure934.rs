#[doc = "Register `SECURE934` reader"]
pub type R = crate::R<Secure934Spec>;
#[doc = "Register `SECURE934` writer"]
pub type W = crate::W<Secure934Spec>;
#[doc = "Field `SecBootSecondVaultKey5Reg` reader - Secure Boot Second Vault Key 5 Register"]
pub type SecBootSecondVaultKey5regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootSecondVaultKey5Reg` writer - Secure Boot Second Vault Key 5 Register"]
pub type SecBootSecondVaultKey5regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 5 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key5reg(&self) -> SecBootSecondVaultKey5regR {
        SecBootSecondVaultKey5regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 5 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key5reg(&mut self) -> SecBootSecondVaultKey5regW<Secure934Spec> {
        SecBootSecondVaultKey5regW::new(self, 0)
    }
}
#[doc = "Secure Boot Second Vault Key 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure934::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure934::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure934Spec;
impl crate::RegisterSpec for Secure934Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure934::R`](R) reader structure"]
impl crate::Readable for Secure934Spec {}
#[doc = "`write(|w| ..)` method takes [`secure934::W`](W) writer structure"]
impl crate::Writable for Secure934Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE934 to value 0"]
impl crate::Resettable for Secure934Spec {}
