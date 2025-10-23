#[doc = "Register `REMAP6` reader"]
pub type R = crate::R<Remap6Spec>;
#[doc = "Register `REMAP6` writer"]
pub type W = crate::W<Remap6Spec>;
#[doc = "Field `CAN1_GMUX` reader - CAN1 muxing"]
pub type Can1GmuxR = crate::FieldReader;
#[doc = "Field `CAN1_GMUX` writer - CAN1 muxing"]
pub type Can1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CAN2_GMUX` reader - CAN2 muxing"]
pub type Can2GmuxR = crate::FieldReader;
#[doc = "Field `CAN2_GMUX` writer - CAN2 muxing"]
pub type Can2GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO2_GMUX` reader - SDIO2 muxing"]
pub type Sdio2GmuxR = crate::FieldReader;
#[doc = "Field `SDIO2_GMUX` writer - SDIO2 muxing"]
pub type Sdio2GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USART1_GMUX` reader - USART1 muxing"]
pub type Usart1GmuxR = crate::FieldReader;
#[doc = "Field `USART1_GMUX` writer - USART1 muxing"]
pub type Usart1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USART2_GMUX` reader - USART2 muxing"]
pub type Usart2GmuxR = crate::FieldReader;
#[doc = "Field `USART2_GMUX` writer - USART2 muxing"]
pub type Usart2GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USART3_GMUX` reader - USART3 muxing"]
pub type Usart3GmuxR = crate::FieldReader;
#[doc = "Field `USART3_GMUX` writer - USART3 muxing"]
pub type Usart3GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UART4_GMUX` reader - UART4 muxing"]
pub type Uart4GmuxR = crate::FieldReader;
#[doc = "Field `UART4_GMUX` writer - UART4 muxing"]
pub type Uart4GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    pub fn can1_gmux(&self) -> Can1GmuxR {
        Can1GmuxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CAN2 muxing"]
    #[inline(always)]
    pub fn can2_gmux(&self) -> Can2GmuxR {
        Can2GmuxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SDIO2 muxing"]
    #[inline(always)]
    pub fn sdio2_gmux(&self) -> Sdio2GmuxR {
        Sdio2GmuxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_gmux(&self) -> Usart1GmuxR {
        Usart1GmuxR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - USART2 muxing"]
    #[inline(always)]
    pub fn usart2_gmux(&self) -> Usart2GmuxR {
        Usart2GmuxR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_gmux(&self) -> Usart3GmuxR {
        Usart3GmuxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - UART4 muxing"]
    #[inline(always)]
    pub fn uart4_gmux(&self) -> Uart4GmuxR {
        Uart4GmuxR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    pub fn can1_gmux(&mut self) -> Can1GmuxW<'_, Remap6Spec> {
        Can1GmuxW::new(self, 0)
    }
    #[doc = "Bits 4:7 - CAN2 muxing"]
    #[inline(always)]
    pub fn can2_gmux(&mut self) -> Can2GmuxW<'_, Remap6Spec> {
        Can2GmuxW::new(self, 4)
    }
    #[doc = "Bits 12:15 - SDIO2 muxing"]
    #[inline(always)]
    pub fn sdio2_gmux(&mut self) -> Sdio2GmuxW<'_, Remap6Spec> {
        Sdio2GmuxW::new(self, 12)
    }
    #[doc = "Bits 16:19 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_gmux(&mut self) -> Usart1GmuxW<'_, Remap6Spec> {
        Usart1GmuxW::new(self, 16)
    }
    #[doc = "Bits 20:23 - USART2 muxing"]
    #[inline(always)]
    pub fn usart2_gmux(&mut self) -> Usart2GmuxW<'_, Remap6Spec> {
        Usart2GmuxW::new(self, 20)
    }
    #[doc = "Bits 24:27 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_gmux(&mut self) -> Usart3GmuxW<'_, Remap6Spec> {
        Usart3GmuxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - UART4 muxing"]
    #[inline(always)]
    pub fn uart4_gmux(&mut self) -> Uart4GmuxW<'_, Remap6Spec> {
        Uart4GmuxW::new(self, 28)
    }
}
#[doc = "IO MUX remap register 6 (IOMUX_REMAP6)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap6Spec;
impl crate::RegisterSpec for Remap6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap6::R`](R) reader structure"]
impl crate::Readable for Remap6Spec {}
#[doc = "`write(|w| ..)` method takes [`remap6::W`](W) writer structure"]
impl crate::Writable for Remap6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP6 to value 0"]
impl crate::Resettable for Remap6Spec {}
