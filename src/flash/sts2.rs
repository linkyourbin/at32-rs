#[doc = "Register `STS2` reader"]
pub type R = crate::R<Sts2Spec>;
#[doc = "Register `STS2` writer"]
pub type W = crate::W<Sts2Spec>;
#[doc = "Field `OBF` reader - Operate busy flag"]
pub type ObfR = crate::BitReader;
#[doc = "Field `PRGMERR` reader - program error"]
pub type PrgmerrR = crate::BitReader;
#[doc = "Field `PRGMERR` writer - program error"]
pub type PrgmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPPERR` reader - Erase/program protection error"]
pub type EpperrR = crate::BitReader;
#[doc = "Field `EPPERR` writer - Erase/program protection error"]
pub type EpperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODF` reader - Operate done flag"]
pub type OdfR = crate::BitReader;
#[doc = "Field `ODF` writer - Operate done flag"]
pub type OdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operate busy flag"]
    #[inline(always)]
    pub fn obf(&self) -> ObfR {
        ObfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    pub fn prgmerr(&self) -> PrgmerrR {
        PrgmerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    pub fn epperr(&self) -> EpperrR {
        EpperrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    pub fn odf(&self) -> OdfR {
        OdfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    pub fn prgmerr(&mut self) -> PrgmerrW<'_, Sts2Spec> {
        PrgmerrW::new(self, 2)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    pub fn epperr(&mut self) -> EpperrW<'_, Sts2Spec> {
        EpperrW::new(self, 4)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    pub fn odf(&mut self) -> OdfW<'_, Sts2Spec> {
        OdfW::new(self, 5)
    }
}
#[doc = "Status 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`sts2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sts2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts2Spec;
impl crate::RegisterSpec for Sts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts2::R`](R) reader structure"]
impl crate::Readable for Sts2Spec {}
#[doc = "`write(|w| ..)` method takes [`sts2::W`](W) writer structure"]
impl crate::Writable for Sts2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STS2 to value 0"]
impl crate::Resettable for Sts2Spec {}
