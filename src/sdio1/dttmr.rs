#[doc = "Register `DTTMR` reader"]
pub type R = crate::R<DttmrSpec>;
#[doc = "Register `DTTMR` writer"]
pub type W = crate::W<DttmrSpec>;
#[doc = "Field `TIMEOUT` reader - Data timeout period"]
pub type TimeoutR = crate::FieldReader<u32>;
#[doc = "Field `TIMEOUT` writer - Data timeout period"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<'_, DttmrSpec> {
        TimeoutW::new(self, 0)
    }
}
#[doc = "Bits 31:0 = TIMEOUT: Data timeout period\n\nYou can [`read`](crate::Reg::read) this register and get [`dttmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dttmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DttmrSpec;
impl crate::RegisterSpec for DttmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dttmr::R`](R) reader structure"]
impl crate::Readable for DttmrSpec {}
#[doc = "`write(|w| ..)` method takes [`dttmr::W`](W) writer structure"]
impl crate::Writable for DttmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTTMR to value 0"]
impl crate::Resettable for DttmrSpec {}
