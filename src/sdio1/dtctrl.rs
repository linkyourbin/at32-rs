#[doc = "Register `DTCTRL` reader"]
pub type R = crate::R<DtctrlSpec>;
#[doc = "Register `DTCTRL` writer"]
pub type W = crate::W<DtctrlSpec>;
#[doc = "Field `TFREN` reader - DTEN"]
pub type TfrenR = crate::BitReader;
#[doc = "Field `TFREN` writer - DTEN"]
pub type TfrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFRDIR` reader - DTDIR"]
pub type TfrdirR = crate::BitReader;
#[doc = "Field `TFRDIR` writer - DTDIR"]
pub type TfrdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFRMODE` reader - DTMODE"]
pub type TfrmodeR = crate::BitReader;
#[doc = "Field `TFRMODE` writer - DTMODE"]
pub type TfrmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLKSIZE` reader - DBLOCKSIZE"]
pub type BlksizeR = crate::FieldReader;
#[doc = "Field `BLKSIZE` writer - DBLOCKSIZE"]
pub type BlksizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RDWTSTART` reader - PWSTART"]
pub type RdwtstartR = crate::BitReader;
#[doc = "Field `RDWTSTART` writer - PWSTART"]
pub type RdwtstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDWTSTOP` reader - PWSTOP"]
pub type RdwtstopR = crate::BitReader;
#[doc = "Field `RDWTSTOP` writer - PWSTOP"]
pub type RdwtstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDWTMODE` reader - RWMOD"]
pub type RdwtmodeR = crate::BitReader;
#[doc = "Field `RDWTMODE` writer - RWMOD"]
pub type RdwtmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOEN` reader - SD I/O function enable"]
pub type IoenR = crate::BitReader;
#[doc = "Field `IOEN` writer - SD I/O function enable"]
pub type IoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn tfren(&self) -> TfrenR {
        TfrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn tfrdir(&self) -> TfrdirR {
        TfrdirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn tfrmode(&self) -> TfrmodeR {
        TfrmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn blksize(&self) -> BlksizeR {
        BlksizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn rdwtstart(&self) -> RdwtstartR {
        RdwtstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn rdwtstop(&self) -> RdwtstopR {
        RdwtstopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rdwtmode(&self) -> RdwtmodeR {
        RdwtmodeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O function enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IoenR {
        IoenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn tfren(&mut self) -> TfrenW<'_, DtctrlSpec> {
        TfrenW::new(self, 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn tfrdir(&mut self) -> TfrdirW<'_, DtctrlSpec> {
        TfrdirW::new(self, 1)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn tfrmode(&mut self) -> TfrmodeW<'_, DtctrlSpec> {
        TfrmodeW::new(self, 2)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<'_, DtctrlSpec> {
        DmaenW::new(self, 3)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn blksize(&mut self) -> BlksizeW<'_, DtctrlSpec> {
        BlksizeW::new(self, 4)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn rdwtstart(&mut self) -> RdwtstartW<'_, DtctrlSpec> {
        RdwtstartW::new(self, 8)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn rdwtstop(&mut self) -> RdwtstopW<'_, DtctrlSpec> {
        RdwtstopW::new(self, 9)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rdwtmode(&mut self) -> RdwtmodeW<'_, DtctrlSpec> {
        RdwtmodeW::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O function enable"]
    #[inline(always)]
    pub fn ioen(&mut self) -> IoenW<'_, DtctrlSpec> {
        IoenW::new(self, 11)
    }
}
#[doc = "SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtctrlSpec;
impl crate::RegisterSpec for DtctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtctrl::R`](R) reader structure"]
impl crate::Readable for DtctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dtctrl::W`](W) writer structure"]
impl crate::Writable for DtctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DtctrlSpec {}
