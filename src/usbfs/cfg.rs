#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SofoutenR = crate::BitReader;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SofoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUO` reader - DP pullup off"]
pub type PuoR = crate::BitReader;
#[doc = "Field `PUO` writer - DP pullup off"]
pub type PuoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&self) -> SofoutenR {
        SofoutenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DP pullup off"]
    #[inline(always)]
    pub fn puo(&self) -> PuoR {
        PuoR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&mut self) -> SofoutenW<'_, CfgSpec> {
        SofoutenW::new(self, 0)
    }
    #[doc = "Bit 1 - DP pullup off"]
    #[inline(always)]
    pub fn puo(&mut self) -> PuoW<'_, CfgSpec> {
        PuoW::new(self, 1)
    }
}
#[doc = "CFG control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
