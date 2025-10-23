#[doc = "Register `MISC3` reader"]
pub type R = crate::R<Misc3Spec>;
#[doc = "Register `MISC3` writer"]
pub type W = crate::W<Misc3Spec>;
#[doc = "Field `AUTO_STEP_EN` reader - AUTO_STEP_EN"]
pub type AutoStepEnR = crate::FieldReader;
#[doc = "Field `AUTO_STEP_EN` writer - AUTO_STEP_EN"]
pub type AutoStepEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HICK_TO_USB` reader - HICK to usb clock"]
pub type HickToUsbR = crate::BitReader;
#[doc = "Field `HICK_TO_USB` writer - HICK to usb clock"]
pub type HickToUsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HICK_TO_SCLK` reader - HICK to system clock"]
pub type HickToSclkR = crate::BitReader;
#[doc = "Field `HICK_TO_SCLK` writer - HICK to system clock"]
pub type HickToSclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEXTDIV` reader - HEXT division"]
pub type HextdivR = crate::FieldReader;
#[doc = "Field `HEXTDIV` writer - HEXT division"]
pub type HextdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMAC_PPS_SEL` reader - Ethernet pulse width Select"]
pub type EmacPpsSelR = crate::BitReader;
#[doc = "Field `EMAC_PPS_SEL` writer - Ethernet pulse width Select"]
pub type EmacPpsSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    pub fn auto_step_en(&self) -> AutoStepEnR {
        AutoStepEnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - HICK to usb clock"]
    #[inline(always)]
    pub fn hick_to_usb(&self) -> HickToUsbR {
        HickToUsbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HICK to system clock"]
    #[inline(always)]
    pub fn hick_to_sclk(&self) -> HickToSclkR {
        HickToSclkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - HEXT division"]
    #[inline(always)]
    pub fn hextdiv(&self) -> HextdivR {
        HextdivR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Ethernet pulse width Select"]
    #[inline(always)]
    pub fn emac_pps_sel(&self) -> EmacPpsSelR {
        EmacPpsSelR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    pub fn auto_step_en(&mut self) -> AutoStepEnW<'_, Misc3Spec> {
        AutoStepEnW::new(self, 4)
    }
    #[doc = "Bit 8 - HICK to usb clock"]
    #[inline(always)]
    pub fn hick_to_usb(&mut self) -> HickToUsbW<'_, Misc3Spec> {
        HickToUsbW::new(self, 8)
    }
    #[doc = "Bit 9 - HICK to system clock"]
    #[inline(always)]
    pub fn hick_to_sclk(&mut self) -> HickToSclkW<'_, Misc3Spec> {
        HickToSclkW::new(self, 9)
    }
    #[doc = "Bits 12:13 - HEXT division"]
    #[inline(always)]
    pub fn hextdiv(&mut self) -> HextdivW<'_, Misc3Spec> {
        HextdivW::new(self, 12)
    }
    #[doc = "Bit 15 - Ethernet pulse width Select"]
    #[inline(always)]
    pub fn emac_pps_sel(&mut self) -> EmacPpsSelW<'_, Misc3Spec> {
        EmacPpsSelW::new(self, 15)
    }
}
#[doc = "Miscellaneous register3\n\nYou can [`read`](crate::Reg::read) this register and get [`misc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Misc3Spec;
impl crate::RegisterSpec for Misc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc3::R`](R) reader structure"]
impl crate::Readable for Misc3Spec {}
#[doc = "`write(|w| ..)` method takes [`misc3::W`](W) writer structure"]
impl crate::Writable for Misc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC3 to value 0"]
impl crate::Resettable for Misc3Spec {}
