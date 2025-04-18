#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    scu000: Scu000,
    scu004: Scu004,
    _reserved2: [u8; 0x08],
    scu010: Scu010,
    scu014: Scu014,
    _reserved4: [u8; 0x28],
    scu040: Scu040,
    scu044: Scu044,
    _reserved6: [u8; 0x08],
    scu050: Scu050,
    scu054: Scu054,
    _reserved8: [u8; 0x08],
    scu060: Scu060,
    _reserved9: [u8; 0x0c],
    scu070: Scu070,
    scu074: Scu074,
    scu078: Scu078,
    _reserved12: [u8; 0x04],
    scu080: Scu080,
    scu084: Scu084,
    _reserved14: [u8; 0x08],
    scu090: Scu090,
    scu094: Scu094,
    _reserved16: [u8; 0x38],
    scu0d0: Scu0d0,
    scu0d4: Scu0d4,
    scu0d8: Scu0d8,
    _reserved19: [u8; 0x14],
    scu0f0: Scu0f0,
    _reserved20: [u8; 0x010c],
    scu200: Scu200,
    scu204: Scu204,
    _reserved22: [u8; 0x0108],
    scu310: Scu310,
    scu314: Scu314,
    _reserved24: [u8; 0x18],
    scu330: Scu330,
    scu334: Scu334,
    _reserved26: [u8; 0x18],
    scu350: Scu350,
    _reserved27: [u8; 0x04],
    scu358: Scu358,
    scu35c: Scu35c,
    _reserved29: [u8; 0x10],
    scu370: Scu370,
    _reserved30: [u8; 0x08],
    scu37c: Scu37c,
    _reserved31: [u8; 0x90],
    scu410: Scu410,
    scu414: Scu414,
    scu418: Scu418,
    scu41c: Scu41c,
    _reserved35: [u8; 0x10],
    scu430: Scu430,
    scu434: Scu434,
    scu438: Scu438,
    _reserved38: [u8; 0x14],
    scu450: Scu450,
    scu454: Scu454,
    scu458: Scu458,
    _reserved41: [u8; 0x54],
    scu4b0: Scu4b0,
    scu4b4: Scu4b4,
    scu4b8: Scu4b8,
    scu4bc: Scu4bc,
    _reserved45: [u8; 0x14],
    scu4d4: Scu4d4,
    scu4d8: Scu4d8,
    _reserved47: [u8; 0x18],
    scu4f4: Scu4f4,
    _reserved48: [u8; 0x08],
    scu500: Scu500,
    scu504: Scu504,
    scu508: Scu508,
    _reserved51: [u8; 0x04],
    scu510: Scu510,
    scu514: Scu514,
    scu518: Scu518,
    scu51c: Scu51c,
    _reserved55: [u8; 0x10],
    scu530: Scu530,
    scu534: Scu534,
    _reserved57: [u8; 0x58],
    scu590: Scu590,
    scu594: Scu594,
    _reserved59: [u8; 0x18],
    scu5b0: Scu5b0,
    scu5b4: Scu5b4,
    scu5b8: Scu5b8,
    scu5bc: Scu5bc,
    _reserved63: [u8; 0x10],
    scu5d0: Scu5d0,
    scu5d4: Scu5d4,
    _reserved65: [u8; 0x38],
    scu610: Scu610,
    scu614: Scu614,
    scu618: Scu618,
    scu61c: Scu61c,
    _reserved69: [u8; 0x10],
    scu630: Scu630,
    scu634: Scu634,
    scu638: Scu638,
    _reserved72: [u8; 0x14],
    scu650: Scu650,
    _reserved73: [u8; 0x3c],
    scu690: Scu690,
    scu694: Scu694,
    scu698: Scu698,
    scu69c: Scu69c,
    _reserved77: [u8; 0x10],
    scu6b0: Scu6b0,
    _reserved78: [u8; 0x04],
    scu6b8: Scu6b8,
    _reserved79: [u8; 0x14],
    scu6d0: Scu6d0,
    scu6d4: Scu6d4,
    _reserved81: [u8; 0x033c],
    scua14: Scua14,
    scua18: Scua18,
    scua1c: Scua1c,
    _reserved84: [u8; 0x30],
    scua50: Scua50,
    scua54: Scua54,
    scua58: Scua58,
    _reserved87: [u8; 0x04a4],
    scuf00: Scuf00,
    _reserved88: [u8; 0x04],
    scuf08: Scuf08,
    _reserved89: [u8; 0x04],
    scuf10: Scuf10,
    _reserved90: [u8; 0x04],
    scuf18: Scuf18,
    scuf1c: Scuf1c,
    _reserved92: [u8; 0x10],
    scuf30: Scuf30,
    scuf34: Scuf34,
    scuf38: Scuf38,
    _reserved95: [u8; 0x14],
    scuf50: Scuf50,
    _reserved96: [u8; 0x04],
    scuf58: Scuf58,
    _reserved97: [u8; 0x10],
    scuf6c: Scuf6c,
    _reserved98: [u8; 0x0c],
    scuf7c: Scuf7c,
    scuf80: Scuf80,
    _reserved100: [u8; 0x04],
    scuf88: Scuf88,
    _reserved101: [u8; 0x04],
    scuf90: Scuf90,
    _reserved102: [u8; 0x04],
    scuf98: Scuf98,
    scuf9c: Scuf9c,
    _reserved104: [u8; 0x10],
    scufb0: Scufb0,
    scufb4: Scufb4,
    scufb8: Scufb8,
    _reserved107: [u8; 0x14],
    scufd0: Scufd0,
    _reserved108: [u8; 0x18],
    scufec: Scufec,
    _reserved109: [u8; 0x0c],
    scuffc: Scuffc,
}
impl RegisterBlock {
    #[doc = "0x00 - Protection Key Register"]
    #[inline(always)]
    pub const fn scu000(&self) -> &Scu000 {
        &self.scu000
    }
    #[doc = "0x04 - Silicon Revision ID Register"]
    #[inline(always)]
    pub const fn scu004(&self) -> &Scu004 {
        &self.scu004
    }
    #[doc = "0x10 - Protection Key Register 2"]
    #[inline(always)]
    pub const fn scu010(&self) -> &Scu010 {
        &self.scu010
    }
    #[doc = "0x14 - Silicon Revision ID Register"]
    #[inline(always)]
    pub const fn scu014(&self) -> &Scu014 {
        &self.scu014
    }
    #[doc = "0x40 - System Reset Control Register"]
    #[inline(always)]
    pub const fn scu040(&self) -> &Scu040 {
        &self.scu040
    }
    #[doc = "0x44 - System Reset Control Clear Register 2"]
    #[inline(always)]
    pub const fn scu044(&self) -> &Scu044 {
        &self.scu044
    }
    #[doc = "0x50 - System Reset Control Register Set 2"]
    #[inline(always)]
    pub const fn scu050(&self) -> &Scu050 {
        &self.scu050
    }
    #[doc = "0x54 - System Reset Control Clear Register 2"]
    #[inline(always)]
    pub const fn scu054(&self) -> &Scu054 {
        &self.scu054
    }
    #[doc = "0x60 - EXTRST\\# Reset Selection"]
    #[inline(always)]
    pub const fn scu060(&self) -> &Scu060 {
        &self.scu060
    }
    #[doc = "0x70 - EXTRST\\# Reset Selection"]
    #[inline(always)]
    pub const fn scu070(&self) -> &Scu070 {
        &self.scu070
    }
    #[doc = "0x74 - System Reset Event Log Register Set 2-1"]
    #[inline(always)]
    pub const fn scu074(&self) -> &Scu074 {
        &self.scu074
    }
    #[doc = "0x78 - System Reset Event Log Register Set 2-2"]
    #[inline(always)]
    pub const fn scu078(&self) -> &Scu078 {
        &self.scu078
    }
    #[doc = "0x80 - Clock Stop Control Register"]
    #[inline(always)]
    pub const fn scu080(&self) -> &Scu080 {
        &self.scu080
    }
    #[doc = "0x84 - Clock Stop Control Clear Register"]
    #[inline(always)]
    pub const fn scu084(&self) -> &Scu084 {
        &self.scu084
    }
    #[doc = "0x90 - Clock Stop Control Register Set 2"]
    #[inline(always)]
    pub const fn scu090(&self) -> &Scu090 {
        &self.scu090
    }
    #[doc = "0x94 - Clock Stop Control Clear Register Set 2"]
    #[inline(always)]
    pub const fn scu094(&self) -> &Scu094 {
        &self.scu094
    }
    #[doc = "0xd0 - Misc. 3 Control Register\\label{SCUREG:MISC3"]
    #[inline(always)]
    pub const fn scu0d0(&self) -> &Scu0d0 {
        &self.scu0d0
    }
    #[doc = "0xd4 - Misc. 4 Control Register\\label{SCUREG:MISC4"]
    #[inline(always)]
    pub const fn scu0d4(&self) -> &Scu0d4 {
        &self.scu0d4
    }
    #[doc = "0xd8 - Debug Control Register 2"]
    #[inline(always)]
    pub const fn scu0d8(&self) -> &Scu0d8 {
        &self.scu0d8
    }
    #[doc = "0xf0 - QSPI Monitor Internal Mux Register"]
    #[inline(always)]
    pub const fn scu0f0(&self) -> &Scu0f0 {
        &self.scu0f0
    }
    #[doc = "0x200 - H-PLL Parameter Register"]
    #[inline(always)]
    pub const fn scu200(&self) -> &Scu200 {
        &self.scu200
    }
    #[doc = "0x204 - Extended H-PLL Parameter Register"]
    #[inline(always)]
    pub const fn scu204(&self) -> &Scu204 {
        &self.scu204
    }
    #[doc = "0x310 - Clock Selection Register Set 4"]
    #[inline(always)]
    pub const fn scu310(&self) -> &Scu310 {
        &self.scu310
    }
    #[doc = "0x314 - Clock Selection Register Set 5"]
    #[inline(always)]
    pub const fn scu314(&self) -> &Scu314 {
        &self.scu314
    }
    #[doc = "0x330 - Frequency Counter Control Register"]
    #[inline(always)]
    pub const fn scu330(&self) -> &Scu330 {
        &self.scu330
    }
    #[doc = "0x334 - Frequency counter comparison range"]
    #[inline(always)]
    pub const fn scu334(&self) -> &Scu334 {
        &self.scu334
    }
    #[doc = "0x350 - MAC Interface Clock Delay Setting"]
    #[inline(always)]
    pub const fn scu350(&self) -> &Scu350 {
        &self.scu350
    }
    #[doc = "0x358 - MAC Interface Clock Delay 100M Setting"]
    #[inline(always)]
    pub const fn scu358(&self) -> &Scu358 {
        &self.scu358
    }
    #[doc = "0x35c - MAC Interface Clock Delay 10M Setting"]
    #[inline(always)]
    pub const fn scu35c(&self) -> &Scu35c {
        &self.scu35c
    }
    #[doc = "0x370 - Clock Duty Measurement Control"]
    #[inline(always)]
    pub const fn scu370(&self) -> &Scu370 {
        &self.scu370
    }
    #[doc = "0x37c - Clock Duty Measurement Result 2"]
    #[inline(always)]
    pub const fn scu37c(&self) -> &Scu37c {
        &self.scu37c
    }
    #[doc = "0x410 - Multi-function Pin Control \\#1"]
    #[inline(always)]
    pub const fn scu410(&self) -> &Scu410 {
        &self.scu410
    }
    #[doc = "0x414 - Multi-function Pin Control \\#2"]
    #[inline(always)]
    pub const fn scu414(&self) -> &Scu414 {
        &self.scu414
    }
    #[doc = "0x418 - Multi-function Pin Control \\#3"]
    #[inline(always)]
    pub const fn scu418(&self) -> &Scu418 {
        &self.scu418
    }
    #[doc = "0x41c - Multi-function Pin Control \\#4"]
    #[inline(always)]
    pub const fn scu41c(&self) -> &Scu41c {
        &self.scu41c
    }
    #[doc = "0x430 - Multi-function Pin Control \\#5"]
    #[inline(always)]
    pub const fn scu430(&self) -> &Scu430 {
        &self.scu430
    }
    #[doc = "0x434 - Multi-function Pin Control \\#6"]
    #[inline(always)]
    pub const fn scu434(&self) -> &Scu434 {
        &self.scu434
    }
    #[doc = "0x438 - Multi-function Pin Control \\#7"]
    #[inline(always)]
    pub const fn scu438(&self) -> &Scu438 {
        &self.scu438
    }
    #[doc = "0x450 - Multi-function Pin Control \\#9"]
    #[inline(always)]
    pub const fn scu450(&self) -> &Scu450 {
        &self.scu450
    }
    #[doc = "0x454 - Multi-function Pin Control \\#10"]
    #[inline(always)]
    pub const fn scu454(&self) -> &Scu454 {
        &self.scu454
    }
    #[doc = "0x458 - Multi-function Pin Control \\#11"]
    #[inline(always)]
    pub const fn scu458(&self) -> &Scu458 {
        &self.scu458
    }
    #[doc = "0x4b0 - Multi-function Pin Control \\#13"]
    #[inline(always)]
    pub const fn scu4b0(&self) -> &Scu4b0 {
        &self.scu4b0
    }
    #[doc = "0x4b4 - Multi-function Pin Control \\#14"]
    #[inline(always)]
    pub const fn scu4b4(&self) -> &Scu4b4 {
        &self.scu4b4
    }
    #[doc = "0x4b8 - Multi-function Pin Control \\#15"]
    #[inline(always)]
    pub const fn scu4b8(&self) -> &Scu4b8 {
        &self.scu4b8
    }
    #[doc = "0x4bc - Multi-function Pin Control \\#16"]
    #[inline(always)]
    pub const fn scu4bc(&self) -> &Scu4bc {
        &self.scu4bc
    }
    #[doc = "0x4d4 - Multi-function Pin Control \\#18"]
    #[inline(always)]
    pub const fn scu4d4(&self) -> &Scu4d4 {
        &self.scu4d4
    }
    #[doc = "0x4d8 - Multi-function Pin Control \\#19"]
    #[inline(always)]
    pub const fn scu4d8(&self) -> &Scu4d8 {
        &self.scu4d8
    }
    #[doc = "0x4f4 - UART Debug interface Baud Rate Control"]
    #[inline(always)]
    pub const fn scu4f4(&self) -> &Scu4f4 {
        &self.scu4f4
    }
    #[doc = "0x500 - Hardware Strap1 Register\\label{SCUREG:HWTRAPL"]
    #[inline(always)]
    pub const fn scu500(&self) -> &Scu500 {
        &self.scu500
    }
    #[doc = "0x504 - Hardware Strap1 Clear Register"]
    #[inline(always)]
    pub const fn scu504(&self) -> &Scu504 {
        &self.scu504
    }
    #[doc = "0x508 - Hardware Strap1 Protect Register"]
    #[inline(always)]
    pub const fn scu508(&self) -> &Scu508 {
        &self.scu508
    }
    #[doc = "0x510 - Hardware Strap2 Register\\label{SCUREG:HWTRAPH"]
    #[inline(always)]
    pub const fn scu510(&self) -> &Scu510 {
        &self.scu510
    }
    #[doc = "0x514 - Hardware Strap2 Clear Register"]
    #[inline(always)]
    pub const fn scu514(&self) -> &Scu514 {
        &self.scu514
    }
    #[doc = "0x518 - Hardware Strap2 Protect Register"]
    #[inline(always)]
    pub const fn scu518(&self) -> &Scu518 {
        &self.scu518
    }
    #[doc = "0x51c - Hardware Pin Strap Register\\label{SCUREG:HWTRAPHRO"]
    #[inline(always)]
    pub const fn scu51c(&self) -> &Scu51c {
        &self.scu51c
    }
    #[doc = "0x530 - Random Number Generator 2 Control"]
    #[inline(always)]
    pub const fn scu530(&self) -> &Scu530 {
        &self.scu530
    }
    #[doc = "0x534 - Random Number Generator 2 Data Output"]
    #[inline(always)]
    pub const fn scu534(&self) -> &Scu534 {
        &self.scu534
    }
    #[doc = "0x590 - EFUSE Control"]
    #[inline(always)]
    pub const fn scu590(&self) -> &Scu590 {
        &self.scu590
    }
    #[doc = "0x594 - EFUSE Data Register"]
    #[inline(always)]
    pub const fn scu594(&self) -> &Scu594 {
        &self.scu594
    }
    #[doc = "0x5b0 - Chip Unique ID 0"]
    #[inline(always)]
    pub const fn scu5b0(&self) -> &Scu5b0 {
        &self.scu5b0
    }
    #[doc = "0x5b4 - Chip Unique ID 1"]
    #[inline(always)]
    pub const fn scu5b4(&self) -> &Scu5b4 {
        &self.scu5b4
    }
    #[doc = "0x5b8 - Reserved Read Only ID 0"]
    #[inline(always)]
    pub const fn scu5b8(&self) -> &Scu5b8 {
        &self.scu5b8
    }
    #[doc = "0x5bc - Reserved Read Only ID 1"]
    #[inline(always)]
    pub const fn scu5bc(&self) -> &Scu5bc {
        &self.scu5bc
    }
    #[doc = "0x5d0 - Reserved Read Only ID 2"]
    #[inline(always)]
    pub const fn scu5d0(&self) -> &Scu5d0 {
        &self.scu5d0
    }
    #[doc = "0x5d4 - Reserved Read Only ID 3"]
    #[inline(always)]
    pub const fn scu5d4(&self) -> &Scu5d4 {
        &self.scu5d4
    }
    #[doc = "0x610 - Disable GPIO Internal Pull-Down \\#0"]
    #[inline(always)]
    pub const fn scu610(&self) -> &Scu610 {
        &self.scu610
    }
    #[doc = "0x614 - Disable GPIO Internal Pull-Down \\#1"]
    #[inline(always)]
    pub const fn scu614(&self) -> &Scu614 {
        &self.scu614
    }
    #[doc = "0x618 - Disable GPIO Internal Pull-Down \\#2"]
    #[inline(always)]
    pub const fn scu618(&self) -> &Scu618 {
        &self.scu618
    }
    #[doc = "0x61c - Disable GPIO Internal Pull-Down \\#3"]
    #[inline(always)]
    pub const fn scu61c(&self) -> &Scu61c {
        &self.scu61c
    }
    #[doc = "0x630 - Disable GPIO Internal Pull-Down \\#4"]
    #[inline(always)]
    pub const fn scu630(&self) -> &Scu630 {
        &self.scu630
    }
    #[doc = "0x634 - Disable GPIO Internal Pull-Down \\#5"]
    #[inline(always)]
    pub const fn scu634(&self) -> &Scu634 {
        &self.scu634
    }
    #[doc = "0x638 - Disable GPIO Internal Pull-Down \\#6"]
    #[inline(always)]
    pub const fn scu638(&self) -> &Scu638 {
        &self.scu638
    }
    #[doc = "0x650 - IO Driving Strength"]
    #[inline(always)]
    pub const fn scu650(&self) -> &Scu650 {
        &self.scu650
    }
    #[doc = "0x690 - Multi-function Pin Control \\#21"]
    #[inline(always)]
    pub const fn scu690(&self) -> &Scu690 {
        &self.scu690
    }
    #[doc = "0x694 - Multi-function Pin Control \\#22"]
    #[inline(always)]
    pub const fn scu694(&self) -> &Scu694 {
        &self.scu694
    }
    #[doc = "0x698 - Multi-function Pin Control \\#23"]
    #[inline(always)]
    pub const fn scu698(&self) -> &Scu698 {
        &self.scu698
    }
    #[doc = "0x69c - Multi-function Pin Control \\#24"]
    #[inline(always)]
    pub const fn scu69c(&self) -> &Scu69c {
        &self.scu69c
    }
    #[doc = "0x6b0 - Multi-function Pin Control \\#25"]
    #[inline(always)]
    pub const fn scu6b0(&self) -> &Scu6b0 {
        &self.scu6b0
    }
    #[doc = "0x6b8 - Multi-function Pin Control \\#27"]
    #[inline(always)]
    pub const fn scu6b8(&self) -> &Scu6b8 {
        &self.scu6b8
    }
    #[doc = "0x6d0 - Multi-function Pin Control \\#29"]
    #[inline(always)]
    pub const fn scu6d0(&self) -> &Scu6d0 {
        &self.scu6d0
    }
    #[doc = "0x6d4 - Multi-function Pin Control \\#30"]
    #[inline(always)]
    pub const fn scu6d4(&self) -> &Scu6d4 {
        &self.scu6d4
    }
    #[doc = "0xa14 - CM4F Memory Base Address Register"]
    #[inline(always)]
    pub const fn scua14(&self) -> &Scua14 {
        &self.scua14
    }
    #[doc = "0xa18 - CM4F Instruction Memory Address Limit Register"]
    #[inline(always)]
    pub const fn scua18(&self) -> &Scua18 {
        &self.scua18
    }
    #[doc = "0xa1c - CM4F Data Memory Address Limit Register"]
    #[inline(always)]
    pub const fn scua1c(&self) -> &Scua1c {
        &self.scua1c
    }
    #[doc = "0xa50 - CM4F Cacheable Area Declaration"]
    #[inline(always)]
    pub const fn scua50(&self) -> &Scua50 {
        &self.scua50
    }
    #[doc = "0xa54 - CM4F Cache Invalidation Control Register"]
    #[inline(always)]
    pub const fn scua54(&self) -> &Scua54 {
        &self.scua54
    }
    #[doc = "0xa58 - CM4F Cache Function Control Register"]
    #[inline(always)]
    pub const fn scua58(&self) -> &Scua58 {
        &self.scua58
    }
    #[doc = "0xf00 - Write Protect Register \\#1"]
    #[inline(always)]
    pub const fn scuf00(&self) -> &Scuf00 {
        &self.scuf00
    }
    #[doc = "0xf08 - Write Protect Register \\#3"]
    #[inline(always)]
    pub const fn scuf08(&self) -> &Scuf08 {
        &self.scuf08
    }
    #[doc = "0xf10 - Write Protect Register \\#5"]
    #[inline(always)]
    pub const fn scuf10(&self) -> &Scuf10 {
        &self.scuf10
    }
    #[doc = "0xf18 - Write Protect Register \\#7"]
    #[inline(always)]
    pub const fn scuf18(&self) -> &Scuf18 {
        &self.scuf18
    }
    #[doc = "0xf1c - Write Protect Register \\#8"]
    #[inline(always)]
    pub const fn scuf1c(&self) -> &Scuf1c {
        &self.scuf1c
    }
    #[doc = "0xf30 - Write Protect Register \\#13"]
    #[inline(always)]
    pub const fn scuf30(&self) -> &Scuf30 {
        &self.scuf30
    }
    #[doc = "0xf34 - Write Protect Register \\#14"]
    #[inline(always)]
    pub const fn scuf34(&self) -> &Scuf34 {
        &self.scuf34
    }
    #[doc = "0xf38 - Write Protect Register \\#15"]
    #[inline(always)]
    pub const fn scuf38(&self) -> &Scuf38 {
        &self.scuf38
    }
    #[doc = "0xf50 - Write Protect Register \\#21"]
    #[inline(always)]
    pub const fn scuf50(&self) -> &Scuf50 {
        &self.scuf50
    }
    #[doc = "0xf58 - Write Protect Register \\#23"]
    #[inline(always)]
    pub const fn scuf58(&self) -> &Scuf58 {
        &self.scuf58
    }
    #[doc = "0xf6c - Write Protect Register \\#28"]
    #[inline(always)]
    pub const fn scuf6c(&self) -> &Scuf6c {
        &self.scuf6c
    }
    #[doc = "0xf7c - Write Protect Register \\#32"]
    #[inline(always)]
    pub const fn scuf7c(&self) -> &Scuf7c {
        &self.scuf7c
    }
    #[doc = "0xf80 - Reset Source Control Register \\#1"]
    #[inline(always)]
    pub const fn scuf80(&self) -> &Scuf80 {
        &self.scuf80
    }
    #[doc = "0xf88 - Reset Source Control Register \\#3"]
    #[inline(always)]
    pub const fn scuf88(&self) -> &Scuf88 {
        &self.scuf88
    }
    #[doc = "0xf90 - Reset Source Control Register \\#5"]
    #[inline(always)]
    pub const fn scuf90(&self) -> &Scuf90 {
        &self.scuf90
    }
    #[doc = "0xf98 - Reset Source Control Register \\#7"]
    #[inline(always)]
    pub const fn scuf98(&self) -> &Scuf98 {
        &self.scuf98
    }
    #[doc = "0xf9c - Reset Source Control Register \\#8"]
    #[inline(always)]
    pub const fn scuf9c(&self) -> &Scuf9c {
        &self.scuf9c
    }
    #[doc = "0xfb0 - Reset Source Control Register \\#13"]
    #[inline(always)]
    pub const fn scufb0(&self) -> &Scufb0 {
        &self.scufb0
    }
    #[doc = "0xfb4 - Reset Source Control Register \\#14"]
    #[inline(always)]
    pub const fn scufb4(&self) -> &Scufb4 {
        &self.scufb4
    }
    #[doc = "0xfb8 - Reset Source Control Register \\#15"]
    #[inline(always)]
    pub const fn scufb8(&self) -> &Scufb8 {
        &self.scufb8
    }
    #[doc = "0xfd0 - Reset Source Control Register \\#21"]
    #[inline(always)]
    pub const fn scufd0(&self) -> &Scufd0 {
        &self.scufd0
    }
    #[doc = "0xfec - Reset Source Control Register \\#28"]
    #[inline(always)]
    pub const fn scufec(&self) -> &Scufec {
        &self.scufec
    }
    #[doc = "0xffc - Reset Source Control Register \\#32"]
    #[inline(always)]
    pub const fn scuffc(&self) -> &Scuffc {
        &self.scuffc
    }
}
#[doc = "SCU000 (rw) register accessor: Protection Key Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu000`] module"]
#[doc(alias = "SCU000")]
pub type Scu000 = crate::Reg<scu000::Scu000Spec>;
#[doc = "Protection Key Register"]
pub mod scu000;
#[doc = "SCU010 (rw) register accessor: Protection Key Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu010`] module"]
#[doc(alias = "SCU010")]
pub type Scu010 = crate::Reg<scu010::Scu010Spec>;
#[doc = "Protection Key Register 2"]
pub mod scu010;
#[doc = "SCU004 (rw) register accessor: Silicon Revision ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu004`] module"]
#[doc(alias = "SCU004")]
pub type Scu004 = crate::Reg<scu004::Scu004Spec>;
#[doc = "Silicon Revision ID Register"]
pub mod scu004;
#[doc = "SCU014 (rw) register accessor: Silicon Revision ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu014`] module"]
#[doc(alias = "SCU014")]
pub type Scu014 = crate::Reg<scu014::Scu014Spec>;
#[doc = "Silicon Revision ID Register"]
pub mod scu014;
#[doc = "SCU040 (rw) register accessor: System Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu040`] module"]
#[doc(alias = "SCU040")]
pub type Scu040 = crate::Reg<scu040::Scu040Spec>;
#[doc = "System Reset Control Register"]
pub mod scu040;
#[doc = "SCU044 (rw) register accessor: System Reset Control Clear Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu044`] module"]
#[doc(alias = "SCU044")]
pub type Scu044 = crate::Reg<scu044::Scu044Spec>;
#[doc = "System Reset Control Clear Register 2"]
pub mod scu044;
#[doc = "SCU050 (rw) register accessor: System Reset Control Register Set 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu050`] module"]
#[doc(alias = "SCU050")]
pub type Scu050 = crate::Reg<scu050::Scu050Spec>;
#[doc = "System Reset Control Register Set 2"]
pub mod scu050;
#[doc = "SCU054 (rw) register accessor: System Reset Control Clear Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu054`] module"]
#[doc(alias = "SCU054")]
pub type Scu054 = crate::Reg<scu054::Scu054Spec>;
#[doc = "System Reset Control Clear Register 2"]
pub mod scu054;
#[doc = "SCU060 (rw) register accessor: EXTRST\\# Reset Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`scu060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu060`] module"]
#[doc(alias = "SCU060")]
pub type Scu060 = crate::Reg<scu060::Scu060Spec>;
#[doc = "EXTRST\\# Reset Selection"]
pub mod scu060;
#[doc = "SCU070 (rw) register accessor: EXTRST\\# Reset Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`scu070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu070`] module"]
#[doc(alias = "SCU070")]
pub type Scu070 = crate::Reg<scu070::Scu070Spec>;
#[doc = "EXTRST\\# Reset Selection"]
pub mod scu070;
#[doc = "SCU074 (rw) register accessor: System Reset Event Log Register Set 2-1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu074`] module"]
#[doc(alias = "SCU074")]
pub type Scu074 = crate::Reg<scu074::Scu074Spec>;
#[doc = "System Reset Event Log Register Set 2-1"]
pub mod scu074;
#[doc = "SCU078 (rw) register accessor: System Reset Event Log Register Set 2-2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu078`] module"]
#[doc(alias = "SCU078")]
pub type Scu078 = crate::Reg<scu078::Scu078Spec>;
#[doc = "System Reset Event Log Register Set 2-2"]
pub mod scu078;
#[doc = "SCU080 (rw) register accessor: Clock Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu080`] module"]
#[doc(alias = "SCU080")]
pub type Scu080 = crate::Reg<scu080::Scu080Spec>;
#[doc = "Clock Stop Control Register"]
pub mod scu080;
#[doc = "SCU084 (rw) register accessor: Clock Stop Control Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu084`] module"]
#[doc(alias = "SCU084")]
pub type Scu084 = crate::Reg<scu084::Scu084Spec>;
#[doc = "Clock Stop Control Clear Register"]
pub mod scu084;
#[doc = "SCU090 (rw) register accessor: Clock Stop Control Register Set 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu090`] module"]
#[doc(alias = "SCU090")]
pub type Scu090 = crate::Reg<scu090::Scu090Spec>;
#[doc = "Clock Stop Control Register Set 2"]
pub mod scu090;
#[doc = "SCU094 (rw) register accessor: Clock Stop Control Clear Register Set 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu094`] module"]
#[doc(alias = "SCU094")]
pub type Scu094 = crate::Reg<scu094::Scu094Spec>;
#[doc = "Clock Stop Control Clear Register Set 2"]
pub mod scu094;
#[doc = "SCU0D0 (rw) register accessor: Misc. 3 Control Register\\label{SCUREG:MISC3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0d0`] module"]
#[doc(alias = "SCU0D0")]
pub type Scu0d0 = crate::Reg<scu0d0::Scu0d0Spec>;
#[doc = "Misc. 3 Control Register\\label{SCUREG:MISC3"]
pub mod scu0d0;
#[doc = "SCU0D4 (rw) register accessor: Misc. 4 Control Register\\label{SCUREG:MISC4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0d4`] module"]
#[doc(alias = "SCU0D4")]
pub type Scu0d4 = crate::Reg<scu0d4::Scu0d4Spec>;
#[doc = "Misc. 4 Control Register\\label{SCUREG:MISC4"]
pub mod scu0d4;
#[doc = "SCU0D8 (rw) register accessor: Debug Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0d8`] module"]
#[doc(alias = "SCU0D8")]
pub type Scu0d8 = crate::Reg<scu0d8::Scu0d8Spec>;
#[doc = "Debug Control Register 2"]
pub mod scu0d8;
#[doc = "SCU0F0 (rw) register accessor: QSPI Monitor Internal Mux Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu0f0`] module"]
#[doc(alias = "SCU0F0")]
pub type Scu0f0 = crate::Reg<scu0f0::Scu0f0Spec>;
#[doc = "QSPI Monitor Internal Mux Register"]
pub mod scu0f0;
#[doc = "SCU200 (rw) register accessor: H-PLL Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu200`] module"]
#[doc(alias = "SCU200")]
pub type Scu200 = crate::Reg<scu200::Scu200Spec>;
#[doc = "H-PLL Parameter Register"]
pub mod scu200;
#[doc = "SCU204 (rw) register accessor: Extended H-PLL Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu204`] module"]
#[doc(alias = "SCU204")]
pub type Scu204 = crate::Reg<scu204::Scu204Spec>;
#[doc = "Extended H-PLL Parameter Register"]
pub mod scu204;
#[doc = "SCU310 (rw) register accessor: Clock Selection Register Set 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu310`] module"]
#[doc(alias = "SCU310")]
pub type Scu310 = crate::Reg<scu310::Scu310Spec>;
#[doc = "Clock Selection Register Set 4"]
pub mod scu310;
#[doc = "SCU314 (rw) register accessor: Clock Selection Register Set 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu314`] module"]
#[doc(alias = "SCU314")]
pub type Scu314 = crate::Reg<scu314::Scu314Spec>;
#[doc = "Clock Selection Register Set 5"]
pub mod scu314;
#[doc = "SCU330 (rw) register accessor: Frequency Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu330::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu330::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu330`] module"]
#[doc(alias = "SCU330")]
pub type Scu330 = crate::Reg<scu330::Scu330Spec>;
#[doc = "Frequency Counter Control Register"]
pub mod scu330;
#[doc = "SCU334 (rw) register accessor: Frequency counter comparison range\n\nYou can [`read`](crate::Reg::read) this register and get [`scu334::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu334::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu334`] module"]
#[doc(alias = "SCU334")]
pub type Scu334 = crate::Reg<scu334::Scu334Spec>;
#[doc = "Frequency counter comparison range"]
pub mod scu334;
#[doc = "SCU350 (rw) register accessor: MAC Interface Clock Delay Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu350::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu350::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu350`] module"]
#[doc(alias = "SCU350")]
pub type Scu350 = crate::Reg<scu350::Scu350Spec>;
#[doc = "MAC Interface Clock Delay Setting"]
pub mod scu350;
#[doc = "SCU358 (rw) register accessor: MAC Interface Clock Delay 100M Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu358::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu358::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu358`] module"]
#[doc(alias = "SCU358")]
pub type Scu358 = crate::Reg<scu358::Scu358Spec>;
#[doc = "MAC Interface Clock Delay 100M Setting"]
pub mod scu358;
#[doc = "SCU35C (rw) register accessor: MAC Interface Clock Delay 10M Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu35c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu35c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu35c`] module"]
#[doc(alias = "SCU35C")]
pub type Scu35c = crate::Reg<scu35c::Scu35cSpec>;
#[doc = "MAC Interface Clock Delay 10M Setting"]
pub mod scu35c;
#[doc = "SCU370 (rw) register accessor: Clock Duty Measurement Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scu370::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu370::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu370`] module"]
#[doc(alias = "SCU370")]
pub type Scu370 = crate::Reg<scu370::Scu370Spec>;
#[doc = "Clock Duty Measurement Control"]
pub mod scu370;
#[doc = "SCU37C (rw) register accessor: Clock Duty Measurement Result 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu37c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu37c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu37c`] module"]
#[doc(alias = "SCU37C")]
pub type Scu37c = crate::Reg<scu37c::Scu37cSpec>;
#[doc = "Clock Duty Measurement Result 2"]
pub mod scu37c;
#[doc = "SCU410 (rw) register accessor: Multi-function Pin Control \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu410::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu410::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu410`] module"]
#[doc(alias = "SCU410")]
pub type Scu410 = crate::Reg<scu410::Scu410Spec>;
#[doc = "Multi-function Pin Control \\#1"]
pub mod scu410;
#[doc = "SCU414 (rw) register accessor: Multi-function Pin Control \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu414::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu414::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu414`] module"]
#[doc(alias = "SCU414")]
pub type Scu414 = crate::Reg<scu414::Scu414Spec>;
#[doc = "Multi-function Pin Control \\#2"]
pub mod scu414;
#[doc = "SCU418 (rw) register accessor: Multi-function Pin Control \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu418::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu418::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu418`] module"]
#[doc(alias = "SCU418")]
pub type Scu418 = crate::Reg<scu418::Scu418Spec>;
#[doc = "Multi-function Pin Control \\#3"]
pub mod scu418;
#[doc = "SCU41C (rw) register accessor: Multi-function Pin Control \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu41c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu41c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu41c`] module"]
#[doc(alias = "SCU41C")]
pub type Scu41c = crate::Reg<scu41c::Scu41cSpec>;
#[doc = "Multi-function Pin Control \\#4"]
pub mod scu41c;
#[doc = "SCU430 (rw) register accessor: Multi-function Pin Control \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu430::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu430::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu430`] module"]
#[doc(alias = "SCU430")]
pub type Scu430 = crate::Reg<scu430::Scu430Spec>;
#[doc = "Multi-function Pin Control \\#5"]
pub mod scu430;
#[doc = "SCU434 (rw) register accessor: Multi-function Pin Control \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu434::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu434::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu434`] module"]
#[doc(alias = "SCU434")]
pub type Scu434 = crate::Reg<scu434::Scu434Spec>;
#[doc = "Multi-function Pin Control \\#6"]
pub mod scu434;
#[doc = "SCU438 (rw) register accessor: Multi-function Pin Control \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu438::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu438::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu438`] module"]
#[doc(alias = "SCU438")]
pub type Scu438 = crate::Reg<scu438::Scu438Spec>;
#[doc = "Multi-function Pin Control \\#7"]
pub mod scu438;
#[doc = "SCU450 (rw) register accessor: Multi-function Pin Control \\#9\n\nYou can [`read`](crate::Reg::read) this register and get [`scu450::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu450::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu450`] module"]
#[doc(alias = "SCU450")]
pub type Scu450 = crate::Reg<scu450::Scu450Spec>;
#[doc = "Multi-function Pin Control \\#9"]
pub mod scu450;
#[doc = "SCU454 (rw) register accessor: Multi-function Pin Control \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`scu454::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu454::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu454`] module"]
#[doc(alias = "SCU454")]
pub type Scu454 = crate::Reg<scu454::Scu454Spec>;
#[doc = "Multi-function Pin Control \\#10"]
pub mod scu454;
#[doc = "SCU458 (rw) register accessor: Multi-function Pin Control \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu458::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu458::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu458`] module"]
#[doc(alias = "SCU458")]
pub type Scu458 = crate::Reg<scu458::Scu458Spec>;
#[doc = "Multi-function Pin Control \\#11"]
pub mod scu458;
#[doc = "SCU4B0 (rw) register accessor: Multi-function Pin Control \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4b0`] module"]
#[doc(alias = "SCU4B0")]
pub type Scu4b0 = crate::Reg<scu4b0::Scu4b0Spec>;
#[doc = "Multi-function Pin Control \\#13"]
pub mod scu4b0;
#[doc = "SCU4B4 (rw) register accessor: Multi-function Pin Control \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4b4`] module"]
#[doc(alias = "SCU4B4")]
pub type Scu4b4 = crate::Reg<scu4b4::Scu4b4Spec>;
#[doc = "Multi-function Pin Control \\#14"]
pub mod scu4b4;
#[doc = "SCU4B8 (rw) register accessor: Multi-function Pin Control \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4b8`] module"]
#[doc(alias = "SCU4B8")]
pub type Scu4b8 = crate::Reg<scu4b8::Scu4b8Spec>;
#[doc = "Multi-function Pin Control \\#15"]
pub mod scu4b8;
#[doc = "SCU4BC (rw) register accessor: Multi-function Pin Control \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4bc`] module"]
#[doc(alias = "SCU4BC")]
pub type Scu4bc = crate::Reg<scu4bc::Scu4bcSpec>;
#[doc = "Multi-function Pin Control \\#16"]
pub mod scu4bc;
#[doc = "SCU4D4 (rw) register accessor: Multi-function Pin Control \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4d4`] module"]
#[doc(alias = "SCU4D4")]
pub type Scu4d4 = crate::Reg<scu4d4::Scu4d4Spec>;
#[doc = "Multi-function Pin Control \\#18"]
pub mod scu4d4;
#[doc = "SCU4D8 (rw) register accessor: Multi-function Pin Control \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4d8`] module"]
#[doc(alias = "SCU4D8")]
pub type Scu4d8 = crate::Reg<scu4d8::Scu4d8Spec>;
#[doc = "Multi-function Pin Control \\#19"]
pub mod scu4d8;
#[doc = "SCU4F4 (rw) register accessor: UART Debug interface Baud Rate Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu4f4`] module"]
#[doc(alias = "SCU4F4")]
pub type Scu4f4 = crate::Reg<scu4f4::Scu4f4Spec>;
#[doc = "UART Debug interface Baud Rate Control"]
pub mod scu4f4;
#[doc = "SCU690 (rw) register accessor: Multi-function Pin Control \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scu690::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu690::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu690`] module"]
#[doc(alias = "SCU690")]
pub type Scu690 = crate::Reg<scu690::Scu690Spec>;
#[doc = "Multi-function Pin Control \\#21"]
pub mod scu690;
#[doc = "SCU694 (rw) register accessor: Multi-function Pin Control \\#22\n\nYou can [`read`](crate::Reg::read) this register and get [`scu694::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu694::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu694`] module"]
#[doc(alias = "SCU694")]
pub type Scu694 = crate::Reg<scu694::Scu694Spec>;
#[doc = "Multi-function Pin Control \\#22"]
pub mod scu694;
#[doc = "SCU698 (rw) register accessor: Multi-function Pin Control \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`scu698::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu698::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu698`] module"]
#[doc(alias = "SCU698")]
pub type Scu698 = crate::Reg<scu698::Scu698Spec>;
#[doc = "Multi-function Pin Control \\#23"]
pub mod scu698;
#[doc = "SCU69C (rw) register accessor: Multi-function Pin Control \\#24\n\nYou can [`read`](crate::Reg::read) this register and get [`scu69c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu69c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu69c`] module"]
#[doc(alias = "SCU69C")]
pub type Scu69c = crate::Reg<scu69c::Scu69cSpec>;
#[doc = "Multi-function Pin Control \\#24"]
pub mod scu69c;
#[doc = "SCU6B0 (rw) register accessor: Multi-function Pin Control \\#25\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu6b0`] module"]
#[doc(alias = "SCU6B0")]
pub type Scu6b0 = crate::Reg<scu6b0::Scu6b0Spec>;
#[doc = "Multi-function Pin Control \\#25"]
pub mod scu6b0;
#[doc = "SCU6B8 (rw) register accessor: Multi-function Pin Control \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu6b8`] module"]
#[doc(alias = "SCU6B8")]
pub type Scu6b8 = crate::Reg<scu6b8::Scu6b8Spec>;
#[doc = "Multi-function Pin Control \\#27"]
pub mod scu6b8;
#[doc = "SCU6D0 (rw) register accessor: Multi-function Pin Control \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu6d0`] module"]
#[doc(alias = "SCU6D0")]
pub type Scu6d0 = crate::Reg<scu6d0::Scu6d0Spec>;
#[doc = "Multi-function Pin Control \\#29"]
pub mod scu6d0;
#[doc = "SCU6D4 (rw) register accessor: Multi-function Pin Control \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu6d4`] module"]
#[doc(alias = "SCU6D4")]
pub type Scu6d4 = crate::Reg<scu6d4::Scu6d4Spec>;
#[doc = "Multi-function Pin Control \\#30"]
pub mod scu6d4;
#[doc = "SCU500 (rw) register accessor: Hardware Strap1 Register\\label{SCUREG:HWTRAPL\n\nYou can [`read`](crate::Reg::read) this register and get [`scu500::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu500::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu500`] module"]
#[doc(alias = "SCU500")]
pub type Scu500 = crate::Reg<scu500::Scu500Spec>;
#[doc = "Hardware Strap1 Register\\label{SCUREG:HWTRAPL"]
pub mod scu500;
#[doc = "SCU504 (rw) register accessor: Hardware Strap1 Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu504::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu504::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu504`] module"]
#[doc(alias = "SCU504")]
pub type Scu504 = crate::Reg<scu504::Scu504Spec>;
#[doc = "Hardware Strap1 Clear Register"]
pub mod scu504;
#[doc = "SCU508 (rw) register accessor: Hardware Strap1 Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu508::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu508::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu508`] module"]
#[doc(alias = "SCU508")]
pub type Scu508 = crate::Reg<scu508::Scu508Spec>;
#[doc = "Hardware Strap1 Protect Register"]
pub mod scu508;
#[doc = "SCU510 (rw) register accessor: Hardware Strap2 Register\\label{SCUREG:HWTRAPH\n\nYou can [`read`](crate::Reg::read) this register and get [`scu510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu510`] module"]
#[doc(alias = "SCU510")]
pub type Scu510 = crate::Reg<scu510::Scu510Spec>;
#[doc = "Hardware Strap2 Register\\label{SCUREG:HWTRAPH"]
pub mod scu510;
#[doc = "SCU514 (rw) register accessor: Hardware Strap2 Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu514::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu514::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu514`] module"]
#[doc(alias = "SCU514")]
pub type Scu514 = crate::Reg<scu514::Scu514Spec>;
#[doc = "Hardware Strap2 Clear Register"]
pub mod scu514;
#[doc = "SCU518 (rw) register accessor: Hardware Strap2 Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu518::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu518::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu518`] module"]
#[doc(alias = "SCU518")]
pub type Scu518 = crate::Reg<scu518::Scu518Spec>;
#[doc = "Hardware Strap2 Protect Register"]
pub mod scu518;
#[doc = "SCU51C (rw) register accessor: Hardware Pin Strap Register\\label{SCUREG:HWTRAPHRO\n\nYou can [`read`](crate::Reg::read) this register and get [`scu51c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu51c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu51c`] module"]
#[doc(alias = "SCU51C")]
pub type Scu51c = crate::Reg<scu51c::Scu51cSpec>;
#[doc = "Hardware Pin Strap Register\\label{SCUREG:HWTRAPHRO"]
pub mod scu51c;
#[doc = "SCU530 (rw) register accessor: Random Number Generator 2 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scu530::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu530::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu530`] module"]
#[doc(alias = "SCU530")]
pub type Scu530 = crate::Reg<scu530::Scu530Spec>;
#[doc = "Random Number Generator 2 Control"]
pub mod scu530;
#[doc = "SCU534 (rw) register accessor: Random Number Generator 2 Data Output\n\nYou can [`read`](crate::Reg::read) this register and get [`scu534::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu534::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu534`] module"]
#[doc(alias = "SCU534")]
pub type Scu534 = crate::Reg<scu534::Scu534Spec>;
#[doc = "Random Number Generator 2 Data Output"]
pub mod scu534;
#[doc = "SCU590 (rw) register accessor: EFUSE Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scu590::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu590::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu590`] module"]
#[doc(alias = "SCU590")]
pub type Scu590 = crate::Reg<scu590::Scu590Spec>;
#[doc = "EFUSE Control"]
pub mod scu590;
#[doc = "SCU594 (rw) register accessor: EFUSE Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu594::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu594::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu594`] module"]
#[doc(alias = "SCU594")]
pub type Scu594 = crate::Reg<scu594::Scu594Spec>;
#[doc = "EFUSE Data Register"]
pub mod scu594;
#[doc = "SCU5B0 (rw) register accessor: Chip Unique ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5b0`] module"]
#[doc(alias = "SCU5B0")]
pub type Scu5b0 = crate::Reg<scu5b0::Scu5b0Spec>;
#[doc = "Chip Unique ID 0"]
pub mod scu5b0;
#[doc = "SCU5B4 (rw) register accessor: Chip Unique ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5b4`] module"]
#[doc(alias = "SCU5B4")]
pub type Scu5b4 = crate::Reg<scu5b4::Scu5b4Spec>;
#[doc = "Chip Unique ID 1"]
pub mod scu5b4;
#[doc = "SCU5B8 (rw) register accessor: Reserved Read Only ID 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5b8`] module"]
#[doc(alias = "SCU5B8")]
pub type Scu5b8 = crate::Reg<scu5b8::Scu5b8Spec>;
#[doc = "Reserved Read Only ID 0"]
pub mod scu5b8;
#[doc = "SCU5BC (rw) register accessor: Reserved Read Only ID 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5bc`] module"]
#[doc(alias = "SCU5BC")]
pub type Scu5bc = crate::Reg<scu5bc::Scu5bcSpec>;
#[doc = "Reserved Read Only ID 1"]
pub mod scu5bc;
#[doc = "SCU5D0 (rw) register accessor: Reserved Read Only ID 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5d0`] module"]
#[doc(alias = "SCU5D0")]
pub type Scu5d0 = crate::Reg<scu5d0::Scu5d0Spec>;
#[doc = "Reserved Read Only ID 2"]
pub mod scu5d0;
#[doc = "SCU5D4 (rw) register accessor: Reserved Read Only ID 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu5d4`] module"]
#[doc(alias = "SCU5D4")]
pub type Scu5d4 = crate::Reg<scu5d4::Scu5d4Spec>;
#[doc = "Reserved Read Only ID 3"]
pub mod scu5d4;
#[doc = "SCU610 (rw) register accessor: Disable GPIO Internal Pull-Down \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu610::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu610::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu610`] module"]
#[doc(alias = "SCU610")]
pub type Scu610 = crate::Reg<scu610::Scu610Spec>;
#[doc = "Disable GPIO Internal Pull-Down \\#0"]
pub mod scu610;
#[doc = "SCU614 (rw) register accessor: Disable GPIO Internal Pull-Down \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu614::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu614::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu614`] module"]
#[doc(alias = "SCU614")]
pub type Scu614 = crate::Reg<scu614::Scu614Spec>;
#[doc = "Disable GPIO Internal Pull-Down \\#1"]
pub mod scu614;
#[doc = "SCU618 (rw) register accessor: Disable GPIO Internal Pull-Down \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu618::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu618::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu618`] module"]
#[doc(alias = "SCU618")]
pub type Scu618 = crate::Reg<scu618::Scu618Spec>;
#[doc = "Disable GPIO Internal Pull-Down \\#2"]
pub mod scu618;
#[doc = "SCU61C (rw) register accessor: Disable GPIO Internal Pull-Down \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu61c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu61c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu61c`] module"]
#[doc(alias = "SCU61C")]
pub type Scu61c = crate::Reg<scu61c::Scu61cSpec>;
#[doc = "Disable GPIO Internal Pull-Down \\#3"]
pub mod scu61c;
#[doc = "SCU630 (rw) register accessor: Disable GPIO Internal Pull-Down \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu630::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu630::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu630`] module"]
#[doc(alias = "SCU630")]
pub type Scu630 = crate::Reg<scu630::Scu630Spec>;
#[doc = "Disable GPIO Internal Pull-Down \\#4"]
pub mod scu630;
#[doc = "SCU634 (rw) register accessor: Disable GPIO Internal Pull-Down \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu634::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu634::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu634`] module"]
#[doc(alias = "SCU634")]
pub type Scu634 = crate::Reg<scu634::Scu634Spec>;
#[doc = "Disable GPIO Internal Pull-Down \\#5"]
pub mod scu634;
#[doc = "SCU638 (rw) register accessor: Disable GPIO Internal Pull-Down \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`scu638::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu638::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu638`] module"]
#[doc(alias = "SCU638")]
pub type Scu638 = crate::Reg<scu638::Scu638Spec>;
#[doc = "Disable GPIO Internal Pull-Down \\#6"]
pub mod scu638;
#[doc = "SCU650 (rw) register accessor: IO Driving Strength\n\nYou can [`read`](crate::Reg::read) this register and get [`scu650::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu650::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scu650`] module"]
#[doc(alias = "SCU650")]
pub type Scu650 = crate::Reg<scu650::Scu650Spec>;
#[doc = "IO Driving Strength"]
pub mod scu650;
#[doc = "SCUA14 (rw) register accessor: CM4F Memory Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scua14`] module"]
#[doc(alias = "SCUA14")]
pub type Scua14 = crate::Reg<scua14::Scua14Spec>;
#[doc = "CM4F Memory Base Address Register"]
pub mod scua14;
#[doc = "SCUA18 (rw) register accessor: CM4F Instruction Memory Address Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scua18`] module"]
#[doc(alias = "SCUA18")]
pub type Scua18 = crate::Reg<scua18::Scua18Spec>;
#[doc = "CM4F Instruction Memory Address Limit Register"]
pub mod scua18;
#[doc = "SCUA1C (rw) register accessor: CM4F Data Memory Address Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scua1c`] module"]
#[doc(alias = "SCUA1C")]
pub type Scua1c = crate::Reg<scua1c::Scua1cSpec>;
#[doc = "CM4F Data Memory Address Limit Register"]
pub mod scua1c;
#[doc = "SCUA50 (rw) register accessor: CM4F Cacheable Area Declaration\n\nYou can [`read`](crate::Reg::read) this register and get [`scua50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scua50`] module"]
#[doc(alias = "SCUA50")]
pub type Scua50 = crate::Reg<scua50::Scua50Spec>;
#[doc = "CM4F Cacheable Area Declaration"]
pub mod scua50;
#[doc = "SCUA54 (rw) register accessor: CM4F Cache Invalidation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scua54`] module"]
#[doc(alias = "SCUA54")]
pub type Scua54 = crate::Reg<scua54::Scua54Spec>;
#[doc = "CM4F Cache Invalidation Control Register"]
pub mod scua54;
#[doc = "SCUA58 (rw) register accessor: CM4F Cache Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scua58`] module"]
#[doc(alias = "SCUA58")]
pub type Scua58 = crate::Reg<scua58::Scua58Spec>;
#[doc = "CM4F Cache Function Control Register"]
pub mod scua58;
#[doc = "SCUF00 (rw) register accessor: Write Protect Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf00`] module"]
#[doc(alias = "SCUF00")]
pub type Scuf00 = crate::Reg<scuf00::Scuf00Spec>;
#[doc = "Write Protect Register \\#1"]
pub mod scuf00;
#[doc = "SCUF08 (rw) register accessor: Write Protect Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf08`] module"]
#[doc(alias = "SCUF08")]
pub type Scuf08 = crate::Reg<scuf08::Scuf08Spec>;
#[doc = "Write Protect Register \\#3"]
pub mod scuf08;
#[doc = "SCUF10 (rw) register accessor: Write Protect Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf10`] module"]
#[doc(alias = "SCUF10")]
pub type Scuf10 = crate::Reg<scuf10::Scuf10Spec>;
#[doc = "Write Protect Register \\#5"]
pub mod scuf10;
#[doc = "SCUF18 (rw) register accessor: Write Protect Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf18`] module"]
#[doc(alias = "SCUF18")]
pub type Scuf18 = crate::Reg<scuf18::Scuf18Spec>;
#[doc = "Write Protect Register \\#7"]
pub mod scuf18;
#[doc = "SCUF1C (rw) register accessor: Write Protect Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf1c`] module"]
#[doc(alias = "SCUF1C")]
pub type Scuf1c = crate::Reg<scuf1c::Scuf1cSpec>;
#[doc = "Write Protect Register \\#8"]
pub mod scuf1c;
#[doc = "SCUF30 (rw) register accessor: Write Protect Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf30`] module"]
#[doc(alias = "SCUF30")]
pub type Scuf30 = crate::Reg<scuf30::Scuf30Spec>;
#[doc = "Write Protect Register \\#13"]
pub mod scuf30;
#[doc = "SCUF34 (rw) register accessor: Write Protect Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf34`] module"]
#[doc(alias = "SCUF34")]
pub type Scuf34 = crate::Reg<scuf34::Scuf34Spec>;
#[doc = "Write Protect Register \\#14"]
pub mod scuf34;
#[doc = "SCUF38 (rw) register accessor: Write Protect Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf38`] module"]
#[doc(alias = "SCUF38")]
pub type Scuf38 = crate::Reg<scuf38::Scuf38Spec>;
#[doc = "Write Protect Register \\#15"]
pub mod scuf38;
#[doc = "SCUF50 (rw) register accessor: Write Protect Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf50`] module"]
#[doc(alias = "SCUF50")]
pub type Scuf50 = crate::Reg<scuf50::Scuf50Spec>;
#[doc = "Write Protect Register \\#21"]
pub mod scuf50;
#[doc = "SCUF58 (rw) register accessor: Write Protect Register \\#23\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf58`] module"]
#[doc(alias = "SCUF58")]
pub type Scuf58 = crate::Reg<scuf58::Scuf58Spec>;
#[doc = "Write Protect Register \\#23"]
pub mod scuf58;
#[doc = "SCUF6C (rw) register accessor: Write Protect Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf6c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf6c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf6c`] module"]
#[doc(alias = "SCUF6C")]
pub type Scuf6c = crate::Reg<scuf6c::Scuf6cSpec>;
#[doc = "Write Protect Register \\#28"]
pub mod scuf6c;
#[doc = "SCUF7C (rw) register accessor: Write Protect Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf7c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf7c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf7c`] module"]
#[doc(alias = "SCUF7C")]
pub type Scuf7c = crate::Reg<scuf7c::Scuf7cSpec>;
#[doc = "Write Protect Register \\#32"]
pub mod scuf7c;
#[doc = "SCUF80 (rw) register accessor: Reset Source Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf80`] module"]
#[doc(alias = "SCUF80")]
pub type Scuf80 = crate::Reg<scuf80::Scuf80Spec>;
#[doc = "Reset Source Control Register \\#1"]
pub mod scuf80;
#[doc = "SCUF88 (rw) register accessor: Reset Source Control Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf88`] module"]
#[doc(alias = "SCUF88")]
pub type Scuf88 = crate::Reg<scuf88::Scuf88Spec>;
#[doc = "Reset Source Control Register \\#3"]
pub mod scuf88;
#[doc = "SCUF90 (rw) register accessor: Reset Source Control Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf90`] module"]
#[doc(alias = "SCUF90")]
pub type Scuf90 = crate::Reg<scuf90::Scuf90Spec>;
#[doc = "Reset Source Control Register \\#5"]
pub mod scuf90;
#[doc = "SCUF98 (rw) register accessor: Reset Source Control Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf98`] module"]
#[doc(alias = "SCUF98")]
pub type Scuf98 = crate::Reg<scuf98::Scuf98Spec>;
#[doc = "Reset Source Control Register \\#7"]
pub mod scuf98;
#[doc = "SCUF9C (rw) register accessor: Reset Source Control Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf9c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf9c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuf9c`] module"]
#[doc(alias = "SCUF9C")]
pub type Scuf9c = crate::Reg<scuf9c::Scuf9cSpec>;
#[doc = "Reset Source Control Register \\#8"]
pub mod scuf9c;
#[doc = "SCUFB0 (rw) register accessor: Reset Source Control Register \\#13\n\nYou can [`read`](crate::Reg::read) this register and get [`scufb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scufb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scufb0`] module"]
#[doc(alias = "SCUFB0")]
pub type Scufb0 = crate::Reg<scufb0::Scufb0Spec>;
#[doc = "Reset Source Control Register \\#13"]
pub mod scufb0;
#[doc = "SCUFB4 (rw) register accessor: Reset Source Control Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`scufb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scufb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scufb4`] module"]
#[doc(alias = "SCUFB4")]
pub type Scufb4 = crate::Reg<scufb4::Scufb4Spec>;
#[doc = "Reset Source Control Register \\#14"]
pub mod scufb4;
#[doc = "SCUFB8 (rw) register accessor: Reset Source Control Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`scufb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scufb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scufb8`] module"]
#[doc(alias = "SCUFB8")]
pub type Scufb8 = crate::Reg<scufb8::Scufb8Spec>;
#[doc = "Reset Source Control Register \\#15"]
pub mod scufb8;
#[doc = "SCUFD0 (rw) register accessor: Reset Source Control Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`scufd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scufd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scufd0`] module"]
#[doc(alias = "SCUFD0")]
pub type Scufd0 = crate::Reg<scufd0::Scufd0Spec>;
#[doc = "Reset Source Control Register \\#21"]
pub mod scufd0;
#[doc = "SCUFEC (rw) register accessor: Reset Source Control Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`scufec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scufec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scufec`] module"]
#[doc(alias = "SCUFEC")]
pub type Scufec = crate::Reg<scufec::ScufecSpec>;
#[doc = "Reset Source Control Register \\#28"]
pub mod scufec;
#[doc = "SCUFFC (rw) register accessor: Reset Source Control Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`scuffc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuffc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scuffc`] module"]
#[doc(alias = "SCUFFC")]
pub type Scuffc = crate::Reg<scuffc::ScuffcSpec>;
#[doc = "Reset Source Control Register \\#32"]
pub mod scuffc;
