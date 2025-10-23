#[doc = "Register `CCTRL` reader"]
pub type R = crate::R<CctrlSpec>;
#[doc = "Register `CCTRL` writer"]
pub type W = crate::W<CctrlSpec>;
#[doc = "Field `C1EN` reader - Channel 1 enable"]
pub type C1enR = crate::BitReader;
#[doc = "Field `C1EN` writer - Channel 1 enable"]
pub type C1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1P` reader - Channel 1 Polarity"]
pub type C1pR = crate::BitReader;
#[doc = "Field `C1P` writer - Channel 1 Polarity"]
pub type C1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1CEN` reader - Channel 1 complementary enable"]
pub type C1cenR = crate::BitReader;
#[doc = "Field `C1CEN` writer - Channel 1 complementary enable"]
pub type C1cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1CP` reader - Channel 1 complementary polarity"]
pub type C1cpR = crate::BitReader;
#[doc = "Field `C1CP` writer - Channel 1 complementary polarity"]
pub type C1cpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2EN` reader - Channel 2 enable"]
pub type C2enR = crate::BitReader;
#[doc = "Field `C2EN` writer - Channel 2 enable"]
pub type C2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2P` reader - Channel 2 Polarity"]
pub type C2pR = crate::BitReader;
#[doc = "Field `C2P` writer - Channel 2 Polarity"]
pub type C2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2CEN` reader - Channel 2 complementary enable"]
pub type C2cenR = crate::BitReader;
#[doc = "Field `C2CEN` writer - Channel 2 complementary enable"]
pub type C2cenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2CP` reader - Channel 2 complementary polarity"]
pub type C2cpR = crate::BitReader;
#[doc = "Field `C2CP` writer - Channel 2 complementary polarity"]
pub type C2cpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    pub fn c1en(&self) -> C1enR {
        C1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn c1p(&self) -> C1pR {
        C1pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 complementary enable"]
    #[inline(always)]
    pub fn c1cen(&self) -> C1cenR {
        C1cenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 complementary polarity"]
    #[inline(always)]
    pub fn c1cp(&self) -> C1cpR {
        C1cpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    pub fn c2en(&self) -> C2enR {
        C2enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn c2p(&self) -> C2pR {
        C2pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 complementary enable"]
    #[inline(always)]
    pub fn c2cen(&self) -> C2cenR {
        C2cenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 complementary polarity"]
    #[inline(always)]
    pub fn c2cp(&self) -> C2cpR {
        C2cpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    pub fn c1en(&mut self) -> C1enW<'_, CctrlSpec> {
        C1enW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn c1p(&mut self) -> C1pW<'_, CctrlSpec> {
        C1pW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 1 complementary enable"]
    #[inline(always)]
    pub fn c1cen(&mut self) -> C1cenW<'_, CctrlSpec> {
        C1cenW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 1 complementary polarity"]
    #[inline(always)]
    pub fn c1cp(&mut self) -> C1cpW<'_, CctrlSpec> {
        C1cpW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    pub fn c2en(&mut self) -> C2enW<'_, CctrlSpec> {
        C2enW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn c2p(&mut self) -> C2pW<'_, CctrlSpec> {
        C2pW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel 2 complementary enable"]
    #[inline(always)]
    pub fn c2cen(&mut self) -> C2cenW<'_, CctrlSpec> {
        C2cenW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel 2 complementary polarity"]
    #[inline(always)]
    pub fn c2cp(&mut self) -> C2cpW<'_, CctrlSpec> {
        C2cpW::new(self, 7)
    }
}
#[doc = "Channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CctrlSpec;
impl crate::RegisterSpec for CctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cctrl::R`](R) reader structure"]
impl crate::Readable for CctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cctrl::W`](W) writer structure"]
impl crate::Writable for CctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCTRL to value 0"]
impl crate::Resettable for CctrlSpec {}
