#[doc = "Register `SPI0A4` reader"]
pub type R = crate::R<Spi0a4Spec>;
#[doc = "Register `SPI0A4` writer"]
pub type W = crate::W<Spi0a4Spec>;
#[doc = "Mode of write address filter #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ModeOfWrAddrFilter1 {
    #[doc = "0: NOP"]
    Nop = 0,
    #[doc = "1: Read only"]
    ReadOnly = 1,
    #[doc = "2: Write only"]
    WriteOnly = 2,
    #[doc = "3: Access disabled"]
    AccessDisabled = 3,
}
impl From<ModeOfWrAddrFilter1> for u8 {
    #[inline(always)]
    fn from(variant: ModeOfWrAddrFilter1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ModeOfWrAddrFilter1 {
    type Ux = u8;
}
impl crate::IsEnum for ModeOfWrAddrFilter1 {}
#[doc = "Field `ModeOfWrAddrFilter1` reader - Mode of write address filter #1"]
pub type ModeOfWrAddrFilter1R = crate::FieldReader<ModeOfWrAddrFilter1>;
impl ModeOfWrAddrFilter1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ModeOfWrAddrFilter1 {
        match self.bits {
            0 => ModeOfWrAddrFilter1::Nop,
            1 => ModeOfWrAddrFilter1::ReadOnly,
            2 => ModeOfWrAddrFilter1::WriteOnly,
            3 => ModeOfWrAddrFilter1::AccessDisabled,
            _ => unreachable!(),
        }
    }
    #[doc = "NOP"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == ModeOfWrAddrFilter1::Nop
    }
    #[doc = "Read only"]
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == ModeOfWrAddrFilter1::ReadOnly
    }
    #[doc = "Write only"]
    #[inline(always)]
    pub fn is_write_only(&self) -> bool {
        *self == ModeOfWrAddrFilter1::WriteOnly
    }
    #[doc = "Access disabled"]
    #[inline(always)]
    pub fn is_access_disabled(&self) -> bool {
        *self == ModeOfWrAddrFilter1::AccessDisabled
    }
}
#[doc = "Field `ModeOfWrAddrFilter1` writer - Mode of write address filter #1"]
pub type ModeOfWrAddrFilter1W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, ModeOfWrAddrFilter1, crate::Safe>;
impl<'a, REG> ModeOfWrAddrFilter1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NOP"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(ModeOfWrAddrFilter1::Nop)
    }
    #[doc = "Read only"]
    #[inline(always)]
    pub fn read_only(self) -> &'a mut crate::W<REG> {
        self.variant(ModeOfWrAddrFilter1::ReadOnly)
    }
    #[doc = "Write only"]
    #[inline(always)]
    pub fn write_only(self) -> &'a mut crate::W<REG> {
        self.variant(ModeOfWrAddrFilter1::WriteOnly)
    }
    #[doc = "Access disabled"]
    #[inline(always)]
    pub fn access_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ModeOfWrAddrFilter1::AccessDisabled)
    }
}
#[doc = "Field `ModeOfWrAddrFilter2` reader - Mode of write address filter #2"]
pub type ModeOfWrAddrFilter2R = crate::FieldReader;
#[doc = "Field `ModeOfWrAddrFilter2` writer - Mode of write address filter #2"]
pub type ModeOfWrAddrFilter2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ModeOfWrAddrFilter3` reader - Mode of write address filter #3"]
pub type ModeOfWrAddrFilter3R = crate::FieldReader;
#[doc = "Field `ModeOfWrAddrFilter3` writer - Mode of write address filter #3"]
pub type ModeOfWrAddrFilter3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ModeOfWrAddrFilter4` reader - Mode of write address filter #4"]
pub type ModeOfWrAddrFilter4R = crate::FieldReader;
#[doc = "Field `ModeOfWrAddrFilter4` writer - Mode of write address filter #4"]
pub type ModeOfWrAddrFilter4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ModeOfWrAddrFilter5` reader - Mode of write address filter #5"]
pub type ModeOfWrAddrFilter5R = crate::FieldReader;
#[doc = "Field `ModeOfWrAddrFilter5` writer - Mode of write address filter #5"]
pub type ModeOfWrAddrFilter5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ModeOfWrAddrFilter6` reader - Mode of write address filter #6"]
pub type ModeOfWrAddrFilter6R = crate::FieldReader;
#[doc = "Field `ModeOfWrAddrFilter6` writer - Mode of write address filter #6"]
pub type ModeOfWrAddrFilter6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Mode of write address filter #1"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter1(&self) -> ModeOfWrAddrFilter1R {
        ModeOfWrAddrFilter1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Mode of write address filter #2"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter2(&self) -> ModeOfWrAddrFilter2R {
        ModeOfWrAddrFilter2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Mode of write address filter #3"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter3(&self) -> ModeOfWrAddrFilter3R {
        ModeOfWrAddrFilter3R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Mode of write address filter #4"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter4(&self) -> ModeOfWrAddrFilter4R {
        ModeOfWrAddrFilter4R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Mode of write address filter #5"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter5(&self) -> ModeOfWrAddrFilter5R {
        ModeOfWrAddrFilter5R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Mode of write address filter #6"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter6(&self) -> ModeOfWrAddrFilter6R {
        ModeOfWrAddrFilter6R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode of write address filter #1"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter1(&mut self) -> ModeOfWrAddrFilter1W<Spi0a4Spec> {
        ModeOfWrAddrFilter1W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Mode of write address filter #2"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter2(&mut self) -> ModeOfWrAddrFilter2W<Spi0a4Spec> {
        ModeOfWrAddrFilter2W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Mode of write address filter #3"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter3(&mut self) -> ModeOfWrAddrFilter3W<Spi0a4Spec> {
        ModeOfWrAddrFilter3W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Mode of write address filter #4"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter4(&mut self) -> ModeOfWrAddrFilter4W<Spi0a4Spec> {
        ModeOfWrAddrFilter4W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Mode of write address filter #5"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter5(&mut self) -> ModeOfWrAddrFilter5W<Spi0a4Spec> {
        ModeOfWrAddrFilter5W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Mode of write address filter #6"]
    #[inline(always)]
    pub fn mode_of_wr_addr_filter6(&mut self) -> ModeOfWrAddrFilter6W<Spi0a4Spec> {
        ModeOfWrAddrFilter6W::new(self, 10)
    }
}
#[doc = "Write Address Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0a4Spec;
impl crate::RegisterSpec for Spi0a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0a4::R`](R) reader structure"]
impl crate::Readable for Spi0a4Spec {}
#[doc = "`write(|w| ..)` method takes [`spi0a4::W`](W) writer structure"]
impl crate::Writable for Spi0a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0A4 to value 0"]
impl crate::Resettable for Spi0a4Spec {}
