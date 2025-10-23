#[doc = "Register `DT7` reader"]
pub type R = crate::R<Dt7Spec>;
#[doc = "Register `DT7` writer"]
pub type W = crate::W<Dt7Spec>;
#[doc = "Field `DT7` reader - BPR data7"]
pub type Dt7R = crate::FieldReader<u16>;
#[doc = "Field `DT7` writer - BPR data7"]
pub type Dt7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data7"]
    #[inline(always)]
    pub fn dt7(&self) -> Dt7R {
        Dt7R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data7"]
    #[inline(always)]
    pub fn dt7(&mut self) -> Dt7W<'_, Dt7Spec> {
        Dt7W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt7Spec;
impl crate::RegisterSpec for Dt7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt7::R`](R) reader structure"]
impl crate::Readable for Dt7Spec {}
#[doc = "`write(|w| ..)` method takes [`dt7::W`](W) writer structure"]
impl crate::Writable for Dt7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT7 to value 0"]
impl crate::Resettable for Dt7Spec {}
