#[doc = "Register `FCTRL` reader"]
pub type R = crate::R<FctrlSpec>;
#[doc = "Register `FCTRL` writer"]
pub type W = crate::W<FctrlSpec>;
#[doc = "Field `FCS` reader - Filters configure switch"]
pub type FcsR = crate::BitReader;
#[doc = "Field `FCS` writer - Filters configure switch"]
pub type FcsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filters configure switch"]
    #[inline(always)]
    pub fn fcs(&self) -> FcsR {
        FcsR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filters configure switch"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FcsW<'_, FctrlSpec> {
        FcsW::new(self, 0)
    }
}
#[doc = "Filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctrlSpec;
impl crate::RegisterSpec for FctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl::R`](R) reader structure"]
impl crate::Readable for FctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctrl::W`](W) writer structure"]
impl crate::Writable for FctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCTRL to value 0"]
impl crate::Resettable for FctrlSpec {}
