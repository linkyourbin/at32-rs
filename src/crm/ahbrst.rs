#[doc = "Register `AHBRST` reader"]
pub type R = crate::R<AhbrstSpec>;
#[doc = "Register `AHBRST` writer"]
pub type W = crate::W<AhbrstSpec>;
#[doc = "Field `EMACRST` reader - EMAC reset"]
pub type EmacrstR = crate::BitReader;
#[doc = "Field `EMACRST` writer - EMAC reset"]
pub type EmacrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - EMAC reset"]
    #[inline(always)]
    pub fn emacrst(&self) -> EmacrstR {
        EmacrstR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - EMAC reset"]
    #[inline(always)]
    pub fn emacrst(&mut self) -> EmacrstW<'_, AhbrstSpec> {
        EmacrstW::new(self, 14)
    }
}
#[doc = "AHB reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstSpec;
impl crate::RegisterSpec for AhbrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst::R`](R) reader structure"]
impl crate::Readable for AhbrstSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst::W`](W) writer structure"]
impl crate::Writable for AhbrstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBRST to value 0"]
impl crate::Resettable for AhbrstSpec {}
