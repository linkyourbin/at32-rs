#[doc = "Register `RSP3` reader"]
pub type R = crate::R<Rsp3Spec>;
#[doc = "Field `CARDSTS3` reader - CARDSTATUS3"]
pub type Cardsts3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS3"]
    #[inline(always)]
    pub fn cardsts3(&self) -> Cardsts3R {
        Cardsts3R::new(self.bits)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS3\n\nYou can [`read`](crate::Reg::read) this register and get [`rsp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsp3Spec;
impl crate::RegisterSpec for Rsp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp3::R`](R) reader structure"]
impl crate::Readable for Rsp3Spec {}
#[doc = "`reset()` method sets RSP3 to value 0"]
impl crate::Resettable for Rsp3Spec {}
