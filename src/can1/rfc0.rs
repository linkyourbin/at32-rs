#[doc = "Register `RFC0` reader"]
pub type R = crate::R<Rfc0Spec>;
#[doc = "Field `RFDTL` reader - Receive FIFO data length"]
pub type RfdtlR = crate::FieldReader;
#[doc = "Field `RFFMN` reader - Receive FIFO filter match number"]
pub type RffmnR = crate::FieldReader;
#[doc = "Field `RFTS` reader - Receive FIFO time stamp"]
pub type RftsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Receive FIFO data length"]
    #[inline(always)]
    pub fn rfdtl(&self) -> RfdtlR {
        RfdtlR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO filter match number"]
    #[inline(always)]
    pub fn rffmn(&self) -> RffmnR {
        RffmnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Receive FIFO time stamp"]
    #[inline(always)]
    pub fn rfts(&self) -> RftsR {
        RftsR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Receive FIFO 0 data length and time stamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`rfc0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfc0Spec;
impl crate::RegisterSpec for Rfc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfc0::R`](R) reader structure"]
impl crate::Readable for Rfc0Spec {}
#[doc = "`reset()` method sets RFC0 to value 0"]
impl crate::Resettable for Rfc0Spec {}
