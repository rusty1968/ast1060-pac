#[doc = "Register `SPI178` reader"]
pub type R = crate::R<Spi178Spec>;
#[doc = "Register `SPI178` writer"]
pub type W = crate::W<Spi178Spec>;
#[doc = "Field `Cmd010` reader - Command 0"]
pub type Cmd010R = crate::FieldReader;
#[doc = "Field `Cmd010` writer - Command 0"]
pub type Cmd010W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd110` reader - Command 1"]
pub type Cmd110R = crate::FieldReader;
#[doc = "Field `Cmd110` writer - Command 1"]
pub type Cmd110W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd210` reader - Command 2"]
pub type Cmd210R = crate::FieldReader;
#[doc = "Field `Cmd210` writer - Command 2"]
pub type Cmd210W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd10 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd10> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd10` reader - 3B/4B Command"]
pub type _3b4bcmd10R = crate::BitReader<_3b4bcmd10>;
impl _3b4bcmd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd10 {
        match self.bits {
            false => _3b4bcmd10::Command210AreFor3bMode,
            true => _3b4bcmd10::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd10::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd10::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd10` writer - 3B/4B Command"]
pub type _3b4bcmd10W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd10>;
impl<'a, REG> _3b4bcmd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd10::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd10::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead10` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead10R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead10` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr10` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr10R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr10` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead10` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead10R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead10` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr10` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr10R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr10` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead10` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead10R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead10` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr10` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr10R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr10` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr10W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd010(&self) -> Cmd010R {
        Cmd010R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd110(&self) -> Cmd110R {
        Cmd110R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd210(&self) -> Cmd210R {
        Cmd210R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd10(&self) -> _3b4bcmd10R {
        _3b4bcmd10R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read10(&self) -> EnblCmd0forRead10R {
        EnblCmd0forRead10R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr10(&self) -> EnblCmd0forWr10R {
        EnblCmd0forWr10R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read10(&self) -> EnblCmd1forRead10R {
        EnblCmd1forRead10R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr10(&self) -> EnblCmd1forWr10R {
        EnblCmd1forWr10R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read10(&self) -> EnblCmd2forRead10R {
        EnblCmd2forRead10R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr10(&self) -> EnblCmd2forWr10R {
        EnblCmd2forWr10R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd010(&mut self) -> Cmd010W<Spi178Spec> {
        Cmd010W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd110(&mut self) -> Cmd110W<Spi178Spec> {
        Cmd110W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd210(&mut self) -> Cmd210W<Spi178Spec> {
        Cmd210W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd10(&mut self) -> _3b4bcmd10W<Spi178Spec> {
        _3b4bcmd10W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read10(&mut self) -> EnblCmd0forRead10W<Spi178Spec> {
        EnblCmd0forRead10W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr10(&mut self) -> EnblCmd0forWr10W<Spi178Spec> {
        EnblCmd0forWr10W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read10(&mut self) -> EnblCmd1forRead10W<Spi178Spec> {
        EnblCmd1forRead10W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr10(&mut self) -> EnblCmd1forWr10W<Spi178Spec> {
        EnblCmd1forWr10W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read10(&mut self) -> EnblCmd2forRead10W<Spi178Spec> {
        EnblCmd2forRead10W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr10(&mut self) -> EnblCmd2forWr10W<Spi178Spec> {
        EnblCmd2forWr10W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi178::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi178::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi178Spec;
impl crate::RegisterSpec for Spi178Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi178::R`](R) reader structure"]
impl crate::Readable for Spi178Spec {}
#[doc = "`write(|w| ..)` method takes [`spi178::W`](W) writer structure"]
impl crate::Writable for Spi178Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI178 to value 0"]
impl crate::Resettable for Spi178Spec {}
