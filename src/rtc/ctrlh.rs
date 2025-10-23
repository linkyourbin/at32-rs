#[doc = "Register `CTRLH` reader"]
pub type R = crate::R<CtrlhSpec>;
#[doc = "Register `CTRLH` writer"]
pub type W = crate::W<CtrlhSpec>;
#[doc = "Field `OVFIEN` reader - Overflow interrupt enable"]
pub type OvfienR = crate::BitReader;
#[doc = "Field `OVFIEN` writer - Overflow interrupt enable"]
pub type OvfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAIEN` reader - Time alarm interrupt enable"]
pub type TaienR = crate::BitReader;
#[doc = "Field `TAIEN` writer - Time alarm interrupt enable"]
pub type TaienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIEN` reader - Time second interrupt enable"]
pub type TsienR = crate::BitReader;
#[doc = "Field `TSIEN` writer - Time second interrupt enable"]
pub type TsienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&self) -> OvfienR {
        OvfienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time alarm interrupt enable"]
    #[inline(always)]
    pub fn taien(&self) -> TaienR {
        TaienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time second interrupt enable"]
    #[inline(always)]
    pub fn tsien(&self) -> TsienR {
        TsienR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ovfien(&mut self) -> OvfienW<'_, CtrlhSpec> {
        OvfienW::new(self, 0)
    }
    #[doc = "Bit 1 - Time alarm interrupt enable"]
    #[inline(always)]
    pub fn taien(&mut self) -> TaienW<'_, CtrlhSpec> {
        TaienW::new(self, 1)
    }
    #[doc = "Bit 2 - Time second interrupt enable"]
    #[inline(always)]
    pub fn tsien(&mut self) -> TsienW<'_, CtrlhSpec> {
        TsienW::new(self, 2)
    }
}
#[doc = "RTC Control Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlhSpec;
impl crate::RegisterSpec for CtrlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlh::R`](R) reader structure"]
impl crate::Readable for CtrlhSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlh::W`](W) writer structure"]
impl crate::Writable for CtrlhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLH to value 0"]
impl crate::Resettable for CtrlhSpec {}
