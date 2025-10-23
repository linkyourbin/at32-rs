#[doc = "Register `SLIB_STS1` reader"]
pub type R = crate::R<SlibSts1Spec>;
#[doc = "Register `SLIB_STS1` writer"]
pub type W = crate::W<SlibSts1Spec>;
#[doc = "Field `SLIB_SS` reader - sLib start sector"]
pub type SlibSsR = crate::FieldReader<u16>;
#[doc = "Field `SLIB_DAT_SS` reader - sLib data start sector"]
pub type SlibDatSsR = crate::FieldReader<u16>;
#[doc = "Field `SLIB_ES` reader - sLib end sector"]
pub type SlibEsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - sLib start sector"]
    #[inline(always)]
    pub fn slib_ss(&self) -> SlibSsR {
        SlibSsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - sLib data start sector"]
    #[inline(always)]
    pub fn slib_dat_ss(&self) -> SlibDatSsR {
        SlibDatSsR::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:31 - sLib end sector"]
    #[inline(always)]
    pub fn slib_es(&self) -> SlibEsR {
        SlibEsR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {}
#[doc = "sLib status 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`slib_sts1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slib_sts1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSts1Spec;
impl crate::RegisterSpec for SlibSts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts1::R`](R) reader structure"]
impl crate::Readable for SlibSts1Spec {}
#[doc = "`write(|w| ..)` method takes [`slib_sts1::W`](W) writer structure"]
impl crate::Writable for SlibSts1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLIB_STS1 to value 0"]
impl crate::Resettable for SlibSts1Spec {}
