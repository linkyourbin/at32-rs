#[doc = "Register `DT16` reader"]
pub type R = crate::R<Dt16Spec>;
#[doc = "Register `DT16` writer"]
pub type W = crate::W<Dt16Spec>;
#[doc = "Field `DT16` reader - BPR data16"]
pub type Dt16R = crate::FieldReader<u16>;
#[doc = "Field `DT16` writer - BPR data16"]
pub type Dt16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data16"]
    #[inline(always)]
    pub fn dt16(&self) -> Dt16R {
        Dt16R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data16"]
    #[inline(always)]
    pub fn dt16(&mut self) -> Dt16W<'_, Dt16Spec> {
        Dt16W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt16Spec;
impl crate::RegisterSpec for Dt16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt16::R`](R) reader structure"]
impl crate::Readable for Dt16Spec {}
#[doc = "`write(|w| ..)` method takes [`dt16::W`](W) writer structure"]
impl crate::Writable for Dt16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT16 to value 0"]
impl crate::Resettable for Dt16Spec {}
