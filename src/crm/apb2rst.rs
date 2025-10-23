#[doc = "Register `APB2RST` reader"]
pub type R = crate::R<Apb2rstSpec>;
#[doc = "Register `APB2RST` writer"]
pub type W = crate::W<Apb2rstSpec>;
#[doc = "Field `IOMUXRST` reader - MUX function I/O reset"]
pub type IomuxrstR = crate::BitReader;
#[doc = "Field `IOMUXRST` writer - MUX function I/O reset"]
pub type IomuxrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXINTRST` reader - External interrupt reset"]
pub type ExintrstR = crate::BitReader;
#[doc = "Field `EXINTRST` writer - External interrupt reset"]
pub type ExintrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOARST` reader - IO port A reset"]
pub type GpioarstR = crate::BitReader;
#[doc = "Field `GPIOARST` writer - IO port A reset"]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - IO port B reset"]
pub type GpiobrstR = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - IO port B reset"]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - IO port C reset"]
pub type GpiocrstR = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - IO port C reset"]
pub type GpiocrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - IO port D reset"]
pub type GpiodrstR = crate::BitReader;
#[doc = "Field `GPIODRST` writer - IO port D reset"]
pub type GpiodrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - IO port E reset"]
pub type GpioerstR = crate::BitReader;
#[doc = "Field `GPIOERST` writer - IO port E reset"]
pub type GpioerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1RST` reader - ADC1 reset"]
pub type Adc1rstR = crate::BitReader;
#[doc = "Field `ADC1RST` writer - ADC1 reset"]
pub type Adc1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2RST` reader - ADC2 reset"]
pub type Adc2rstR = crate::BitReader;
#[doc = "Field `ADC2RST` writer - ADC2 reset"]
pub type Adc2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1RST` reader - Timer1 reset"]
pub type Tmr1rstR = crate::BitReader;
#[doc = "Field `TMR1RST` writer - Timer1 reset"]
pub type Tmr1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8RST` reader - Timer8 reset"]
pub type Tmr8rstR = crate::BitReader;
#[doc = "Field `TMR8RST` writer - Timer8 reset"]
pub type Tmr8rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3RST` reader - ADC3 reset"]
pub type Adc3rstR = crate::BitReader;
#[doc = "Field `ADC3RST` writer - ADC3 reset"]
pub type Adc3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9RST` reader - Timer9 reset"]
pub type Tmr9rstR = crate::BitReader;
#[doc = "Field `TMR9RST` writer - Timer9 reset"]
pub type Tmr9rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10RST` reader - Timer10 reset"]
pub type Tmr10rstR = crate::BitReader;
#[doc = "Field `TMR10RST` writer - Timer10 reset"]
pub type Tmr10rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11RST` reader - Timer11 reset"]
pub type Tmr11rstR = crate::BitReader;
#[doc = "Field `TMR11RST` writer - Timer11 reset"]
pub type Tmr11rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCRST` reader - ACC reset"]
pub type AccrstR = crate::BitReader;
#[doc = "Field `ACCRST` writer - ACC reset"]
pub type AccrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3RST` reader - I2C3 reset"]
pub type I2c3rstR = crate::BitReader;
#[doc = "Field `I2C3RST` writer - I2C3 reset"]
pub type I2c3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6RST` reader - USART6 reset"]
pub type Usart6rstR = crate::BitReader;
#[doc = "Field `USART6RST` writer - USART6 reset"]
pub type Usart6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7RST` reader - UART7 reset"]
pub type Uart7rstR = crate::BitReader;
#[doc = "Field `UART7RST` writer - UART7 reset"]
pub type Uart7rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8RST` reader - UART8 reset"]
pub type Uart8rstR = crate::BitReader;
#[doc = "Field `UART8RST` writer - UART8 reset"]
pub type Uart8rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MUX function I/O reset"]
    #[inline(always)]
    pub fn iomuxrst(&self) -> IomuxrstR {
        IomuxrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    pub fn exintrst(&self) -> ExintrstR {
        ExintrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GpiodrstR {
        GpiodrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GpioerstR {
        GpioerstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1rst(&self) -> Adc1rstR {
        Adc1rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC2 reset"]
    #[inline(always)]
    pub fn adc2rst(&self) -> Adc2rstR {
        Adc2rstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1rst(&self) -> Tmr1rstR {
        Tmr1rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer8 reset"]
    #[inline(always)]
    pub fn tmr8rst(&self) -> Tmr8rstR {
        Tmr8rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC3 reset"]
    #[inline(always)]
    pub fn adc3rst(&self) -> Adc3rstR {
        Adc3rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9rst(&self) -> Tmr9rstR {
        Tmr9rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10rst(&self) -> Tmr10rstR {
        Tmr10rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer11 reset"]
    #[inline(always)]
    pub fn tmr11rst(&self) -> Tmr11rstR {
        Tmr11rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ACC reset"]
    #[inline(always)]
    pub fn accrst(&self) -> AccrstR {
        AccrstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2c3rstR {
        I2c3rstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - USART6 reset"]
    #[inline(always)]
    pub fn usart6rst(&self) -> Usart6rstR {
        Usart6rstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - UART7 reset"]
    #[inline(always)]
    pub fn uart7rst(&self) -> Uart7rstR {
        Uart7rstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - UART8 reset"]
    #[inline(always)]
    pub fn uart8rst(&self) -> Uart8rstR {
        Uart8rstR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MUX function I/O reset"]
    #[inline(always)]
    pub fn iomuxrst(&mut self) -> IomuxrstW<'_, Apb2rstSpec> {
        IomuxrstW::new(self, 0)
    }
    #[doc = "Bit 1 - External interrupt reset"]
    #[inline(always)]
    pub fn exintrst(&mut self) -> ExintrstW<'_, Apb2rstSpec> {
        ExintrstW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port A reset"]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GpioarstW<'_, Apb2rstSpec> {
        GpioarstW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port B reset"]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GpiobrstW<'_, Apb2rstSpec> {
        GpiobrstW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port C reset"]
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GpiocrstW<'_, Apb2rstSpec> {
        GpiocrstW::new(self, 4)
    }
    #[doc = "Bit 5 - IO port D reset"]
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GpiodrstW<'_, Apb2rstSpec> {
        GpiodrstW::new(self, 5)
    }
    #[doc = "Bit 6 - IO port E reset"]
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GpioerstW<'_, Apb2rstSpec> {
        GpioerstW::new(self, 6)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1rst(&mut self) -> Adc1rstW<'_, Apb2rstSpec> {
        Adc1rstW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC2 reset"]
    #[inline(always)]
    pub fn adc2rst(&mut self) -> Adc2rstW<'_, Apb2rstSpec> {
        Adc2rstW::new(self, 10)
    }
    #[doc = "Bit 11 - Timer1 reset"]
    #[inline(always)]
    pub fn tmr1rst(&mut self) -> Tmr1rstW<'_, Apb2rstSpec> {
        Tmr1rstW::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> Spi1rstW<'_, Apb2rstSpec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 13 - Timer8 reset"]
    #[inline(always)]
    pub fn tmr8rst(&mut self) -> Tmr8rstW<'_, Apb2rstSpec> {
        Tmr8rstW::new(self, 13)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> Usart1rstW<'_, Apb2rstSpec> {
        Usart1rstW::new(self, 14)
    }
    #[doc = "Bit 15 - ADC3 reset"]
    #[inline(always)]
    pub fn adc3rst(&mut self) -> Adc3rstW<'_, Apb2rstSpec> {
        Adc3rstW::new(self, 15)
    }
    #[doc = "Bit 19 - Timer9 reset"]
    #[inline(always)]
    pub fn tmr9rst(&mut self) -> Tmr9rstW<'_, Apb2rstSpec> {
        Tmr9rstW::new(self, 19)
    }
    #[doc = "Bit 20 - Timer10 reset"]
    #[inline(always)]
    pub fn tmr10rst(&mut self) -> Tmr10rstW<'_, Apb2rstSpec> {
        Tmr10rstW::new(self, 20)
    }
    #[doc = "Bit 21 - Timer11 reset"]
    #[inline(always)]
    pub fn tmr11rst(&mut self) -> Tmr11rstW<'_, Apb2rstSpec> {
        Tmr11rstW::new(self, 21)
    }
    #[doc = "Bit 22 - ACC reset"]
    #[inline(always)]
    pub fn accrst(&mut self) -> AccrstW<'_, Apb2rstSpec> {
        AccrstW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2c3rstW<'_, Apb2rstSpec> {
        I2c3rstW::new(self, 23)
    }
    #[doc = "Bit 24 - USART6 reset"]
    #[inline(always)]
    pub fn usart6rst(&mut self) -> Usart6rstW<'_, Apb2rstSpec> {
        Usart6rstW::new(self, 24)
    }
    #[doc = "Bit 25 - UART7 reset"]
    #[inline(always)]
    pub fn uart7rst(&mut self) -> Uart7rstW<'_, Apb2rstSpec> {
        Uart7rstW::new(self, 25)
    }
    #[doc = "Bit 26 - UART8 reset"]
    #[inline(always)]
    pub fn uart8rst(&mut self) -> Uart8rstW<'_, Apb2rstSpec> {
        Uart8rstW::new(self, 26)
    }
}
#[doc = "APB2 peripheral reset register (CRM_APB2RST)\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstSpec;
impl crate::RegisterSpec for Apb2rstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rst::R`](R) reader structure"]
impl crate::Readable for Apb2rstSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rst::W`](W) writer structure"]
impl crate::Writable for Apb2rstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2RST to value 0"]
impl crate::Resettable for Apb2rstSpec {}
