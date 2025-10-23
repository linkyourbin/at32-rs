#[doc = "Register `TMC1` reader"]
pub type R = crate::R<Tmc1Spec>;
#[doc = "Register `TMC1` writer"]
pub type W = crate::W<Tmc1Spec>;
#[doc = "Field `TMDTBL` reader - Transmit mailbox data byte length"]
pub type TmdtblR = crate::FieldReader;
#[doc = "Field `TMDTBL` writer - Transmit mailbox data byte length"]
pub type TmdtblW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMTSTEN` reader - Transmit mailbox time stamp transmit enable"]
pub type TmtstenR = crate::BitReader;
#[doc = "Field `TMTSTEN` writer - Transmit mailbox time stamp transmit enable"]
pub type TmtstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMTS` reader - Transmit mailbox time stamp"]
pub type TmtsR = crate::FieldReader<u16>;
#[doc = "Field `TMTS` writer - Transmit mailbox time stamp"]
pub type TmtsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - Transmit mailbox data byte length"]
    #[inline(always)]
    pub fn tmdtbl(&self) -> TmdtblR {
        TmdtblR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Transmit mailbox time stamp transmit enable"]
    #[inline(always)]
    pub fn tmtsten(&self) -> TmtstenR {
        TmtstenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Transmit mailbox time stamp"]
    #[inline(always)]
    pub fn tmts(&self) -> TmtsR {
        TmtsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmit mailbox data byte length"]
    #[inline(always)]
    pub fn tmdtbl(&mut self) -> TmdtblW<'_, Tmc1Spec> {
        TmdtblW::new(self, 0)
    }
    #[doc = "Bit 8 - Transmit mailbox time stamp transmit enable"]
    #[inline(always)]
    pub fn tmtsten(&mut self) -> TmtstenW<'_, Tmc1Spec> {
        TmtstenW::new(self, 8)
    }
    #[doc = "Bits 16:31 - Transmit mailbox time stamp"]
    #[inline(always)]
    pub fn tmts(&mut self) -> TmtsW<'_, Tmc1Spec> {
        TmtsW::new(self, 16)
    }
}
#[doc = "Transmit mailbox 1 data length and time stamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmc1Spec;
impl crate::RegisterSpec for Tmc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmc1::R`](R) reader structure"]
impl crate::Readable for Tmc1Spec {}
#[doc = "`write(|w| ..)` method takes [`tmc1::W`](W) writer structure"]
impl crate::Writable for Tmc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMC1 to value 0"]
impl crate::Resettable for Tmc1Spec {}
