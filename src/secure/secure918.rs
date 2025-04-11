#[doc = "Register `SECURE918` reader"]
pub type R = crate::R<Secure918Spec>;
#[doc = "Register `SECURE918` writer"]
pub type W = crate::W<Secure918Spec>;
#[doc = "Field `SecBootFirstVaultKey6Reg` reader - Secure Boot First Vault Key 6 Register"]
pub type SecBootFirstVaultKey6regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootFirstVaultKey6Reg` writer - Secure Boot First Vault Key 6 Register"]
pub type SecBootFirstVaultKey6regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 6 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key6reg(&self) -> SecBootFirstVaultKey6regR {
        SecBootFirstVaultKey6regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 6 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key6reg(&mut self) -> SecBootFirstVaultKey6regW<Secure918Spec> {
        SecBootFirstVaultKey6regW::new(self, 0)
    }
}
#[doc = "Secure Boot First Vault Key 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure918::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure918::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure918Spec;
impl crate::RegisterSpec for Secure918Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure918::R`](R) reader structure"]
impl crate::Readable for Secure918Spec {}
#[doc = "`write(|w| ..)` method takes [`secure918::W`](W) writer structure"]
impl crate::Writable for Secure918Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE918 to value 0"]
impl crate::Resettable for Secure918Spec {}
