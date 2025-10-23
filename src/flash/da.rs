#[doc = "Register `DA` writer"]
pub type W = crate::W<DaSpec>;
#[doc = "Field `FDA` writer - Flash decryption address"]
pub type FdaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash decryption address"]
    #[inline(always)]
    pub fn fda(&mut self) -> FdaW<'_, DaSpec> {
        FdaW::new(self, 0)
    }
}
#[doc = "Spim decryption address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`da::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaSpec;
impl crate::RegisterSpec for DaSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`da::W`](W) writer structure"]
impl crate::Writable for DaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DA to value 0"]
impl crate::Resettable for DaSpec {}
