#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "Field `CMDFAIL` reader - Command crc fail flag clear"]
pub type CmdfailR = crate::BitReader;
#[doc = "Field `CMDFAIL` writer - Command crc fail flag clear"]
pub type CmdfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFAIL` reader - Data crc fail flag clear"]
pub type DtfailR = crate::BitReader;
#[doc = "Field `DTFAIL` writer - Data crc fail flag clear"]
pub type DtfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTIMEOUT` reader - Command timeout flag clear"]
pub type CmdtimeoutR = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` writer - Command timeout flag clear"]
pub type CmdtimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTIMEOUT` reader - Data timeout flag clear"]
pub type DttimeoutR = crate::BitReader;
#[doc = "Field `DTTIMEOUT` writer - Data timeout flag clear"]
pub type DttimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRU` reader - Tx under run error flag clear"]
pub type TxerruR = crate::BitReader;
#[doc = "Field `TXERRU` writer - Tx under run error flag clear"]
pub type TxerruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRU` reader - Rx over run error flag clear"]
pub type RxerruR = crate::BitReader;
#[doc = "Field `RXERRU` writer - Rx over run error flag clear"]
pub type RxerruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRSPCMPL` reader - Command response complete flag clear"]
pub type CmdrspcmplR = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` writer - Command response complete flag clear"]
pub type CmdrspcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCMPL` reader - Command sent flag clear"]
pub type CmdcmplR = crate::BitReader;
#[doc = "Field `CMDCMPL` writer - Command sent flag clear"]
pub type CmdcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCMPL` reader - Data sent flag clear"]
pub type DtcmplR = crate::BitReader;
#[doc = "Field `DTCMPL` writer - Data sent flag clear"]
pub type DtcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBITERR` reader - Start bit error flag clear"]
pub type SbiterrR = crate::BitReader;
#[doc = "Field `SBITERR` writer - Start bit error flag clear"]
pub type SbiterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBLKCMPL` reader - Data block sent clear"]
pub type DtblkcmplR = crate::BitReader;
#[doc = "Field `DTBLKCMPL` writer - Data block sent clear"]
pub type DtblkcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOIF` reader - SD I/O interrupt flag clear"]
pub type IoifR = crate::BitReader;
#[doc = "Field `IOIF` writer - SD I/O interrupt flag clear"]
pub type IoifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command crc fail flag clear"]
    #[inline(always)]
    pub fn cmdfail(&self) -> CmdfailR {
        CmdfailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data crc fail flag clear"]
    #[inline(always)]
    pub fn dtfail(&self) -> DtfailR {
        DtfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout flag clear"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CmdtimeoutR {
        CmdtimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout flag clear"]
    #[inline(always)]
    pub fn dttimeout(&self) -> DttimeoutR {
        DttimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx under run error flag clear"]
    #[inline(always)]
    pub fn txerru(&self) -> TxerruR {
        TxerruR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx over run error flag clear"]
    #[inline(always)]
    pub fn rxerru(&self) -> RxerruR {
        RxerruR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response complete flag clear"]
    #[inline(always)]
    pub fn cmdrspcmpl(&self) -> CmdrspcmplR {
        CmdrspcmplR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent flag clear"]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CmdcmplR {
        CmdcmplR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data sent flag clear"]
    #[inline(always)]
    pub fn dtcmpl(&self) -> DtcmplR {
        DtcmplR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error flag clear"]
    #[inline(always)]
    pub fn sbiterr(&self) -> SbiterrR {
        SbiterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent clear"]
    #[inline(always)]
    pub fn dtblkcmpl(&self) -> DtblkcmplR {
        DtblkcmplR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt flag clear"]
    #[inline(always)]
    pub fn ioif(&self) -> IoifR {
        IoifR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command crc fail flag clear"]
    #[inline(always)]
    pub fn cmdfail(&mut self) -> CmdfailW<'_, IntclrSpec> {
        CmdfailW::new(self, 0)
    }
    #[doc = "Bit 1 - Data crc fail flag clear"]
    #[inline(always)]
    pub fn dtfail(&mut self) -> DtfailW<'_, IntclrSpec> {
        DtfailW::new(self, 1)
    }
    #[doc = "Bit 2 - Command timeout flag clear"]
    #[inline(always)]
    pub fn cmdtimeout(&mut self) -> CmdtimeoutW<'_, IntclrSpec> {
        CmdtimeoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Data timeout flag clear"]
    #[inline(always)]
    pub fn dttimeout(&mut self) -> DttimeoutW<'_, IntclrSpec> {
        DttimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - Tx under run error flag clear"]
    #[inline(always)]
    pub fn txerru(&mut self) -> TxerruW<'_, IntclrSpec> {
        TxerruW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx over run error flag clear"]
    #[inline(always)]
    pub fn rxerru(&mut self) -> RxerruW<'_, IntclrSpec> {
        RxerruW::new(self, 5)
    }
    #[doc = "Bit 6 - Command response complete flag clear"]
    #[inline(always)]
    pub fn cmdrspcmpl(&mut self) -> CmdrspcmplW<'_, IntclrSpec> {
        CmdrspcmplW::new(self, 6)
    }
    #[doc = "Bit 7 - Command sent flag clear"]
    #[inline(always)]
    pub fn cmdcmpl(&mut self) -> CmdcmplW<'_, IntclrSpec> {
        CmdcmplW::new(self, 7)
    }
    #[doc = "Bit 8 - Data sent flag clear"]
    #[inline(always)]
    pub fn dtcmpl(&mut self) -> DtcmplW<'_, IntclrSpec> {
        DtcmplW::new(self, 8)
    }
    #[doc = "Bit 9 - Start bit error flag clear"]
    #[inline(always)]
    pub fn sbiterr(&mut self) -> SbiterrW<'_, IntclrSpec> {
        SbiterrW::new(self, 9)
    }
    #[doc = "Bit 10 - Data block sent clear"]
    #[inline(always)]
    pub fn dtblkcmpl(&mut self) -> DtblkcmplW<'_, IntclrSpec> {
        DtblkcmplW::new(self, 10)
    }
    #[doc = "Bit 22 - SD I/O interrupt flag clear"]
    #[inline(always)]
    pub fn ioif(&mut self) -> IoifW<'_, IntclrSpec> {
        IoifW::new(self, 22)
    }
}
#[doc = "SDIO interrupt clear register (SDIO_INTCLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`intclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {}
