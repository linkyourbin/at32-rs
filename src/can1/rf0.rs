#[doc = "Register `RF0` reader"]
pub type R = crate::R<Rf0Spec>;
#[doc = "Register `RF0` writer"]
pub type W = crate::W<Rf0Spec>;
#[doc = "Field `RF0MN` reader - Receive FIFO 0 message num"]
pub type Rf0mnR = crate::FieldReader;
#[doc = "Field `RF0FF` reader - Receive FIFO 0 full flag"]
pub type Rf0ffR = crate::BitReader;
#[doc = "Field `RF0FF` writer - Receive FIFO 0 full flag"]
pub type Rf0ffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0OF` reader - Receive FIFO 0 overflow flag"]
pub type Rf0ofR = crate::BitReader;
#[doc = "Field `RF0OF` writer - Receive FIFO 0 overflow flag"]
pub type Rf0ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0R` reader - Receive FIFO 0 release"]
pub type Rf0rR = crate::BitReader;
#[doc = "Field `RF0R` writer - Receive FIFO 0 release"]
pub type Rf0rW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Receive FIFO 0 message num"]
    #[inline(always)]
    pub fn rf0mn(&self) -> Rf0mnR {
        Rf0mnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO 0 full flag"]
    #[inline(always)]
    pub fn rf0ff(&self) -> Rf0ffR {
        Rf0ffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO 0 overflow flag"]
    #[inline(always)]
    pub fn rf0of(&self) -> Rf0ofR {
        Rf0ofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO 0 release"]
    #[inline(always)]
    pub fn rf0r(&self) -> Rf0rR {
        Rf0rR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO 0 full flag"]
    #[inline(always)]
    pub fn rf0ff(&mut self) -> Rf0ffW<'_, Rf0Spec> {
        Rf0ffW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO 0 overflow flag"]
    #[inline(always)]
    pub fn rf0of(&mut self) -> Rf0ofW<'_, Rf0Spec> {
        Rf0ofW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO 0 release"]
    #[inline(always)]
    pub fn rf0r(&mut self) -> Rf0rW<'_, Rf0Spec> {
        Rf0rW::new(self, 5)
    }
}
#[doc = "Receive FIFO 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rf0Spec;
impl crate::RegisterSpec for Rf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf0::R`](R) reader structure"]
impl crate::Readable for Rf0Spec {}
#[doc = "`write(|w| ..)` method takes [`rf0::W`](W) writer structure"]
impl crate::Writable for Rf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF0 to value 0"]
impl crate::Resettable for Rf0Spec {}
