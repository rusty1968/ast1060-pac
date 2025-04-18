#[doc = "Register `SPI154` reader"]
pub type R = crate::R<Spi154Spec>;
#[doc = "Register `SPI154` writer"]
pub type W = crate::W<Spi154Spec>;
#[doc = "Field `Cmd01` reader - Command 0"]
pub type Cmd01R = crate::FieldReader;
#[doc = "Field `Cmd01` writer - Command 0"]
pub type Cmd01W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd11` reader - Command 1"]
pub type Cmd11R = crate::FieldReader;
#[doc = "Field `Cmd11` writer - Command 1"]
pub type Cmd11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd21` reader - Command 2"]
pub type Cmd21R = crate::FieldReader;
#[doc = "Field `Cmd21` writer - Command 2"]
pub type Cmd21W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd1 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd1> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd1` reader - 3B/4B Command"]
pub type _3b4bcmd1R = crate::BitReader<_3b4bcmd1>;
impl _3b4bcmd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd1 {
        match self.bits {
            false => _3b4bcmd1::Command210AreFor3bMode,
            true => _3b4bcmd1::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd1::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd1::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd1` writer - 3B/4B Command"]
pub type _3b4bcmd1W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd1>;
impl<'a, REG> _3b4bcmd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd1::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd1::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead1` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead1R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead1` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr1` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr1R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr1` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead1` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead1R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead1` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr1` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr1R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr1` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead1` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead1R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead1` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr1` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr1R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr1` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd01(&self) -> Cmd01R {
        Cmd01R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd11(&self) -> Cmd11R {
        Cmd11R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd21(&self) -> Cmd21R {
        Cmd21R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd1(&self) -> _3b4bcmd1R {
        _3b4bcmd1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read1(&self) -> EnblCmd0forRead1R {
        EnblCmd0forRead1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr1(&self) -> EnblCmd0forWr1R {
        EnblCmd0forWr1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read1(&self) -> EnblCmd1forRead1R {
        EnblCmd1forRead1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr1(&self) -> EnblCmd1forWr1R {
        EnblCmd1forWr1R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read1(&self) -> EnblCmd2forRead1R {
        EnblCmd2forRead1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr1(&self) -> EnblCmd2forWr1R {
        EnblCmd2forWr1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd01(&mut self) -> Cmd01W<Spi154Spec> {
        Cmd01W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd11(&mut self) -> Cmd11W<Spi154Spec> {
        Cmd11W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd21(&mut self) -> Cmd21W<Spi154Spec> {
        Cmd21W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd1(&mut self) -> _3b4bcmd1W<Spi154Spec> {
        _3b4bcmd1W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read1(&mut self) -> EnblCmd0forRead1W<Spi154Spec> {
        EnblCmd0forRead1W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr1(&mut self) -> EnblCmd0forWr1W<Spi154Spec> {
        EnblCmd0forWr1W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read1(&mut self) -> EnblCmd1forRead1W<Spi154Spec> {
        EnblCmd1forRead1W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr1(&mut self) -> EnblCmd1forWr1W<Spi154Spec> {
        EnblCmd1forWr1W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read1(&mut self) -> EnblCmd2forRead1W<Spi154Spec> {
        EnblCmd2forRead1W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr1(&mut self) -> EnblCmd2forWr1W<Spi154Spec> {
        EnblCmd2forWr1W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi154::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi154::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi154Spec;
impl crate::RegisterSpec for Spi154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi154::R`](R) reader structure"]
impl crate::Readable for Spi154Spec {}
#[doc = "`write(|w| ..)` method takes [`spi154::W`](W) writer structure"]
impl crate::Writable for Spi154Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI154 to value 0x2800_dcd8"]
impl crate::Resettable for Spi154Spec {
    const RESET_VALUE: u32 = 0x2800_dcd8;
}
