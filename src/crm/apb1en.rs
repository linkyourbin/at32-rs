#[doc = "Register `APB1EN` reader"]
pub type R = crate::R<Apb1enSpec>;
#[doc = "Register `APB1EN` writer"]
pub type W = crate::W<Apb1enSpec>;
#[doc = "Field `TMR2EN` reader - Timer2 clock enable"]
pub type Tmr2enR = crate::BitReader;
#[doc = "Field `TMR2EN` writer - Timer2 clock enable"]
pub type Tmr2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3EN` reader - Timer3 clock enable"]
pub type Tmr3enR = crate::BitReader;
#[doc = "Field `TMR3EN` writer - Timer3 clock enable"]
pub type Tmr3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4EN` reader - Timer4 clock enable"]
pub type Tmr4enR = crate::BitReader;
#[doc = "Field `TMR4EN` writer - Timer4 clock enable"]
pub type Tmr4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5EN` reader - Timer5 clock enable"]
pub type Tmr5enR = crate::BitReader;
#[doc = "Field `TMR5EN` writer - Timer5 clock enable"]
pub type Tmr5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6EN` reader - Timer6 clock enable"]
pub type Tmr6enR = crate::BitReader;
#[doc = "Field `TMR6EN` writer - Timer6 clock enable"]
pub type Tmr6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR7EN` reader - Timer7 clock enable"]
pub type Tmr7enR = crate::BitReader;
#[doc = "Field `TMR7EN` writer - Timer7 clock enable"]
pub type Tmr7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR12EN` reader - Timer12 clock enable"]
pub type Tmr12enR = crate::BitReader;
#[doc = "Field `TMR12EN` writer - Timer12 clock enable"]
pub type Tmr12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR13EN` reader - Timer13 clock enable"]
pub type Tmr13enR = crate::BitReader;
#[doc = "Field `TMR13EN` writer - Timer13 clock enable"]
pub type Tmr13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14EN` reader - Timer14 clock enable"]
pub type Tmr14enR = crate::BitReader;
#[doc = "Field `TMR14EN` writer - Timer14 clock enable"]
pub type Tmr14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTEN` reader - Window watchdog timer clock enable"]
pub type WwdtenR = crate::BitReader;
#[doc = "Field `WWDTEN` writer - Window watchdog timer clock enable"]
pub type WwdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub type Spi2enR = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub type Spi2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3EN` reader - SPI3 clock enable"]
pub type Spi3enR = crate::BitReader;
#[doc = "Field `SPI3EN` writer - SPI3 clock enable"]
pub type Spi3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4EN` reader - SPI4 clock enable"]
pub type Spi4enR = crate::BitReader;
#[doc = "Field `SPI4EN` writer - SPI4 clock enable"]
pub type Spi4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 clock enable"]
pub type Usart2enR = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 clock enable"]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3EN` reader - USART3 clock enable"]
pub type Usart3enR = crate::BitReader;
#[doc = "Field `USART3EN` writer - USART3 clock enable"]
pub type Usart3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4EN` reader - UART4 clock enable"]
pub type Uart4enR = crate::BitReader;
#[doc = "Field `UART4EN` writer - UART4 clock enable"]
pub type Uart4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5EN` reader - UART5 clock enable"]
pub type Uart5enR = crate::BitReader;
#[doc = "Field `UART5EN` writer - UART5 clock enable"]
pub type Uart5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub type I2c1enR = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub type I2c2enR = crate::BitReader;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBEN` reader - USB clock enable"]
pub type UsbenR = crate::BitReader;
#[doc = "Field `USBEN` writer - USB clock enable"]
pub type UsbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1EN` reader - CAN1 clock enable"]
pub type Can1enR = crate::BitReader;
#[doc = "Field `CAN1EN` writer - CAN1 clock enable"]
pub type Can1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2EN` reader - CAN2 clock enable"]
pub type Can2enR = crate::BitReader;
#[doc = "Field `CAN2EN` writer - CAN2 clock enable"]
pub type Can2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BPREN` reader - Barrery powered domain register clock enable"]
pub type BprenR = crate::BitReader;
#[doc = "Field `BPREN` writer - Barrery powered domain register clock enable"]
pub type BprenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWCEN` reader - Power clock enable"]
pub type PwcenR = crate::BitReader;
#[doc = "Field `PWCEN` writer - Power clock enable"]
pub type PwcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACEN` reader - DAC clock enable"]
pub type DacenR = crate::BitReader;
#[doc = "Field `DACEN` writer - DAC clock enable"]
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer2 clock enable"]
    #[inline(always)]
    pub fn tmr2en(&self) -> Tmr2enR {
        Tmr2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    pub fn tmr3en(&self) -> Tmr3enR {
        Tmr3enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer4 clock enable"]
    #[inline(always)]
    pub fn tmr4en(&self) -> Tmr4enR {
        Tmr4enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer5 clock enable"]
    #[inline(always)]
    pub fn tmr5en(&self) -> Tmr5enR {
        Tmr5enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer6 clock enable"]
    #[inline(always)]
    pub fn tmr6en(&self) -> Tmr6enR {
        Tmr6enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer7 clock enable"]
    #[inline(always)]
    pub fn tmr7en(&self) -> Tmr7enR {
        Tmr7enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer12 clock enable"]
    #[inline(always)]
    pub fn tmr12en(&self) -> Tmr12enR {
        Tmr12enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer13 clock enable"]
    #[inline(always)]
    pub fn tmr13en(&self) -> Tmr13enR {
        Tmr13enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer14 clock enable"]
    #[inline(always)]
    pub fn tmr14en(&self) -> Tmr14enR {
        Tmr14enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdten(&self) -> WwdtenR {
        WwdtenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> Spi3enR {
        Spi3enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> Spi4enR {
        Spi4enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> Usart3enR {
        Usart3enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> Uart4enR {
        Uart4enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> Uart5enR {
        Uart5enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    pub fn usben(&self) -> UsbenR {
        UsbenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&self) -> Can1enR {
        Can1enR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CAN2 clock enable"]
    #[inline(always)]
    pub fn can2en(&self) -> Can2enR {
        Can2enR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Barrery powered domain register clock enable"]
    #[inline(always)]
    pub fn bpren(&self) -> BprenR {
        BprenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Power clock enable"]
    #[inline(always)]
    pub fn pwcen(&self) -> PwcenR {
        PwcenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 clock enable"]
    #[inline(always)]
    pub fn tmr2en(&mut self) -> Tmr2enW<'_, Apb1enSpec> {
        Tmr2enW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer3 clock enable"]
    #[inline(always)]
    pub fn tmr3en(&mut self) -> Tmr3enW<'_, Apb1enSpec> {
        Tmr3enW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer4 clock enable"]
    #[inline(always)]
    pub fn tmr4en(&mut self) -> Tmr4enW<'_, Apb1enSpec> {
        Tmr4enW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer5 clock enable"]
    #[inline(always)]
    pub fn tmr5en(&mut self) -> Tmr5enW<'_, Apb1enSpec> {
        Tmr5enW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer6 clock enable"]
    #[inline(always)]
    pub fn tmr6en(&mut self) -> Tmr6enW<'_, Apb1enSpec> {
        Tmr6enW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer7 clock enable"]
    #[inline(always)]
    pub fn tmr7en(&mut self) -> Tmr7enW<'_, Apb1enSpec> {
        Tmr7enW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer12 clock enable"]
    #[inline(always)]
    pub fn tmr12en(&mut self) -> Tmr12enW<'_, Apb1enSpec> {
        Tmr12enW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer13 clock enable"]
    #[inline(always)]
    pub fn tmr13en(&mut self) -> Tmr13enW<'_, Apb1enSpec> {
        Tmr13enW::new(self, 7)
    }
    #[doc = "Bit 8 - Timer14 clock enable"]
    #[inline(always)]
    pub fn tmr14en(&mut self) -> Tmr14enW<'_, Apb1enSpec> {
        Tmr14enW::new(self, 8)
    }
    #[doc = "Bit 11 - Window watchdog timer clock enable"]
    #[inline(always)]
    pub fn wwdten(&mut self) -> WwdtenW<'_, Apb1enSpec> {
        WwdtenW::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> Spi2enW<'_, Apb1enSpec> {
        Spi2enW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable"]
    #[inline(always)]
    pub fn spi3en(&mut self) -> Spi3enW<'_, Apb1enSpec> {
        Spi3enW::new(self, 15)
    }
    #[doc = "Bit 16 - SPI4 clock enable"]
    #[inline(always)]
    pub fn spi4en(&mut self) -> Spi4enW<'_, Apb1enSpec> {
        Spi4enW::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> Usart2enW<'_, Apb1enSpec> {
        Usart2enW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 clock enable"]
    #[inline(always)]
    pub fn usart3en(&mut self) -> Usart3enW<'_, Apb1enSpec> {
        Usart3enW::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 clock enable"]
    #[inline(always)]
    pub fn uart4en(&mut self) -> Uart4enW<'_, Apb1enSpec> {
        Uart4enW::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 clock enable"]
    #[inline(always)]
    pub fn uart5en(&mut self) -> Uart5enW<'_, Apb1enSpec> {
        Uart5enW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2c1enW<'_, Apb1enSpec> {
        I2c1enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2c2enW<'_, Apb1enSpec> {
        I2c2enW::new(self, 22)
    }
    #[doc = "Bit 23 - USB clock enable"]
    #[inline(always)]
    pub fn usben(&mut self) -> UsbenW<'_, Apb1enSpec> {
        UsbenW::new(self, 23)
    }
    #[doc = "Bit 25 - CAN1 clock enable"]
    #[inline(always)]
    pub fn can1en(&mut self) -> Can1enW<'_, Apb1enSpec> {
        Can1enW::new(self, 25)
    }
    #[doc = "Bit 26 - CAN2 clock enable"]
    #[inline(always)]
    pub fn can2en(&mut self) -> Can2enW<'_, Apb1enSpec> {
        Can2enW::new(self, 26)
    }
    #[doc = "Bit 27 - Barrery powered domain register clock enable"]
    #[inline(always)]
    pub fn bpren(&mut self) -> BprenW<'_, Apb1enSpec> {
        BprenW::new(self, 27)
    }
    #[doc = "Bit 28 - Power clock enable"]
    #[inline(always)]
    pub fn pwcen(&mut self) -> PwcenW<'_, Apb1enSpec> {
        PwcenW::new(self, 28)
    }
    #[doc = "Bit 29 - DAC clock enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DacenW<'_, Apb1enSpec> {
        DacenW::new(self, 29)
    }
}
#[doc = "APB1 peripheral clock enable register (CRM_APB1EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enSpec;
impl crate::RegisterSpec for Apb1enSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1en::R`](R) reader structure"]
impl crate::Readable for Apb1enSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1en::W`](W) writer structure"]
impl crate::Writable for Apb1enSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1EN to value 0"]
impl crate::Resettable for Apb1enSpec {}
