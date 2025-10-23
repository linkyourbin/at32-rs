#[doc = "Register `DTCNT` reader"]
pub type R = crate::R<DtcntSpec>;
#[doc = "Field `CNT` reader - Data count value"]
pub type CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "Bits 24:0 = DATACOUNT: Data count value\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtcntSpec;
impl crate::RegisterSpec for DtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtcnt::R`](R) reader structure"]
impl crate::Readable for DtcntSpec {}
#[doc = "`reset()` method sets DTCNT to value 0"]
impl crate::Resettable for DtcntSpec {}
