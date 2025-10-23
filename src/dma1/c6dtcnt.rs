#[doc = "Register `C6DTCNT` reader"]
pub type R = crate::R<C6dtcntSpec>;
#[doc = "Register `C6DTCNT` writer"]
pub type W = crate::W<C6dtcntSpec>;
#[doc = "Field `CNT` reader - Number of data to transfer"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Number of data to transfer"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<'_, C6dtcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "DMA channel 6 number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`c6dtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6dtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C6dtcntSpec;
impl crate::RegisterSpec for C6dtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c6dtcnt::R`](R) reader structure"]
impl crate::Readable for C6dtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`c6dtcnt::W`](W) writer structure"]
impl crate::Writable for C6dtcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C6DTCNT to value 0"]
impl crate::Resettable for C6dtcntSpec {}
