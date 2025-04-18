#[doc = "Register `SPI15C` reader"]
pub type R = crate::R<Spi15cSpec>;
#[doc = "Register `SPI15C` writer"]
pub type W = crate::W<Spi15cSpec>;
#[doc = "Field `Cmd03` reader - Command 0"]
pub type Cmd03R = crate::FieldReader;
#[doc = "Field `Cmd03` writer - Command 0"]
pub type Cmd03W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd13` reader - Command 1"]
pub type Cmd13R = crate::FieldReader;
#[doc = "Field `Cmd13` writer - Command 1"]
pub type Cmd13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd23` reader - Command 2"]
pub type Cmd23R = crate::FieldReader;
#[doc = "Field `Cmd23` writer - Command 2"]
pub type Cmd23W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd3 {
    #[doc = "0: Command \\#2/\\#1/\\#0 are for 3B mode."]
    Command210AreFor3bMode = 0,
    #[doc = "1: Command \\#2/\\#1/\\#0 are for 4B mode."]
    Command210AreFor4bMode = 1,
}
impl From<_3b4bcmd3> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd3` reader - 3B/4B Command"]
pub type _3b4bcmd3R = crate::BitReader<_3b4bcmd3>;
impl _3b4bcmd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd3 {
        match self.bits {
            false => _3b4bcmd3::Command210AreFor3bMode,
            true => _3b4bcmd3::Command210AreFor4bMode,
        }
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd3::Command210AreFor3bMode
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_210_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd3::Command210AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd3` writer - 3B/4B Command"]
pub type _3b4bcmd3W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd3>;
impl<'a, REG> _3b4bcmd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#2/\\#1/\\#0 are for 3B mode."]
    #[inline(always)]
    pub fn command_210_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd3::Command210AreFor3bMode)
    }
    #[doc = "Command \\#2/\\#1/\\#0 are for 4B mode."]
    #[inline(always)]
    pub fn command_210_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd3::Command210AreFor4bMode)
    }
}
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead3` reader - Enable Command 0 for read"]
pub type EnblCmd0forRead3R = crate::BitReader;
#[doc = "Field `EnblCmd0ForRead3` writer - Enable Command 0 for read"]
pub type EnblCmd0forRead3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd0ForWr3` reader - Enable Command 0 for write"]
pub type EnblCmd0forWr3R = crate::BitReader;
#[doc = "Field `EnblCmd0ForWr3` writer - Enable Command 0 for write"]
pub type EnblCmd0forWr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForRead3` reader - Enable Command 1 for read"]
pub type EnblCmd1forRead3R = crate::BitReader;
#[doc = "Field `EnblCmd1ForRead3` writer - Enable Command 1 for read"]
pub type EnblCmd1forRead3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd1ForWr3` reader - Enable Command 1 for write"]
pub type EnblCmd1forWr3R = crate::BitReader;
#[doc = "Field `EnblCmd1ForWr3` writer - Enable Command 1 for write"]
pub type EnblCmd1forWr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForRead3` reader - Enable Command 2 for read"]
pub type EnblCmd2forRead3R = crate::BitReader;
#[doc = "Field `EnblCmd2ForRead3` writer - Enable Command 2 for read"]
pub type EnblCmd2forRead3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCmd2ForWr3` reader - Enable Command 2 for write"]
pub type EnblCmd2forWr3R = crate::BitReader;
#[doc = "Field `EnblCmd2ForWr3` writer - Enable Command 2 for write"]
pub type EnblCmd2forWr3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd03(&self) -> Cmd03R {
        Cmd03R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd13(&self) -> Cmd13R {
        Cmd13R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd23(&self) -> Cmd23R {
        Cmd23R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd3(&self) -> _3b4bcmd3R {
        _3b4bcmd3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read3(&self) -> EnblCmd0forRead3R {
        EnblCmd0forRead3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr3(&self) -> EnblCmd0forWr3R {
        EnblCmd0forWr3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read3(&self) -> EnblCmd1forRead3R {
        EnblCmd1forRead3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr3(&self) -> EnblCmd1forWr3R {
        EnblCmd1forWr3R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read3(&self) -> EnblCmd2forRead3R {
        EnblCmd2forRead3R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr3(&self) -> EnblCmd2forWr3R {
        EnblCmd2forWr3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command 0"]
    #[inline(always)]
    pub fn cmd03(&mut self) -> Cmd03W<Spi15cSpec> {
        Cmd03W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command 1"]
    #[inline(always)]
    pub fn cmd13(&mut self) -> Cmd13W<Spi15cSpec> {
        Cmd13W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command 2"]
    #[inline(always)]
    pub fn cmd23(&mut self) -> Cmd23W<Spi15cSpec> {
        Cmd23W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd3(&mut self) -> _3b4bcmd3W<Spi15cSpec> {
        _3b4bcmd3W::new(self, 24)
    }
    #[doc = "Bit 26 - Enable Command 0 for read"]
    #[inline(always)]
    pub fn enbl_cmd0for_read3(&mut self) -> EnblCmd0forRead3W<Spi15cSpec> {
        EnblCmd0forRead3W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable Command 0 for write"]
    #[inline(always)]
    pub fn enbl_cmd0for_wr3(&mut self) -> EnblCmd0forWr3W<Spi15cSpec> {
        EnblCmd0forWr3W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable Command 1 for read"]
    #[inline(always)]
    pub fn enbl_cmd1for_read3(&mut self) -> EnblCmd1forRead3W<Spi15cSpec> {
        EnblCmd1forRead3W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable Command 1 for write"]
    #[inline(always)]
    pub fn enbl_cmd1for_wr3(&mut self) -> EnblCmd1forWr3W<Spi15cSpec> {
        EnblCmd1forWr3W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Command 2 for read"]
    #[inline(always)]
    pub fn enbl_cmd2for_read3(&mut self) -> EnblCmd2forRead3W<Spi15cSpec> {
        EnblCmd2forRead3W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Command 2 for write"]
    #[inline(always)]
    pub fn enbl_cmd2for_wr3(&mut self) -> EnblCmd2forWr3W<Spi15cSpec> {
        EnblCmd2forWr3W::new(self, 31)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi15c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi15c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi15cSpec;
impl crate::RegisterSpec for Spi15cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi15c::R`](R) reader structure"]
impl crate::Readable for Spi15cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi15c::W`](W) writer structure"]
impl crate::Writable for Spi15cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI15C to value 0"]
impl crate::Resettable for Spi15cSpec {}
