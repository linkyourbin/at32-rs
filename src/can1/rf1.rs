#[doc = "Register `RF1` reader"]
pub type R = crate::R<Rf1Spec>;
#[doc = "Register `RF1` writer"]
pub type W = crate::W<Rf1Spec>;
#[doc = "Field `RF1MN` reader - Receive FIFO 1 message num"]
pub type Rf1mnR = crate::FieldReader;
#[doc = "Field `RF1FF` reader - Receive FIFO 1 full flag"]
pub type Rf1ffR = crate::BitReader;
#[doc = "Field `RF1FF` writer - Receive FIFO 1 full flag"]
pub type Rf1ffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1OF` reader - Receive FIFO 1 overflow flag"]
pub type Rf1ofR = crate::BitReader;
#[doc = "Field `RF1OF` writer - Receive FIFO 1 overflow flag"]
pub type Rf1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1R` reader - Receive FIFO 1 release"]
pub type Rf1rR = crate::BitReader;
#[doc = "Field `RF1R` writer - Receive FIFO 1 release"]
pub type Rf1rW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Receive FIFO 1 message num"]
    #[inline(always)]
    pub fn rf1mn(&self) -> Rf1mnR {
        Rf1mnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Receive FIFO 1 full flag"]
    #[inline(always)]
    pub fn rf1ff(&self) -> Rf1ffR {
        Rf1ffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO 1 overflow flag"]
    #[inline(always)]
    pub fn rf1of(&self) -> Rf1ofR {
        Rf1ofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO 1 release"]
    #[inline(always)]
    pub fn rf1r(&self) -> Rf1rR {
        Rf1rR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Receive FIFO 1 full flag"]
    #[inline(always)]
    pub fn rf1ff(&mut self) -> Rf1ffW<'_, Rf1Spec> {
        Rf1ffW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO 1 overflow flag"]
    #[inline(always)]
    pub fn rf1of(&mut self) -> Rf1ofW<'_, Rf1Spec> {
        Rf1ofW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO 1 release"]
    #[inline(always)]
    pub fn rf1r(&mut self) -> Rf1rW<'_, Rf1Spec> {
        Rf1rW::new(self, 5)
    }
}
#[doc = "Receive FIFO 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rf1Spec;
impl crate::RegisterSpec for Rf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rf1::R`](R) reader structure"]
impl crate::Readable for Rf1Spec {}
#[doc = "`write(|w| ..)` method takes [`rf1::W`](W) writer structure"]
impl crate::Writable for Rf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RF1 to value 0"]
impl crate::Resettable for Rf1Spec {}
