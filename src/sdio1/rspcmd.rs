#[doc = "Register `RSPCMD` reader"]
pub type R = crate::R<RspcmdSpec>;
#[doc = "Field `RSPCMD` reader - RSPCMD"]
pub type RspcmdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - RSPCMD"]
    #[inline(always)]
    pub fn rspcmd(&self) -> RspcmdR {
        RspcmdR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "SDIO command register\n\nYou can [`read`](crate::Reg::read) this register and get [`rspcmd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RspcmdSpec;
impl crate::RegisterSpec for RspcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rspcmd::R`](R) reader structure"]
impl crate::Readable for RspcmdSpec {}
#[doc = "`reset()` method sets RSPCMD to value 0"]
impl crate::Resettable for RspcmdSpec {}
