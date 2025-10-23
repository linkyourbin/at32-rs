#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `TCIEN` reader - Transmission complete interrupt enable"]
pub type TcienR = crate::BitReader;
#[doc = "Field `TCIEN` writer - Transmission complete interrupt enable"]
pub type TcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0MIEN` reader - FIFO 0 receive message interrupt enable"]
pub type Rf0mienR = crate::BitReader;
#[doc = "Field `RF0MIEN` writer - FIFO 0 receive message interrupt enable"]
pub type Rf0mienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0FIEN` reader - Receive FIFO 0 full interrupt enable"]
pub type Rf0fienR = crate::BitReader;
#[doc = "Field `RF0FIEN` writer - Receive FIFO 0 full interrupt enable"]
pub type Rf0fienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0OIEN` reader - Receive FIFO 0 overflow interrupt enable"]
pub type Rf0oienR = crate::BitReader;
#[doc = "Field `RF0OIEN` writer - Receive FIFO 0 overflow interrupt enable"]
pub type Rf0oienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1MIEN` reader - FIFO 1 receive message interrupt enable"]
pub type Rf1mienR = crate::BitReader;
#[doc = "Field `RF1MIEN` writer - FIFO 1 receive message interrupt enable"]
pub type Rf1mienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1FIEN` reader - Receive FIFO 1 full interrupt enable"]
pub type Rf1fienR = crate::BitReader;
#[doc = "Field `RF1FIEN` writer - Receive FIFO 1 full interrupt enable"]
pub type Rf1fienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1OIEN` reader - Receive FIFO 1 overflow interrupt enable"]
pub type Rf1oienR = crate::BitReader;
#[doc = "Field `RF1OIEN` writer - Receive FIFO 1 overflow interrupt enable"]
pub type Rf1oienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EAIEN` reader - Error active interrupt enable"]
pub type EaienR = crate::BitReader;
#[doc = "Field `EAIEN` writer - Error active interrupt enable"]
pub type EaienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPIEN` reader - Error passive interrupt enable"]
pub type EpienR = crate::BitReader;
#[doc = "Field `EPIEN` writer - Error passive interrupt enable"]
pub type EpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOIEN` reader - Bus-off interrupt enable"]
pub type BoienR = crate::BitReader;
#[doc = "Field `BOIEN` writer - Bus-off interrupt enable"]
pub type BoienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRIEN` reader - Error type record interrupt enable"]
pub type EtrienR = crate::BitReader;
#[doc = "Field `ETRIEN` writer - Error type record interrupt enable"]
pub type EtrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOIEN` reader - Error occur interrupt enable"]
pub type EoienR = crate::BitReader;
#[doc = "Field `EOIEN` writer - Error occur interrupt enable"]
pub type EoienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDZIEN` reader - Quit doze mode interrupt enable"]
pub type QdzienR = crate::BitReader;
#[doc = "Field `QDZIEN` writer - Quit doze mode interrupt enable"]
pub type QdzienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDZIEN` reader - Enter doze mode interrupt enable"]
pub type EdzienR = crate::BitReader;
#[doc = "Field `EDZIEN` writer - Enter doze mode interrupt enable"]
pub type EdzienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TcienR {
        TcienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO 0 receive message interrupt enable"]
    #[inline(always)]
    pub fn rf0mien(&self) -> Rf0mienR {
        Rf0mienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn rf0fien(&self) -> Rf0fienR {
        Rf0fienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO 0 overflow interrupt enable"]
    #[inline(always)]
    pub fn rf0oien(&self) -> Rf0oienR {
        Rf0oienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO 1 receive message interrupt enable"]
    #[inline(always)]
    pub fn rf1mien(&self) -> Rf1mienR {
        Rf1mienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn rf1fien(&self) -> Rf1fienR {
        Rf1fienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO 1 overflow interrupt enable"]
    #[inline(always)]
    pub fn rf1oien(&self) -> Rf1oienR {
        Rf1oienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Error active interrupt enable"]
    #[inline(always)]
    pub fn eaien(&self) -> EaienR {
        EaienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epien(&self) -> EpienR {
        EpienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    pub fn boien(&self) -> BoienR {
        BoienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Error type record interrupt enable"]
    #[inline(always)]
    pub fn etrien(&self) -> EtrienR {
        EtrienR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Error occur interrupt enable"]
    #[inline(always)]
    pub fn eoien(&self) -> EoienR {
        EoienR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Quit doze mode interrupt enable"]
    #[inline(always)]
    pub fn qdzien(&self) -> QdzienR {
        QdzienR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enter doze mode interrupt enable"]
    #[inline(always)]
    pub fn edzien(&self) -> EdzienR {
        EdzienR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcien(&mut self) -> TcienW<'_, IntenSpec> {
        TcienW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO 0 receive message interrupt enable"]
    #[inline(always)]
    pub fn rf0mien(&mut self) -> Rf0mienW<'_, IntenSpec> {
        Rf0mienW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO 0 full interrupt enable"]
    #[inline(always)]
    pub fn rf0fien(&mut self) -> Rf0fienW<'_, IntenSpec> {
        Rf0fienW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO 0 overflow interrupt enable"]
    #[inline(always)]
    pub fn rf0oien(&mut self) -> Rf0oienW<'_, IntenSpec> {
        Rf0oienW::new(self, 3)
    }
    #[doc = "Bit 4 - FIFO 1 receive message interrupt enable"]
    #[inline(always)]
    pub fn rf1mien(&mut self) -> Rf1mienW<'_, IntenSpec> {
        Rf1mienW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO 1 full interrupt enable"]
    #[inline(always)]
    pub fn rf1fien(&mut self) -> Rf1fienW<'_, IntenSpec> {
        Rf1fienW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive FIFO 1 overflow interrupt enable"]
    #[inline(always)]
    pub fn rf1oien(&mut self) -> Rf1oienW<'_, IntenSpec> {
        Rf1oienW::new(self, 6)
    }
    #[doc = "Bit 8 - Error active interrupt enable"]
    #[inline(always)]
    pub fn eaien(&mut self) -> EaienW<'_, IntenSpec> {
        EaienW::new(self, 8)
    }
    #[doc = "Bit 9 - Error passive interrupt enable"]
    #[inline(always)]
    pub fn epien(&mut self) -> EpienW<'_, IntenSpec> {
        EpienW::new(self, 9)
    }
    #[doc = "Bit 10 - Bus-off interrupt enable"]
    #[inline(always)]
    pub fn boien(&mut self) -> BoienW<'_, IntenSpec> {
        BoienW::new(self, 10)
    }
    #[doc = "Bit 11 - Error type record interrupt enable"]
    #[inline(always)]
    pub fn etrien(&mut self) -> EtrienW<'_, IntenSpec> {
        EtrienW::new(self, 11)
    }
    #[doc = "Bit 15 - Error occur interrupt enable"]
    #[inline(always)]
    pub fn eoien(&mut self) -> EoienW<'_, IntenSpec> {
        EoienW::new(self, 15)
    }
    #[doc = "Bit 16 - Quit doze mode interrupt enable"]
    #[inline(always)]
    pub fn qdzien(&mut self) -> QdzienW<'_, IntenSpec> {
        QdzienW::new(self, 16)
    }
    #[doc = "Bit 17 - Enter doze mode interrupt enable"]
    #[inline(always)]
    pub fn edzien(&mut self) -> EdzienW<'_, IntenSpec> {
        EdzienW::new(self, 17)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
