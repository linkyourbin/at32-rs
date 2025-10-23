#[doc = "Register `EXINTC3` reader"]
pub type R = crate::R<Exintc3Spec>;
#[doc = "Register `EXINTC3` writer"]
pub type W = crate::W<Exintc3Spec>;
#[doc = "Field `EXINT8` reader - Configure EXINT8 source"]
pub type Exint8R = crate::FieldReader;
#[doc = "Field `EXINT8` writer - Configure EXINT8 source"]
pub type Exint8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT9` reader - Configure EXINT9 source"]
pub type Exint9R = crate::FieldReader;
#[doc = "Field `EXINT9` writer - Configure EXINT9 source"]
pub type Exint9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT10` reader - Configure EXINT10 source"]
pub type Exint10R = crate::FieldReader;
#[doc = "Field `EXINT10` writer - Configure EXINT10 source"]
pub type Exint10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXINT11` reader - Configure EXINT11 source"]
pub type Exint11R = crate::FieldReader;
#[doc = "Field `EXINT11` writer - Configure EXINT11 source"]
pub type Exint11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configure EXINT8 source"]
    #[inline(always)]
    pub fn exint8(&self) -> Exint8R {
        Exint8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configure EXINT9 source"]
    #[inline(always)]
    pub fn exint9(&self) -> Exint9R {
        Exint9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configure EXINT10 source"]
    #[inline(always)]
    pub fn exint10(&self) -> Exint10R {
        Exint10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configure EXINT11 source"]
    #[inline(always)]
    pub fn exint11(&self) -> Exint11R {
        Exint11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configure EXINT8 source"]
    #[inline(always)]
    pub fn exint8(&mut self) -> Exint8W<'_, Exintc3Spec> {
        Exint8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configure EXINT9 source"]
    #[inline(always)]
    pub fn exint9(&mut self) -> Exint9W<'_, Exintc3Spec> {
        Exint9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configure EXINT10 source"]
    #[inline(always)]
    pub fn exint10(&mut self) -> Exint10W<'_, Exintc3Spec> {
        Exint10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Configure EXINT11 source"]
    #[inline(always)]
    pub fn exint11(&mut self) -> Exint11W<'_, Exintc3Spec> {
        Exint11W::new(self, 12)
    }
}
#[doc = "External interrupt configuration register 3 (IOMUX_EXINTC3)\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exintc3Spec;
impl crate::RegisterSpec for Exintc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc3::R`](R) reader structure"]
impl crate::Readable for Exintc3Spec {}
#[doc = "`write(|w| ..)` method takes [`exintc3::W`](W) writer structure"]
impl crate::Writable for Exintc3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXINTC3 to value 0"]
impl crate::Resettable for Exintc3Spec {}
