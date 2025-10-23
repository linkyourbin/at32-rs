#[doc = "Register `D1DTH12R` reader"]
pub type R = crate::R<D1dth12rSpec>;
#[doc = "Register `D1DTH12R` writer"]
pub type W = crate::W<D1dth12rSpec>;
#[doc = "Field `D1DT12R` reader - DAC1 12-bit right-aligned data"]
pub type D1dt12rR = crate::FieldReader<u16>;
#[doc = "Field `D1DT12R` writer - DAC1 12-bit right-aligned data"]
pub type D1dt12rW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn d1dt12r(&self) -> D1dt12rR {
        D1dt12rR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DAC1 12-bit right-aligned data"]
    #[inline(always)]
    pub fn d1dt12r(&mut self) -> D1dt12rW<'_, D1dth12rSpec> {
        D1dt12rW::new(self, 0)
    }
}
#[doc = "DAC1 12-bit right-aligned data holding register(DAC_D1DTH12R)\n\nYou can [`read`](crate::Reg::read) this register and get [`d1dth12r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1dth12r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1dth12rSpec;
impl crate::RegisterSpec for D1dth12rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1dth12r::R`](R) reader structure"]
impl crate::Readable for D1dth12rSpec {}
#[doc = "`write(|w| ..)` method takes [`d1dth12r::W`](W) writer structure"]
impl crate::Writable for D1dth12rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D1DTH12R to value 0"]
impl crate::Resettable for D1dth12rSpec {}
