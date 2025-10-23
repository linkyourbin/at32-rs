#[doc = "Register `BUFTBL` reader"]
pub type R = crate::R<BuftblSpec>;
#[doc = "Register `BUFTBL` writer"]
pub type W = crate::W<BuftblSpec>;
#[doc = "Field `BTADDR` reader - Endpoint buffer table start address"]
pub type BtaddrR = crate::FieldReader<u16>;
#[doc = "Field `BTADDR` writer - Endpoint buffer table start address"]
pub type BtaddrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - Endpoint buffer table start address"]
    #[inline(always)]
    pub fn btaddr(&self) -> BtaddrR {
        BtaddrR::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Endpoint buffer table start address"]
    #[inline(always)]
    pub fn btaddr(&mut self) -> BtaddrW<'_, BuftblSpec> {
        BtaddrW::new(self, 3)
    }
}
#[doc = "Buffer table address\n\nYou can [`read`](crate::Reg::read) this register and get [`buftbl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buftbl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuftblSpec;
impl crate::RegisterSpec for BuftblSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buftbl::R`](R) reader structure"]
impl crate::Readable for BuftblSpec {}
#[doc = "`write(|w| ..)` method takes [`buftbl::W`](W) writer structure"]
impl crate::Writable for BuftblSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFTBL to value 0"]
impl crate::Resettable for BuftblSpec {}
