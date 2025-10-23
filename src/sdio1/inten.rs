#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `CMDFAILIEN` reader - Command crc fail interrupt enable"]
pub type CmdfailienR = crate::BitReader;
#[doc = "Field `CMDFAILIEN` writer - Command crc fail interrupt enable"]
pub type CmdfailienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFAILIEN` reader - Data crc fail interrupt enable"]
pub type DtfailienR = crate::BitReader;
#[doc = "Field `DTFAILIEN` writer - Data crc fail interrupt enable"]
pub type DtfailienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTIMEOUTIEN` reader - Command timeout interrupt enable"]
pub type CmdtimeoutienR = crate::BitReader;
#[doc = "Field `CMDTIMEOUTIEN` writer - Command timeout interrupt enable"]
pub type CmdtimeoutienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTIMEOUTIEN` reader - Data timeout interrupt enable"]
pub type DttimeoutienR = crate::BitReader;
#[doc = "Field `DTTIMEOUTIEN` writer - Data timeout interrupt enable"]
pub type DttimeoutienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRUIEN` reader - Tx under run interrupt enable"]
pub type TxerruienR = crate::BitReader;
#[doc = "Field `TXERRUIEN` writer - Tx under run interrupt enable"]
pub type TxerruienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRUIEN` reader - Rx over run interrupt enable"]
pub type RxerruienR = crate::BitReader;
#[doc = "Field `RXERRUIEN` writer - Rx over run interrupt enable"]
pub type RxerruienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRSPCMPLIEN` reader - Command response complete interrupt enable"]
pub type CmdrspcmplienR = crate::BitReader;
#[doc = "Field `CMDRSPCMPLIEN` writer - Command response complete interrupt enable"]
pub type CmdrspcmplienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCMPLIEN` reader - Command sent complete interrupt enable"]
pub type CmdcmplienR = crate::BitReader;
#[doc = "Field `CMDCMPLIEN` writer - Command sent complete interrupt enable"]
pub type CmdcmplienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCMPLIEN` reader - Data sent complete interrupt enable"]
pub type DtcmplienR = crate::BitReader;
#[doc = "Field `DTCMPLIEN` writer - Data sent complete interrupt enable"]
pub type DtcmplienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBITERRIEN` reader - Start bit error interrupt enable"]
pub type SbiterrienR = crate::BitReader;
#[doc = "Field `SBITERRIEN` writer - Start bit error interrupt enable"]
pub type SbiterrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBLKCMPLIEN` reader - Data block sent complete interrupt enable"]
pub type DtblkcmplienR = crate::BitReader;
#[doc = "Field `DTBLKCMPLIEN` writer - Data block sent complete interrupt enable"]
pub type DtblkcmplienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOCMDIEN` reader - Command acting interrupt enable"]
pub type DocmdienR = crate::BitReader;
#[doc = "Field `DOCMDIEN` writer - Command acting interrupt enable"]
pub type DocmdienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOTXIEN` reader - Data transmit acting interrupt enable"]
pub type DotxienR = crate::BitReader;
#[doc = "Field `DOTXIEN` writer - Data transmit acting interrupt enable"]
pub type DotxienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORXIEN` reader - Data receive acting interrupt enable"]
pub type DorxienR = crate::BitReader;
#[doc = "Field `DORXIEN` writer - Data receive acting interrupt enable"]
pub type DorxienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFHIEN` reader - Tx buffer half empty interrupt enable"]
pub type TxbufhienR = crate::BitReader;
#[doc = "Field `TXBUFHIEN` writer - Tx buffer half empty interrupt enable"]
pub type TxbufhienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFHIEN` reader - Rx buffer half empty interrupt enable"]
pub type RxbufhienR = crate::BitReader;
#[doc = "Field `RXBUFHIEN` writer - Rx buffer half empty interrupt enable"]
pub type RxbufhienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFFIEN` reader - Tx buffer full interrupt enable"]
pub type TxbuffienR = crate::BitReader;
#[doc = "Field `TXBUFFIEN` writer - Tx buffer full interrupt enable"]
pub type TxbuffienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFFIEN` reader - Rx buffer full interrupt enable"]
pub type RxbuffienR = crate::BitReader;
#[doc = "Field `RXBUFFIEN` writer - Rx buffer full interrupt enable"]
pub type RxbuffienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFEIEN` reader - Tx buffer empty interrupt enable"]
pub type TxbufeienR = crate::BitReader;
#[doc = "Field `TXBUFEIEN` writer - Tx buffer empty interrupt enable"]
pub type TxbufeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFEIEN` reader - Rx buffer empty interrupt enable"]
pub type RxbufeienR = crate::BitReader;
#[doc = "Field `RXBUFEIEN` writer - Rx buffer empty interrupt enable"]
pub type RxbufeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFIEN` reader - Tx buffer data vaild interrupt enable"]
pub type TxbufienR = crate::BitReader;
#[doc = "Field `TXBUFIEN` writer - Tx buffer data vaild interrupt enable"]
pub type TxbufienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFIEN` reader - Rx buffer data vaild interrupt enable"]
pub type RxbufienR = crate::BitReader;
#[doc = "Field `RXBUFIEN` writer - Rx buffer data vaild interrupt enable"]
pub type RxbufienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOIFIEN` reader - SD I/O interrupt enable"]
pub type IoifienR = crate::BitReader;
#[doc = "Field `IOIFIEN` writer - SD I/O interrupt enable"]
pub type IoifienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command crc fail interrupt enable"]
    #[inline(always)]
    pub fn cmdfailien(&self) -> CmdfailienR {
        CmdfailienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data crc fail interrupt enable"]
    #[inline(always)]
    pub fn dtfailien(&self) -> DtfailienR {
        DtfailienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn cmdtimeoutien(&self) -> CmdtimeoutienR {
        CmdtimeoutienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dttimeoutien(&self) -> DttimeoutienR {
        DttimeoutienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx under run interrupt enable"]
    #[inline(always)]
    pub fn txerruien(&self) -> TxerruienR {
        TxerruienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx over run interrupt enable"]
    #[inline(always)]
    pub fn rxerruien(&self) -> RxerruienR {
        RxerruienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response complete interrupt enable"]
    #[inline(always)]
    pub fn cmdrspcmplien(&self) -> CmdrspcmplienR {
        CmdrspcmplienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent complete interrupt enable"]
    #[inline(always)]
    pub fn cmdcmplien(&self) -> CmdcmplienR {
        CmdcmplienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data sent complete interrupt enable"]
    #[inline(always)]
    pub fn dtcmplien(&self) -> DtcmplienR {
        DtcmplienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn sbiterrien(&self) -> SbiterrienR {
        SbiterrienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent complete interrupt enable"]
    #[inline(always)]
    pub fn dtblkcmplien(&self) -> DtblkcmplienR {
        DtblkcmplienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    pub fn docmdien(&self) -> DocmdienR {
        DocmdienR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    pub fn dotxien(&self) -> DotxienR {
        DotxienR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    pub fn dorxien(&self) -> DorxienR {
        DorxienR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx buffer half empty interrupt enable"]
    #[inline(always)]
    pub fn txbufhien(&self) -> TxbufhienR {
        TxbufhienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx buffer half empty interrupt enable"]
    #[inline(always)]
    pub fn rxbufhien(&self) -> RxbufhienR {
        RxbufhienR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx buffer full interrupt enable"]
    #[inline(always)]
    pub fn txbuffien(&self) -> TxbuffienR {
        TxbuffienR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbuffien(&self) -> RxbuffienR {
        RxbuffienR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbufeien(&self) -> TxbufeienR {
        TxbufeienR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn rxbufeien(&self) -> RxbufeienR {
        RxbufeienR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tx buffer data vaild interrupt enable"]
    #[inline(always)]
    pub fn txbufien(&self) -> TxbufienR {
        TxbufienR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rx buffer data vaild interrupt enable"]
    #[inline(always)]
    pub fn rxbufien(&self) -> RxbufienR {
        RxbufienR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt enable"]
    #[inline(always)]
    pub fn ioifien(&self) -> IoifienR {
        IoifienR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command crc fail interrupt enable"]
    #[inline(always)]
    pub fn cmdfailien(&mut self) -> CmdfailienW<'_, IntenSpec> {
        CmdfailienW::new(self, 0)
    }
    #[doc = "Bit 1 - Data crc fail interrupt enable"]
    #[inline(always)]
    pub fn dtfailien(&mut self) -> DtfailienW<'_, IntenSpec> {
        DtfailienW::new(self, 1)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable"]
    #[inline(always)]
    pub fn cmdtimeoutien(&mut self) -> CmdtimeoutienW<'_, IntenSpec> {
        CmdtimeoutienW::new(self, 2)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dttimeoutien(&mut self) -> DttimeoutienW<'_, IntenSpec> {
        DttimeoutienW::new(self, 3)
    }
    #[doc = "Bit 4 - Tx under run interrupt enable"]
    #[inline(always)]
    pub fn txerruien(&mut self) -> TxerruienW<'_, IntenSpec> {
        TxerruienW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx over run interrupt enable"]
    #[inline(always)]
    pub fn rxerruien(&mut self) -> RxerruienW<'_, IntenSpec> {
        RxerruienW::new(self, 5)
    }
    #[doc = "Bit 6 - Command response complete interrupt enable"]
    #[inline(always)]
    pub fn cmdrspcmplien(&mut self) -> CmdrspcmplienW<'_, IntenSpec> {
        CmdrspcmplienW::new(self, 6)
    }
    #[doc = "Bit 7 - Command sent complete interrupt enable"]
    #[inline(always)]
    pub fn cmdcmplien(&mut self) -> CmdcmplienW<'_, IntenSpec> {
        CmdcmplienW::new(self, 7)
    }
    #[doc = "Bit 8 - Data sent complete interrupt enable"]
    #[inline(always)]
    pub fn dtcmplien(&mut self) -> DtcmplienW<'_, IntenSpec> {
        DtcmplienW::new(self, 8)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn sbiterrien(&mut self) -> SbiterrienW<'_, IntenSpec> {
        SbiterrienW::new(self, 9)
    }
    #[doc = "Bit 10 - Data block sent complete interrupt enable"]
    #[inline(always)]
    pub fn dtblkcmplien(&mut self) -> DtblkcmplienW<'_, IntenSpec> {
        DtblkcmplienW::new(self, 10)
    }
    #[doc = "Bit 11 - Command acting interrupt enable"]
    #[inline(always)]
    pub fn docmdien(&mut self) -> DocmdienW<'_, IntenSpec> {
        DocmdienW::new(self, 11)
    }
    #[doc = "Bit 12 - Data transmit acting interrupt enable"]
    #[inline(always)]
    pub fn dotxien(&mut self) -> DotxienW<'_, IntenSpec> {
        DotxienW::new(self, 12)
    }
    #[doc = "Bit 13 - Data receive acting interrupt enable"]
    #[inline(always)]
    pub fn dorxien(&mut self) -> DorxienW<'_, IntenSpec> {
        DorxienW::new(self, 13)
    }
    #[doc = "Bit 14 - Tx buffer half empty interrupt enable"]
    #[inline(always)]
    pub fn txbufhien(&mut self) -> TxbufhienW<'_, IntenSpec> {
        TxbufhienW::new(self, 14)
    }
    #[doc = "Bit 15 - Rx buffer half empty interrupt enable"]
    #[inline(always)]
    pub fn rxbufhien(&mut self) -> RxbufhienW<'_, IntenSpec> {
        RxbufhienW::new(self, 15)
    }
    #[doc = "Bit 16 - Tx buffer full interrupt enable"]
    #[inline(always)]
    pub fn txbuffien(&mut self) -> TxbuffienW<'_, IntenSpec> {
        TxbuffienW::new(self, 16)
    }
    #[doc = "Bit 17 - Rx buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbuffien(&mut self) -> RxbuffienW<'_, IntenSpec> {
        RxbuffienW::new(self, 17)
    }
    #[doc = "Bit 18 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbufeien(&mut self) -> TxbufeienW<'_, IntenSpec> {
        TxbufeienW::new(self, 18)
    }
    #[doc = "Bit 19 - Rx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn rxbufeien(&mut self) -> RxbufeienW<'_, IntenSpec> {
        RxbufeienW::new(self, 19)
    }
    #[doc = "Bit 20 - Tx buffer data vaild interrupt enable"]
    #[inline(always)]
    pub fn txbufien(&mut self) -> TxbufienW<'_, IntenSpec> {
        TxbufienW::new(self, 20)
    }
    #[doc = "Bit 21 - Rx buffer data vaild interrupt enable"]
    #[inline(always)]
    pub fn rxbufien(&mut self) -> RxbufienW<'_, IntenSpec> {
        RxbufienW::new(self, 21)
    }
    #[doc = "Bit 22 - SD I/O interrupt enable"]
    #[inline(always)]
    pub fn ioifien(&mut self) -> IoifienW<'_, IntenSpec> {
        IoifienW::new(self, 22)
    }
}
#[doc = "SDIO interrupt enable register (SDIO_INTEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
