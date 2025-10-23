#[doc = "Register `RSP4` reader"]
pub type R = crate::R<Rsp4Spec>;
#[doc = "Field `CARDSTS4` reader - CARDSTATUS4"]
pub type Cardsts4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS4"]
    #[inline(always)]
    pub fn cardsts4(&self) -> Cardsts4R {
        Cardsts4R::new(self.bits)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS4\n\nYou can [`read`](crate::Reg::read) this register and get [`rsp4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rsp4Spec;
impl crate::RegisterSpec for Rsp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsp4::R`](R) reader structure"]
impl crate::Readable for Rsp4Spec {}
#[doc = "`reset()` method sets RSP4 to value 0"]
impl crate::Resettable for Rsp4Spec {}
