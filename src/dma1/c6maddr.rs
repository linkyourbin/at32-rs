#[doc = "Register `C6MADDR` reader"]
pub type R = crate::R<C6maddrSpec>;
#[doc = "Register `C6MADDR` writer"]
pub type W = crate::W<C6maddrSpec>;
#[doc = "Field `MADDR` reader - Memory address"]
pub type MaddrR = crate::FieldReader<u32>;
#[doc = "Field `MADDR` writer - Memory address"]
pub type MaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn maddr(&self) -> MaddrR {
        MaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn maddr(&mut self) -> MaddrW<'_, C6maddrSpec> {
        MaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 6 memory address base register\n\nYou can [`read`](crate::Reg::read) this register and get [`c6maddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6maddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C6maddrSpec;
impl crate::RegisterSpec for C6maddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c6maddr::R`](R) reader structure"]
impl crate::Readable for C6maddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c6maddr::W`](W) writer structure"]
impl crate::Writable for C6maddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C6MADDR to value 0"]
impl crate::Resettable for C6maddrSpec {}
