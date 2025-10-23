#[doc = "Register `DTLEN` reader"]
pub type R = crate::R<DtlenSpec>;
#[doc = "Register `DTLEN` writer"]
pub type W = crate::W<DtlenSpec>;
#[doc = "Field `DTLEN` reader - Data length value"]
pub type DtlenR = crate::FieldReader<u32>;
#[doc = "Field `DTLEN` writer - Data length value"]
pub type DtlenW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    pub fn dtlen(&self) -> DtlenR {
        DtlenR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    pub fn dtlen(&mut self) -> DtlenW<'_, DtlenSpec> {
        DtlenW::new(self, 0)
    }
}
#[doc = "Bits 24:0 = DATALENGTH: Data length value\n\nYou can [`read`](crate::Reg::read) this register and get [`dtlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtlenSpec;
impl crate::RegisterSpec for DtlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtlen::R`](R) reader structure"]
impl crate::Readable for DtlenSpec {}
#[doc = "`write(|w| ..)` method takes [`dtlen::W`](W) writer structure"]
impl crate::Writable for DtlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTLEN to value 0"]
impl crate::Resettable for DtlenSpec {}
