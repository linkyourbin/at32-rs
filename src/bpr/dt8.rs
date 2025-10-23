#[doc = "Register `DT8` reader"]
pub type R = crate::R<Dt8Spec>;
#[doc = "Register `DT8` writer"]
pub type W = crate::W<Dt8Spec>;
#[doc = "Field `DT8` reader - BPR data8"]
pub type Dt8R = crate::FieldReader<u16>;
#[doc = "Field `DT8` writer - BPR data8"]
pub type Dt8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data8"]
    #[inline(always)]
    pub fn dt8(&self) -> Dt8R {
        Dt8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data8"]
    #[inline(always)]
    pub fn dt8(&mut self) -> Dt8W<'_, Dt8Spec> {
        Dt8W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt8Spec;
impl crate::RegisterSpec for Dt8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt8::R`](R) reader structure"]
impl crate::Readable for Dt8Spec {}
#[doc = "`write(|w| ..)` method takes [`dt8::W`](W) writer structure"]
impl crate::Writable for Dt8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT8 to value 0"]
impl crate::Resettable for Dt8Spec {}
