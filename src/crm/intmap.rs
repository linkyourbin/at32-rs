#[doc = "Register `INTMAP` reader"]
pub type R = crate::R<IntmapSpec>;
#[doc = "Register `INTMAP` writer"]
pub type W = crate::W<IntmapSpec>;
#[doc = "Field `USB_INT_MAP` reader - USBDEV interrupt remap"]
pub type UsbIntMapR = crate::BitReader;
#[doc = "Field `USB_INT_MAP` writer - USBDEV interrupt remap"]
pub type UsbIntMapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USBDEV interrupt remap"]
    #[inline(always)]
    pub fn usb_int_map(&self) -> UsbIntMapR {
        UsbIntMapR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBDEV interrupt remap"]
    #[inline(always)]
    pub fn usb_int_map(&mut self) -> UsbIntMapW<'_, IntmapSpec> {
        UsbIntMapW::new(self, 0)
    }
}
#[doc = "Interrupt remap register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmapSpec;
impl crate::RegisterSpec for IntmapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmap::R`](R) reader structure"]
impl crate::Readable for IntmapSpec {}
#[doc = "`write(|w| ..)` method takes [`intmap::W`](W) writer structure"]
impl crate::Writable for IntmapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMAP to value 0"]
impl crate::Resettable for IntmapSpec {}
