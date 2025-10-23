#[doc = "Register `DEVADDR` reader"]
pub type R = crate::R<DevaddrSpec>;
#[doc = "Register `DEVADDR` writer"]
pub type W = crate::W<DevaddrSpec>;
#[doc = "Field `ADDR` reader - Host assign device address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Host assign device address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CEN` reader - USB core enable"]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - USB core enable"]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Host assign device address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB core enable"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Host assign device address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, DevaddrSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 7 - USB core enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, DevaddrSpec> {
        CenW::new(self, 7)
    }
}
#[doc = "device address\n\nYou can [`read`](crate::Reg::read) this register and get [`devaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaddrSpec;
impl crate::RegisterSpec for DevaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devaddr::R`](R) reader structure"]
impl crate::Readable for DevaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`devaddr::W`](W) writer structure"]
impl crate::Writable for DevaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVADDR to value 0"]
impl crate::Resettable for DevaddrSpec {}
