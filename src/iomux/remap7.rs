#[doc = "Register `REMAP7` reader"]
pub type R = crate::R<Remap7Spec>;
#[doc = "Register `REMAP7` writer"]
pub type W = crate::W<Remap7Spec>;
#[doc = "Field `EXT_SPIM_GMUX` reader - SPIM muxing"]
pub type ExtSpimGmuxR = crate::FieldReader;
#[doc = "Field `EXT_SPIM_GMUX` writer - SPIM muxing"]
pub type ExtSpimGmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXT_SPIM_GEN` reader - SPIM enable"]
pub type ExtSpimGenR = crate::BitReader;
#[doc = "Field `EXT_SPIM_GEN` writer - SPIM enable"]
pub type ExtSpimGenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETP_GMUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type Adc1EtpGmuxR = crate::BitReader;
#[doc = "Field `ADC1_ETP_GMUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type Adc1EtpGmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETO_GMUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type Adc1EtoGmuxR = crate::BitReader;
#[doc = "Field `ADC1_ETO_GMUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type Adc1EtoGmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETP_GMUX` reader - ADC2 external trigger preempted conversion muxing"]
pub type Adc2EtpGmuxR = crate::BitReader;
#[doc = "Field `ADC2_ETP_GMUX` writer - ADC2 external trigger preempted conversion muxing"]
pub type Adc2EtpGmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETO_GMUX` reader - ADC2 external trigger ordinary conversion muxing"]
pub type Adc2EtoGmuxR = crate::BitReader;
#[doc = "Field `ADC2_ETO_GMUX` writer - ADC2 external trigger ordinary conversion muxing"]
pub type Adc2EtoGmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWJTAG_GMUX` reader - Serial wire JTAG muxing"]
pub type SwjtagGmuxR = crate::FieldReader;
#[doc = "Field `SWJTAG_GMUX` writer - Serial wire JTAG muxing"]
pub type SwjtagGmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD01_GMUX` reader - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
pub type Pd01GmuxR = crate::BitReader;
#[doc = "Field `PD01_GMUX` writer - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
pub type Pd01GmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMC_GMUX` reader - XMC muxing"]
pub type XmcGmuxR = crate::FieldReader;
#[doc = "Field `XMC_GMUX` writer - XMC muxing"]
pub type XmcGmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `XMC_NADV_GMUX` reader - XMC_NADV muxing"]
pub type XmcNadvGmuxR = crate::BitReader;
#[doc = "Field `XMC_NADV_GMUX` writer - XMC_NADV muxing"]
pub type XmcNadvGmuxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SPIM muxing"]
    #[inline(always)]
    pub fn ext_spim_gmux(&self) -> ExtSpimGmuxR {
        ExtSpimGmuxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - SPIM enable"]
    #[inline(always)]
    pub fn ext_spim_gen(&self) -> ExtSpimGenR {
        ExtSpimGenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_gmux(&self) -> Adc1EtpGmuxR {
        Adc1EtpGmuxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_gmux(&self) -> Adc1EtoGmuxR {
        Adc1EtoGmuxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc2_etp_gmux(&self) -> Adc2EtpGmuxR {
        Adc2EtpGmuxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc2_eto_gmux(&self) -> Adc2EtoGmuxR {
        Adc2EtoGmuxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Serial wire JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_gmux(&self) -> SwjtagGmuxR {
        SwjtagGmuxR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_gmux(&self) -> Pd01GmuxR {
        Pd01GmuxR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - XMC muxing"]
    #[inline(always)]
    pub fn xmc_gmux(&self) -> XmcGmuxR {
        XmcGmuxR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - XMC_NADV muxing"]
    #[inline(always)]
    pub fn xmc_nadv_gmux(&self) -> XmcNadvGmuxR {
        XmcNadvGmuxR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPIM muxing"]
    #[inline(always)]
    pub fn ext_spim_gmux(&mut self) -> ExtSpimGmuxW<'_, Remap7Spec> {
        ExtSpimGmuxW::new(self, 0)
    }
    #[doc = "Bit 3 - SPIM enable"]
    #[inline(always)]
    pub fn ext_spim_gen(&mut self) -> ExtSpimGenW<'_, Remap7Spec> {
        ExtSpimGenW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_gmux(&mut self) -> Adc1EtpGmuxW<'_, Remap7Spec> {
        Adc1EtpGmuxW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_gmux(&mut self) -> Adc1EtoGmuxW<'_, Remap7Spec> {
        Adc1EtoGmuxW::new(self, 5)
    }
    #[doc = "Bit 8 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc2_etp_gmux(&mut self) -> Adc2EtpGmuxW<'_, Remap7Spec> {
        Adc2EtpGmuxW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc2_eto_gmux(&mut self) -> Adc2EtoGmuxW<'_, Remap7Spec> {
        Adc2EtoGmuxW::new(self, 9)
    }
    #[doc = "Bits 16:18 - Serial wire JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_gmux(&mut self) -> SwjtagGmuxW<'_, Remap7Spec> {
        SwjtagGmuxW::new(self, 16)
    }
    #[doc = "Bit 20 - PortD0/PortD1 mappingon OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_gmux(&mut self) -> Pd01GmuxW<'_, Remap7Spec> {
        Pd01GmuxW::new(self, 20)
    }
    #[doc = "Bits 24:26 - XMC muxing"]
    #[inline(always)]
    pub fn xmc_gmux(&mut self) -> XmcGmuxW<'_, Remap7Spec> {
        XmcGmuxW::new(self, 24)
    }
    #[doc = "Bit 27 - XMC_NADV muxing"]
    #[inline(always)]
    pub fn xmc_nadv_gmux(&mut self) -> XmcNadvGmuxW<'_, Remap7Spec> {
        XmcNadvGmuxW::new(self, 27)
    }
}
#[doc = "IO MUX remap register 7 (IOMUX_REMAP7)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap7Spec;
impl crate::RegisterSpec for Remap7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap7::R`](R) reader structure"]
impl crate::Readable for Remap7Spec {}
#[doc = "`write(|w| ..)` method takes [`remap7::W`](W) writer structure"]
impl crate::Writable for Remap7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP7 to value 0"]
impl crate::Resettable for Remap7Spec {}
