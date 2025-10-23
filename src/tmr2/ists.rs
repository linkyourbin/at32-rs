#[doc = "Register `ISTS` reader"]
pub type R = crate::R<IstsSpec>;
#[doc = "Register `ISTS` writer"]
pub type W = crate::W<IstsSpec>;
#[doc = "Field `OVFIF` reader - Overflow interrupt flag"]
pub type OvfifR = crate::BitReader;
#[doc = "Field `OVFIF` writer - Overflow interrupt flag"]
pub type OvfifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1IF` reader - Channel 1 interrupt flag"]
pub type C1ifR = crate::BitReader;
#[doc = "Field `C1IF` writer - Channel 1 interrupt flag"]
pub type C1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2IF` reader - Channel 2 interrupt flag"]
pub type C2ifR = crate::BitReader;
#[doc = "Field `C2IF` writer - Channel 2 interrupt flag"]
pub type C2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3IF` reader - Channel 3 interrupt flag"]
pub type C3ifR = crate::BitReader;
#[doc = "Field `C3IF` writer - Channel 3 interrupt flag"]
pub type C3ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4IF` reader - Channel 4 interrupt flag"]
pub type C4ifR = crate::BitReader;
#[doc = "Field `C4IF` writer - Channel 4 interrupt flag"]
pub type C4ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGIF` reader - Trigger interrupt flag"]
pub type TrgifR = crate::BitReader;
#[doc = "Field `TRGIF` writer - Trigger interrupt flag"]
pub type TrgifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1RF` reader - Channel 1 recapture flag"]
pub type C1rfR = crate::BitReader;
#[doc = "Field `C1RF` writer - Channel 1 recapture flag"]
pub type C1rfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2RF` reader - Channel 2 recapture flag"]
pub type C2rfR = crate::BitReader;
#[doc = "Field `C2RF` writer - Channel 2 recapture flag"]
pub type C2rfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3RF` reader - Channel 3 recapture flag"]
pub type C3rfR = crate::BitReader;
#[doc = "Field `C3RF` writer - Channel 3 recapture flag"]
pub type C3rfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4RF` reader - Channel 4 recapture flag"]
pub type C4rfR = crate::BitReader;
#[doc = "Field `C4RF` writer - Channel 4 recapture flag"]
pub type C4rfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovfif(&self) -> OvfifR {
        OvfifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    pub fn c1if(&self) -> C1ifR {
        C1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    pub fn c2if(&self) -> C2ifR {
        C2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 interrupt flag"]
    #[inline(always)]
    pub fn c3if(&self) -> C3ifR {
        C3ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 interrupt flag"]
    #[inline(always)]
    pub fn c4if(&self) -> C4ifR {
        C4ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&self) -> TrgifR {
        TrgifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    pub fn c1rf(&self) -> C1rfR {
        C1rfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    pub fn c2rf(&self) -> C2rfR {
        C2rfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 recapture flag"]
    #[inline(always)]
    pub fn c3rf(&self) -> C3rfR {
        C3rfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 recapture flag"]
    #[inline(always)]
    pub fn c4rf(&self) -> C4rfR {
        C4rfR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovfif(&mut self) -> OvfifW<'_, IstsSpec> {
        OvfifW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 interrupt flag"]
    #[inline(always)]
    pub fn c1if(&mut self) -> C1ifW<'_, IstsSpec> {
        C1ifW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 interrupt flag"]
    #[inline(always)]
    pub fn c2if(&mut self) -> C2ifW<'_, IstsSpec> {
        C2ifW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 interrupt flag"]
    #[inline(always)]
    pub fn c3if(&mut self) -> C3ifW<'_, IstsSpec> {
        C3ifW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 interrupt flag"]
    #[inline(always)]
    pub fn c4if(&mut self) -> C4ifW<'_, IstsSpec> {
        C4ifW::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn trgif(&mut self) -> TrgifW<'_, IstsSpec> {
        TrgifW::new(self, 6)
    }
    #[doc = "Bit 9 - Channel 1 recapture flag"]
    #[inline(always)]
    pub fn c1rf(&mut self) -> C1rfW<'_, IstsSpec> {
        C1rfW::new(self, 9)
    }
    #[doc = "Bit 10 - Channel 2 recapture flag"]
    #[inline(always)]
    pub fn c2rf(&mut self) -> C2rfW<'_, IstsSpec> {
        C2rfW::new(self, 10)
    }
    #[doc = "Bit 11 - Channel 3 recapture flag"]
    #[inline(always)]
    pub fn c3rf(&mut self) -> C3rfW<'_, IstsSpec> {
        C3rfW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 4 recapture flag"]
    #[inline(always)]
    pub fn c4rf(&mut self) -> C4rfW<'_, IstsSpec> {
        C4rfW::new(self, 12)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ists::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ists::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstsSpec;
impl crate::RegisterSpec for IstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ists::R`](R) reader structure"]
impl crate::Readable for IstsSpec {}
#[doc = "`write(|w| ..)` method takes [`ists::W`](W) writer structure"]
impl crate::Writable for IstsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for IstsSpec {}
