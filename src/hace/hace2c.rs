#[doc = "Register `HACE2C` reader"]
pub type R = crate::R<Hace2cSpec>;
#[doc = "Register `HACE2C` writer"]
pub type W = crate::W<Hace2cSpec>;
#[doc = "Field `HashDataLen` reader - Hash data length"]
pub type HashDataLenR = crate::FieldReader<u32>;
#[doc = "Field `HashDataLen` writer - Hash data length"]
pub type HashDataLenW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:27 - Hash data length"]
    #[inline(always)]
    pub fn hash_data_len(&self) -> HashDataLenR {
        HashDataLenR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - Hash data length"]
    #[inline(always)]
    pub fn hash_data_len(&mut self) -> HashDataLenW<Hace2cSpec> {
        HashDataLenW::new(self, 0)
    }
}
#[doc = "Hash Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace2cSpec;
impl crate::RegisterSpec for Hace2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace2c::R`](R) reader structure"]
impl crate::Readable for Hace2cSpec {}
#[doc = "`write(|w| ..)` method takes [`hace2c::W`](W) writer structure"]
impl crate::Writable for Hace2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE2C to value 0"]
impl crate::Resettable for Hace2cSpec {}
