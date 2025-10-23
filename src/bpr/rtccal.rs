#[doc = "Register `RTCCAL` reader"]
pub type R = crate::R<RtccalSpec>;
#[doc = "Register `RTCCAL` writer"]
pub type W = crate::W<RtccalSpec>;
#[doc = "Field `CALVAL` reader - Calibration value"]
pub type CalvalR = crate::FieldReader;
#[doc = "Field `CALVAL` writer - Calibration value"]
pub type CalvalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CALOUT` reader - Calibration Clock Output"]
pub type CaloutR = crate::BitReader;
#[doc = "Field `CALOUT` writer - Calibration Clock Output"]
pub type CaloutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEN` reader - Output enable"]
pub type OutenR = crate::BitReader;
#[doc = "Field `OUTEN` writer - Output enable"]
pub type OutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTSEL` reader - Output selection"]
pub type OutselR = crate::BitReader;
#[doc = "Field `OUTSEL` writer - Output selection"]
pub type OutselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCOS` reader - Calibration clock output selection"]
pub type CcosR = crate::BitReader;
#[doc = "Field `CCOS` writer - Calibration clock output selection"]
pub type CcosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTM` reader - Output mode"]
pub type OutmR = crate::BitReader;
#[doc = "Field `OUTM` writer - Output mode"]
pub type OutmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn calval(&self) -> CalvalR {
        CalvalR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn calout(&self) -> CaloutR {
        CaloutR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output enable"]
    #[inline(always)]
    pub fn outen(&self) -> OutenR {
        OutenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output selection"]
    #[inline(always)]
    pub fn outsel(&self) -> OutselR {
        OutselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Calibration clock output selection"]
    #[inline(always)]
    pub fn ccos(&self) -> CcosR {
        CcosR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output mode"]
    #[inline(always)]
    pub fn outm(&self) -> OutmR {
        OutmR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn calval(&mut self) -> CalvalW<'_, RtccalSpec> {
        CalvalW::new(self, 0)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn calout(&mut self) -> CaloutW<'_, RtccalSpec> {
        CaloutW::new(self, 7)
    }
    #[doc = "Bit 8 - Output enable"]
    #[inline(always)]
    pub fn outen(&mut self) -> OutenW<'_, RtccalSpec> {
        OutenW::new(self, 8)
    }
    #[doc = "Bit 9 - Output selection"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OutselW<'_, RtccalSpec> {
        OutselW::new(self, 9)
    }
    #[doc = "Bit 10 - Calibration clock output selection"]
    #[inline(always)]
    pub fn ccos(&mut self) -> CcosW<'_, RtccalSpec> {
        CcosW::new(self, 10)
    }
    #[doc = "Bit 11 - Output mode"]
    #[inline(always)]
    pub fn outm(&mut self) -> OutmW<'_, RtccalSpec> {
        OutmW::new(self, 11)
    }
}
#[doc = "RTC clock calibration register (BPR_RTCCAL)\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtccalSpec;
impl crate::RegisterSpec for RtccalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccal::R`](R) reader structure"]
impl crate::Readable for RtccalSpec {}
#[doc = "`write(|w| ..)` method takes [`rtccal::W`](W) writer structure"]
impl crate::Writable for RtccalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTCCAL to value 0"]
impl crate::Resettable for RtccalSpec {}
