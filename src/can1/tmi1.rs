#[doc = "Register `TMI1` reader"]
pub type R = crate::R<Tmi1Spec>;
#[doc = "Register `TMI1` writer"]
pub type W = crate::W<Tmi1Spec>;
#[doc = "Field `TMSR` reader - Transmit mailbox send request"]
pub type TmsrR = crate::BitReader;
#[doc = "Field `TMSR` writer - Transmit mailbox send request"]
pub type TmsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMFRSEL` reader - Transmit mailbox frame type select"]
pub type TmfrselR = crate::BitReader;
#[doc = "Field `TMFRSEL` writer - Transmit mailbox frame type select"]
pub type TmfrselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIDSEL` reader - Transmit mailbox identifier type select"]
pub type TmidselR = crate::BitReader;
#[doc = "Field `TMIDSEL` writer - Transmit mailbox identifier type select"]
pub type TmidselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMEID` reader - Ttransmit mailbox extended identifier"]
pub type TmeidR = crate::FieldReader<u32>;
#[doc = "Field `TMEID` writer - Ttransmit mailbox extended identifier"]
pub type TmeidW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `TMSID` reader - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type TmsidR = crate::FieldReader<u16>;
#[doc = "Field `TMSID` writer - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type TmsidW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Transmit mailbox send request"]
    #[inline(always)]
    pub fn tmsr(&self) -> TmsrR {
        TmsrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit mailbox frame type select"]
    #[inline(always)]
    pub fn tmfrsel(&self) -> TmfrselR {
        TmfrselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit mailbox identifier type select"]
    #[inline(always)]
    pub fn tmidsel(&self) -> TmidselR {
        TmidselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - Ttransmit mailbox extended identifier"]
    #[inline(always)]
    pub fn tmeid(&self) -> TmeidR {
        TmeidR::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - Transmit mailbox standard identifier or extended identifier high bytes"]
    #[inline(always)]
    pub fn tmsid(&self) -> TmsidR {
        TmsidR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox send request"]
    #[inline(always)]
    pub fn tmsr(&mut self) -> TmsrW<'_, Tmi1Spec> {
        TmsrW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit mailbox frame type select"]
    #[inline(always)]
    pub fn tmfrsel(&mut self) -> TmfrselW<'_, Tmi1Spec> {
        TmfrselW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit mailbox identifier type select"]
    #[inline(always)]
    pub fn tmidsel(&mut self) -> TmidselW<'_, Tmi1Spec> {
        TmidselW::new(self, 2)
    }
    #[doc = "Bits 3:20 - Ttransmit mailbox extended identifier"]
    #[inline(always)]
    pub fn tmeid(&mut self) -> TmeidW<'_, Tmi1Spec> {
        TmeidW::new(self, 3)
    }
    #[doc = "Bits 21:31 - Transmit mailbox standard identifier or extended identifier high bytes"]
    #[inline(always)]
    pub fn tmsid(&mut self) -> TmsidW<'_, Tmi1Spec> {
        TmsidW::new(self, 21)
    }
}
#[doc = "Transmit mailbox 1 identifier register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmi1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmi1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmi1Spec;
impl crate::RegisterSpec for Tmi1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmi1::R`](R) reader structure"]
impl crate::Readable for Tmi1Spec {}
#[doc = "`write(|w| ..)` method takes [`tmi1::W`](W) writer structure"]
impl crate::Writable for Tmi1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMI1 to value 0"]
impl crate::Resettable for Tmi1Spec {}
