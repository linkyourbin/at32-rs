#[doc = "Register `BUF` reader"]
pub type R = crate::R<BufSpec>;
#[doc = "Register `BUF` writer"]
pub type W = crate::W<BufSpec>;
#[doc = "Field `DT` reader - Buffer data"]
pub type DtR = crate::FieldReader<u32>;
#[doc = "Field `DT` writer - Buffer data"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer data"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer data"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, BufSpec> {
        DtW::new(self, 0)
    }
}
#[doc = "bits 31:0 = Buffer Data: Receive and transmit buffer data\n\nYou can [`read`](crate::Reg::read) this register and get [`buf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufSpec;
impl crate::RegisterSpec for BufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf::R`](R) reader structure"]
impl crate::Readable for BufSpec {}
#[doc = "`write(|w| ..)` method takes [`buf::W`](W) writer structure"]
impl crate::Writable for BufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF to value 0"]
impl crate::Resettable for BufSpec {}
