#[doc = "Register `REMAP4` reader"]
pub type R = crate::R<Remap4Spec>;
#[doc = "Register `REMAP4` writer"]
pub type W = crate::W<Remap4Spec>;
#[doc = "Field `TMR1_GMUX` reader - TMR1 muxing"]
pub type Tmr1GmuxR = crate::FieldReader;
#[doc = "Field `TMR1_GMUX` writer - TMR1 muxing"]
pub type Tmr1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMR2_GMUX` reader - TMR2 muxing"]
pub type Tmr2GmuxR = crate::FieldReader;
#[doc = "Field `TMR2_GMUX` writer - TMR2 muxing"]
pub type Tmr2GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR2ITR1_GMUX` reader - TMR2 internal trigger 1 muxing"]
pub type Tmr2itr1GmuxR = crate::FieldReader;
#[doc = "Field `TMR2ITR1_GMUX` writer - TMR2 internal trigger 1 muxing"]
pub type Tmr2itr1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR3_GMUX` reader - TMR3 muxing"]
pub type Tmr3GmuxR = crate::FieldReader;
#[doc = "Field `TMR3_GMUX` writer - TMR3 muxing"]
pub type Tmr3GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMR4_GMUX` reader - TMR4 muxing"]
pub type Tmr4GmuxR = crate::FieldReader;
#[doc = "Field `TMR4_GMUX` writer - TMR4 muxing"]
pub type Tmr4GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMR5CH4_GMUX` reader - TMR5 channel4 internal muxing"]
pub type Tmr5ch4GmuxR = crate::BitReader;
#[doc = "Field `TMR5CH4_GMUX` writer - TMR5 channel4 internal muxing"]
pub type Tmr5ch4GmuxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_gmux(&self) -> Tmr1GmuxR {
        Tmr1GmuxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_gmux(&self) -> Tmr2GmuxR {
        Tmr2GmuxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TMR2 internal trigger 1 muxing"]
    #[inline(always)]
    pub fn tmr2itr1_gmux(&self) -> Tmr2itr1GmuxR {
        Tmr2itr1GmuxR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_gmux(&self) -> Tmr3GmuxR {
        Tmr3GmuxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TMR4 muxing"]
    #[inline(always)]
    pub fn tmr4_gmux(&self) -> Tmr4GmuxR {
        Tmr4GmuxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_gmux(&self) -> Tmr5ch4GmuxR {
        Tmr5ch4GmuxR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_gmux(&mut self) -> Tmr1GmuxW<'_, Remap4Spec> {
        Tmr1GmuxW::new(self, 0)
    }
    #[doc = "Bits 4:5 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_gmux(&mut self) -> Tmr2GmuxW<'_, Remap4Spec> {
        Tmr2GmuxW::new(self, 4)
    }
    #[doc = "Bits 6:7 - TMR2 internal trigger 1 muxing"]
    #[inline(always)]
    pub fn tmr2itr1_gmux(&mut self) -> Tmr2itr1GmuxW<'_, Remap4Spec> {
        Tmr2itr1GmuxW::new(self, 6)
    }
    #[doc = "Bits 8:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_gmux(&mut self) -> Tmr3GmuxW<'_, Remap4Spec> {
        Tmr3GmuxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - TMR4 muxing"]
    #[inline(always)]
    pub fn tmr4_gmux(&mut self) -> Tmr4GmuxW<'_, Remap4Spec> {
        Tmr4GmuxW::new(self, 12)
    }
    #[doc = "Bit 19 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_gmux(&mut self) -> Tmr5ch4GmuxW<'_, Remap4Spec> {
        Tmr5ch4GmuxW::new(self, 19)
    }
}
#[doc = "IO MUX remap register 4 (IOMUX_REMAP4)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap4Spec;
impl crate::RegisterSpec for Remap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap4::R`](R) reader structure"]
impl crate::Readable for Remap4Spec {}
#[doc = "`write(|w| ..)` method takes [`remap4::W`](W) writer structure"]
impl crate::Writable for Remap4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP4 to value 0"]
impl crate::Resettable for Remap4Spec {}
