#[doc = "Register `I3CD040` reader"]
pub type R = crate::R<I3cd040Spec>;
#[doc = "Register `I3CD040` writer"]
pub type W = crate::W<I3cd040Spec>;
#[doc = "Field `TXTHLDSTATEN` reader - TX_THLD_STAT_EN"]
pub type TxthldstatenR = crate::BitReader;
#[doc = "Field `TXTHLDSTATEN` writer - TX_THLD_STAT_EN"]
pub type TxthldstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHLDSTATEN` reader - RX_THLD_STAT_EN"]
pub type RxthldstatenR = crate::BitReader;
#[doc = "Field `RXTHLDSTATEN` writer - RX_THLD_STAT_EN"]
pub type RxthldstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBITHLDSTATEN` reader - IBI_THLD_STAT_EN"]
pub type IbithldstatenR = crate::BitReader;
#[doc = "Field `IBITHLDSTATEN` writer - IBI_THLD_STAT_EN"]
pub type IbithldstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDQUEUEREADYSTATEN` reader - CMD_QUEUE_READY_STAT_EN"]
pub type CmdqueuereadystatenR = crate::BitReader;
#[doc = "Field `CMDQUEUEREADYSTATEN` writer - CMD_QUEUE_READY_STAT_EN"]
pub type CmdqueuereadystatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPREADYSTATINTREN` reader - RESP_READY_STAT_INTR_EN"]
pub type RespreadystatintrenR = crate::BitReader;
#[doc = "Field `RESPREADYSTATINTREN` writer - RESP_READY_STAT_INTR_EN"]
pub type RespreadystatintrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFERABORTSTATEN` reader - TRANSFER_ABORT_STAT_EN"]
pub type TransferabortstatenR = crate::BitReader;
#[doc = "Field `TRANSFERABORTSTATEN` writer - TRANSFER_ABORT_STAT_EN"]
pub type TransferabortstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCCUPDATEDSTATEN` reader - CCC_UPDATED_STAT_EN"]
pub type CccupdatedstatenR = crate::BitReader;
#[doc = "Field `CCCUPDATEDSTATEN` writer - CCC_UPDATED_STAT_EN"]
pub type CccupdatedstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD7` reader - This bit in Interrupt Status Enable register is reserved."]
pub type Rsvd7R = crate::BitReader;
#[doc = "Field `DYNADDRASSGNSTATEN` reader - DYN_ADDR_ASSGN_STAT_EN"]
pub type DynaddrassgnstatenR = crate::BitReader;
#[doc = "Field `DYNADDRASSGNSTATEN` writer - DYN_ADDR_ASSGN_STAT_EN"]
pub type DynaddrassgnstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFERERRSTATEN` reader - TRANSFER_ERR_STAT_EN"]
pub type TransfererrstatenR = crate::BitReader;
#[doc = "Field `TRANSFERERRSTATEN` writer - TRANSFER_ERR_STAT_EN"]
pub type TransfererrstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFSLVSTATEN` reader - DEFSLV_STAT_EN"]
pub type DefslvstatenR = crate::BitReader;
#[doc = "Field `DEFSLVSTATEN` writer - DEFSLV_STAT_EN"]
pub type DefslvstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READREQRECVSTATEN` reader - READ_REQ_RECV_STAT_EN"]
pub type ReadreqrecvstatenR = crate::BitReader;
#[doc = "Field `READREQRECVSTATEN` writer - READ_REQ_RECV_STAT_EN"]
pub type ReadreqrecvstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIUPDATEDSTATEN` reader - IBI_UPDATED_STAT_EN"]
pub type IbiupdatedstatenR = crate::BitReader;
#[doc = "Field `IBIUPDATEDSTATEN` writer - IBI_UPDATED_STAT_EN"]
pub type IbiupdatedstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSOWNERUPDATEDSTATEN` reader - BUSOWNER_UPDATED_STAT_EN"]
pub type BusownerupdatedstatenR = crate::BitReader;
#[doc = "Field `BUSOWNERUPDATEDSTATEN` writer - BUSOWNER_UPDATED_STAT_EN"]
pub type BusownerupdatedstatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD1` reader - reserved"]
pub type Rsvd1R = crate::BitReader;
#[doc = "Field `BUSRESETDONESTSEN` reader - BUS_RESET_DONE_STS_EN"]
pub type BusresetdonestsenR = crate::BitReader;
#[doc = "Field `BUSRESETDONESTSEN` writer - BUS_RESET_DONE_STS_EN"]
pub type BusresetdonestsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - TX_THLD_STAT_EN"]
    #[inline(always)]
    pub fn txthldstaten(&self) -> TxthldstatenR {
        TxthldstatenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_THLD_STAT_EN"]
    #[inline(always)]
    pub fn rxthldstaten(&self) -> RxthldstatenR {
        RxthldstatenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IBI_THLD_STAT_EN"]
    #[inline(always)]
    pub fn ibithldstaten(&self) -> IbithldstatenR {
        IbithldstatenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMD_QUEUE_READY_STAT_EN"]
    #[inline(always)]
    pub fn cmdqueuereadystaten(&self) -> CmdqueuereadystatenR {
        CmdqueuereadystatenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RESP_READY_STAT_INTR_EN"]
    #[inline(always)]
    pub fn respreadystatintren(&self) -> RespreadystatintrenR {
        RespreadystatintrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TRANSFER_ABORT_STAT_EN"]
    #[inline(always)]
    pub fn transferabortstaten(&self) -> TransferabortstatenR {
        TransferabortstatenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCC_UPDATED_STAT_EN"]
    #[inline(always)]
    pub fn cccupdatedstaten(&self) -> CccupdatedstatenR {
        CccupdatedstatenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit in Interrupt Status Enable register is reserved."]
    #[inline(always)]
    pub fn rsvd7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DYN_ADDR_ASSGN_STAT_EN"]
    #[inline(always)]
    pub fn dynaddrassgnstaten(&self) -> DynaddrassgnstatenR {
        DynaddrassgnstatenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRANSFER_ERR_STAT_EN"]
    #[inline(always)]
    pub fn transfererrstaten(&self) -> TransfererrstatenR {
        TransfererrstatenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DEFSLV_STAT_EN"]
    #[inline(always)]
    pub fn defslvstaten(&self) -> DefslvstatenR {
        DefslvstatenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - READ_REQ_RECV_STAT_EN"]
    #[inline(always)]
    pub fn readreqrecvstaten(&self) -> ReadreqrecvstatenR {
        ReadreqrecvstatenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IBI_UPDATED_STAT_EN"]
    #[inline(always)]
    pub fn ibiupdatedstaten(&self) -> IbiupdatedstatenR {
        IbiupdatedstatenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BUSOWNER_UPDATED_STAT_EN"]
    #[inline(always)]
    pub fn busownerupdatedstaten(&self) -> BusownerupdatedstatenR {
        BusownerupdatedstatenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BUS_RESET_DONE_STS_EN"]
    #[inline(always)]
    pub fn busresetdonestsen(&self) -> BusresetdonestsenR {
        BusresetdonestsenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TX_THLD_STAT_EN"]
    #[inline(always)]
    pub fn txthldstaten(&mut self) -> TxthldstatenW<I3cd040Spec> {
        TxthldstatenW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_THLD_STAT_EN"]
    #[inline(always)]
    pub fn rxthldstaten(&mut self) -> RxthldstatenW<I3cd040Spec> {
        RxthldstatenW::new(self, 1)
    }
    #[doc = "Bit 2 - IBI_THLD_STAT_EN"]
    #[inline(always)]
    pub fn ibithldstaten(&mut self) -> IbithldstatenW<I3cd040Spec> {
        IbithldstatenW::new(self, 2)
    }
    #[doc = "Bit 3 - CMD_QUEUE_READY_STAT_EN"]
    #[inline(always)]
    pub fn cmdqueuereadystaten(&mut self) -> CmdqueuereadystatenW<I3cd040Spec> {
        CmdqueuereadystatenW::new(self, 3)
    }
    #[doc = "Bit 4 - RESP_READY_STAT_INTR_EN"]
    #[inline(always)]
    pub fn respreadystatintren(&mut self) -> RespreadystatintrenW<I3cd040Spec> {
        RespreadystatintrenW::new(self, 4)
    }
    #[doc = "Bit 5 - TRANSFER_ABORT_STAT_EN"]
    #[inline(always)]
    pub fn transferabortstaten(&mut self) -> TransferabortstatenW<I3cd040Spec> {
        TransferabortstatenW::new(self, 5)
    }
    #[doc = "Bit 6 - CCC_UPDATED_STAT_EN"]
    #[inline(always)]
    pub fn cccupdatedstaten(&mut self) -> CccupdatedstatenW<I3cd040Spec> {
        CccupdatedstatenW::new(self, 6)
    }
    #[doc = "Bit 8 - DYN_ADDR_ASSGN_STAT_EN"]
    #[inline(always)]
    pub fn dynaddrassgnstaten(&mut self) -> DynaddrassgnstatenW<I3cd040Spec> {
        DynaddrassgnstatenW::new(self, 8)
    }
    #[doc = "Bit 9 - TRANSFER_ERR_STAT_EN"]
    #[inline(always)]
    pub fn transfererrstaten(&mut self) -> TransfererrstatenW<I3cd040Spec> {
        TransfererrstatenW::new(self, 9)
    }
    #[doc = "Bit 10 - DEFSLV_STAT_EN"]
    #[inline(always)]
    pub fn defslvstaten(&mut self) -> DefslvstatenW<I3cd040Spec> {
        DefslvstatenW::new(self, 10)
    }
    #[doc = "Bit 11 - READ_REQ_RECV_STAT_EN"]
    #[inline(always)]
    pub fn readreqrecvstaten(&mut self) -> ReadreqrecvstatenW<I3cd040Spec> {
        ReadreqrecvstatenW::new(self, 11)
    }
    #[doc = "Bit 12 - IBI_UPDATED_STAT_EN"]
    #[inline(always)]
    pub fn ibiupdatedstaten(&mut self) -> IbiupdatedstatenW<I3cd040Spec> {
        IbiupdatedstatenW::new(self, 12)
    }
    #[doc = "Bit 13 - BUSOWNER_UPDATED_STAT_EN"]
    #[inline(always)]
    pub fn busownerupdatedstaten(&mut self) -> BusownerupdatedstatenW<I3cd040Spec> {
        BusownerupdatedstatenW::new(self, 13)
    }
    #[doc = "Bit 15 - BUS_RESET_DONE_STS_EN"]
    #[inline(always)]
    pub fn busresetdonestsen(&mut self) -> BusresetdonestsenW<I3cd040Spec> {
        BusresetdonestsenW::new(self, 15)
    }
}
#[doc = "Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd040Spec;
impl crate::RegisterSpec for I3cd040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd040::R`](R) reader structure"]
impl crate::Readable for I3cd040Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd040::W`](W) writer structure"]
impl crate::Writable for I3cd040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD040 to value 0"]
impl crate::Resettable for I3cd040Spec {}
