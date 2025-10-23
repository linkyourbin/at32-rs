#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SLEEP_DEBUG` reader - SLEEP_DEBUG"]
pub type SleepDebugR = crate::BitReader;
#[doc = "Field `SLEEP_DEBUG` writer - SLEEP_DEBUG"]
pub type SleepDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPSLEEP_DEBUG` reader - DEEPSLEEP_DEBUG"]
pub type DeepsleepDebugR = crate::BitReader;
#[doc = "Field `DEEPSLEEP_DEBUG` writer - DEEPSLEEP_DEBUG"]
pub type DeepsleepDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STANDBY_DEBUG` reader - STANDBY_DEBUG"]
pub type StandbyDebugR = crate::BitReader;
#[doc = "Field `STANDBY_DEBUG` writer - STANDBY_DEBUG"]
pub type StandbyDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - TRACE_IOEN"]
pub type TraceIoenR = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - TRACE_IOEN"]
pub type TraceIoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - TRACE_MODE"]
pub type TraceModeR = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - TRACE_MODE"]
pub type TraceModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDT_PAUSE` reader - WDT_PAUSE"]
pub type WdtPauseR = crate::BitReader;
#[doc = "Field `WDT_PAUSE` writer - WDT_PAUSE"]
pub type WdtPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDT_PAUSE` reader - WWDT_PAUSE"]
pub type WwdtPauseR = crate::BitReader;
#[doc = "Field `WWDT_PAUSE` writer - WWDT_PAUSE"]
pub type WwdtPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR1_PAUSE` reader - TMR1_PAUSE"]
pub type Tmr1PauseR = crate::BitReader;
#[doc = "Field `TMR1_PAUSE` writer - TMR1_PAUSE"]
pub type Tmr1PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2_PAUSE` reader - TMR2_PAUSE"]
pub type Tmr2PauseR = crate::BitReader;
#[doc = "Field `TMR2_PAUSE` writer - TMR2_PAUSE"]
pub type Tmr2PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR3_PAUSE` reader - TMR3_PAUSE"]
pub type Tmr3PauseR = crate::BitReader;
#[doc = "Field `TMR3_PAUSE` writer - TMR3_PAUSE"]
pub type Tmr3PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR4_PAUSE` reader - TMR4_PAUSE"]
pub type Tmr4PauseR = crate::BitReader;
#[doc = "Field `TMR4_PAUSE` writer - TMR4_PAUSE"]
pub type Tmr4PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1_PAUSE` reader - CAN1_PAUSE"]
pub type Can1PauseR = crate::BitReader;
#[doc = "Field `CAN1_PAUSE` writer - CAN1_PAUSE"]
pub type Can1PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` reader - I2C1_SMBUS_TIMEOUT"]
pub type I2c1SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C1_SMBUS_TIMEOUT` writer - I2C1_SMBUS_TIMEOUT"]
pub type I2c1SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` reader - I2C2_SMBUS_TIMEOUT"]
pub type I2c2SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C2_SMBUS_TIMEOUT` writer - I2C2_SMBUS_TIMEOUT"]
pub type I2c2SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR8_PAUSE` reader - TMR8_PAUSE"]
pub type Tmr8PauseR = crate::BitReader;
#[doc = "Field `TMR8_PAUSE` writer - TMR8_PAUSE"]
pub type Tmr8PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5_PAUSE` reader - TMR5_PAUSE"]
pub type Tmr5PauseR = crate::BitReader;
#[doc = "Field `TMR5_PAUSE` writer - TMR5_PAUSE"]
pub type Tmr5PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR6_PAUSE` reader - TMR6_PAUSE"]
pub type Tmr6PauseR = crate::BitReader;
#[doc = "Field `TMR6_PAUSE` writer - TMR6_PAUSE"]
pub type Tmr6PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR7_PAUSE` reader - TMR7_PAUSE"]
pub type Tmr7PauseR = crate::BitReader;
#[doc = "Field `TMR7_PAUSE` writer - TMR7_PAUSE"]
pub type Tmr7PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2_PAUSE` reader - CAN2_PAUSE"]
pub type Can2PauseR = crate::BitReader;
#[doc = "Field `CAN2_PAUSE` writer - CAN2_PAUSE"]
pub type Can2PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR12_PAUSE` reader - TMR12_PAUSE"]
pub type Tmr12PauseR = crate::BitReader;
#[doc = "Field `TMR12_PAUSE` writer - TMR12_PAUSE"]
pub type Tmr12PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR13_PAUSE` reader - TMR13_PAUSE"]
pub type Tmr13PauseR = crate::BitReader;
#[doc = "Field `TMR13_PAUSE` writer - TMR13_PAUSE"]
pub type Tmr13PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR14_PAUSE` reader - TMR14_PAUSE"]
pub type Tmr14PauseR = crate::BitReader;
#[doc = "Field `TMR14_PAUSE` writer - TMR14_PAUSE"]
pub type Tmr14PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR9_PAUSE` reader - TMR9_PAUSE"]
pub type Tmr9PauseR = crate::BitReader;
#[doc = "Field `TMR9_PAUSE` writer - TMR9_PAUSE"]
pub type Tmr9PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR10_PAUSE` reader - TMR10_PAUSE"]
pub type Tmr10PauseR = crate::BitReader;
#[doc = "Field `TMR10_PAUSE` writer - TMR10_PAUSE"]
pub type Tmr10PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR11_PAUSE` reader - TMR11_PAUSE"]
pub type Tmr11PauseR = crate::BitReader;
#[doc = "Field `TMR11_PAUSE` writer - TMR11_PAUSE"]
pub type Tmr11PauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_SMBUS_TIMEOUT` reader - I2C3_SMBUS_TIMEOUT"]
pub type I2c3SmbusTimeoutR = crate::BitReader;
#[doc = "Field `I2C3_SMBUS_TIMEOUT` writer - I2C3_SMBUS_TIMEOUT"]
pub type I2c3SmbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SLEEP_DEBUG"]
    #[inline(always)]
    pub fn sleep_debug(&self) -> SleepDebugR {
        SleepDebugR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DEEPSLEEP_DEBUG"]
    #[inline(always)]
    pub fn deepsleep_debug(&self) -> DeepsleepDebugR {
        DeepsleepDebugR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STANDBY_DEBUG"]
    #[inline(always)]
    pub fn standby_debug(&self) -> StandbyDebugR {
        StandbyDebugR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TraceIoenR {
        TraceIoenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TraceModeR {
        TraceModeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - WDT_PAUSE"]
    #[inline(always)]
    pub fn wdt_pause(&self) -> WdtPauseR {
        WdtPauseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - WWDT_PAUSE"]
    #[inline(always)]
    pub fn wwdt_pause(&self) -> WwdtPauseR {
        WwdtPauseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TMR1_PAUSE"]
    #[inline(always)]
    pub fn tmr1_pause(&self) -> Tmr1PauseR {
        Tmr1PauseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TMR2_PAUSE"]
    #[inline(always)]
    pub fn tmr2_pause(&self) -> Tmr2PauseR {
        Tmr2PauseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TMR3_PAUSE"]
    #[inline(always)]
    pub fn tmr3_pause(&self) -> Tmr3PauseR {
        Tmr3PauseR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TMR4_PAUSE"]
    #[inline(always)]
    pub fn tmr4_pause(&self) -> Tmr4PauseR {
        Tmr4PauseR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CAN1_PAUSE"]
    #[inline(always)]
    pub fn can1_pause(&self) -> Can1PauseR {
        Can1PauseR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&self) -> I2c1SmbusTimeoutR {
        I2c1SmbusTimeoutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&self) -> I2c2SmbusTimeoutR {
        I2c2SmbusTimeoutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TMR8_PAUSE"]
    #[inline(always)]
    pub fn tmr8_pause(&self) -> Tmr8PauseR {
        Tmr8PauseR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TMR5_PAUSE"]
    #[inline(always)]
    pub fn tmr5_pause(&self) -> Tmr5PauseR {
        Tmr5PauseR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TMR6_PAUSE"]
    #[inline(always)]
    pub fn tmr6_pause(&self) -> Tmr6PauseR {
        Tmr6PauseR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TMR7_PAUSE"]
    #[inline(always)]
    pub fn tmr7_pause(&self) -> Tmr7PauseR {
        Tmr7PauseR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CAN2_PAUSE"]
    #[inline(always)]
    pub fn can2_pause(&self) -> Can2PauseR {
        Can2PauseR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - TMR12_PAUSE"]
    #[inline(always)]
    pub fn tmr12_pause(&self) -> Tmr12PauseR {
        Tmr12PauseR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TMR13_PAUSE"]
    #[inline(always)]
    pub fn tmr13_pause(&self) -> Tmr13PauseR {
        Tmr13PauseR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TMR14_PAUSE"]
    #[inline(always)]
    pub fn tmr14_pause(&self) -> Tmr14PauseR {
        Tmr14PauseR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - TMR9_PAUSE"]
    #[inline(always)]
    pub fn tmr9_pause(&self) -> Tmr9PauseR {
        Tmr9PauseR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TMR10_PAUSE"]
    #[inline(always)]
    pub fn tmr10_pause(&self) -> Tmr10PauseR {
        Tmr10PauseR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TMR11_PAUSE"]
    #[inline(always)]
    pub fn tmr11_pause(&self) -> Tmr11PauseR {
        Tmr11PauseR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - I2C3_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c3_smbus_timeout(&self) -> I2c3SmbusTimeoutR {
        I2c3SmbusTimeoutR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SLEEP_DEBUG"]
    #[inline(always)]
    pub fn sleep_debug(&mut self) -> SleepDebugW<'_, CtrlSpec> {
        SleepDebugW::new(self, 0)
    }
    #[doc = "Bit 1 - DEEPSLEEP_DEBUG"]
    #[inline(always)]
    pub fn deepsleep_debug(&mut self) -> DeepsleepDebugW<'_, CtrlSpec> {
        DeepsleepDebugW::new(self, 1)
    }
    #[doc = "Bit 2 - STANDBY_DEBUG"]
    #[inline(always)]
    pub fn standby_debug(&mut self) -> StandbyDebugW<'_, CtrlSpec> {
        StandbyDebugW::new(self, 2)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TraceIoenW<'_, CtrlSpec> {
        TraceIoenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TraceModeW<'_, CtrlSpec> {
        TraceModeW::new(self, 6)
    }
    #[doc = "Bit 8 - WDT_PAUSE"]
    #[inline(always)]
    pub fn wdt_pause(&mut self) -> WdtPauseW<'_, CtrlSpec> {
        WdtPauseW::new(self, 8)
    }
    #[doc = "Bit 9 - WWDT_PAUSE"]
    #[inline(always)]
    pub fn wwdt_pause(&mut self) -> WwdtPauseW<'_, CtrlSpec> {
        WwdtPauseW::new(self, 9)
    }
    #[doc = "Bit 10 - TMR1_PAUSE"]
    #[inline(always)]
    pub fn tmr1_pause(&mut self) -> Tmr1PauseW<'_, CtrlSpec> {
        Tmr1PauseW::new(self, 10)
    }
    #[doc = "Bit 11 - TMR2_PAUSE"]
    #[inline(always)]
    pub fn tmr2_pause(&mut self) -> Tmr2PauseW<'_, CtrlSpec> {
        Tmr2PauseW::new(self, 11)
    }
    #[doc = "Bit 12 - TMR3_PAUSE"]
    #[inline(always)]
    pub fn tmr3_pause(&mut self) -> Tmr3PauseW<'_, CtrlSpec> {
        Tmr3PauseW::new(self, 12)
    }
    #[doc = "Bit 13 - TMR4_PAUSE"]
    #[inline(always)]
    pub fn tmr4_pause(&mut self) -> Tmr4PauseW<'_, CtrlSpec> {
        Tmr4PauseW::new(self, 13)
    }
    #[doc = "Bit 14 - CAN1_PAUSE"]
    #[inline(always)]
    pub fn can1_pause(&mut self) -> Can1PauseW<'_, CtrlSpec> {
        Can1PauseW::new(self, 14)
    }
    #[doc = "Bit 15 - I2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c1_smbus_timeout(&mut self) -> I2c1SmbusTimeoutW<'_, CtrlSpec> {
        I2c1SmbusTimeoutW::new(self, 15)
    }
    #[doc = "Bit 16 - I2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c2_smbus_timeout(&mut self) -> I2c2SmbusTimeoutW<'_, CtrlSpec> {
        I2c2SmbusTimeoutW::new(self, 16)
    }
    #[doc = "Bit 17 - TMR8_PAUSE"]
    #[inline(always)]
    pub fn tmr8_pause(&mut self) -> Tmr8PauseW<'_, CtrlSpec> {
        Tmr8PauseW::new(self, 17)
    }
    #[doc = "Bit 18 - TMR5_PAUSE"]
    #[inline(always)]
    pub fn tmr5_pause(&mut self) -> Tmr5PauseW<'_, CtrlSpec> {
        Tmr5PauseW::new(self, 18)
    }
    #[doc = "Bit 19 - TMR6_PAUSE"]
    #[inline(always)]
    pub fn tmr6_pause(&mut self) -> Tmr6PauseW<'_, CtrlSpec> {
        Tmr6PauseW::new(self, 19)
    }
    #[doc = "Bit 20 - TMR7_PAUSE"]
    #[inline(always)]
    pub fn tmr7_pause(&mut self) -> Tmr7PauseW<'_, CtrlSpec> {
        Tmr7PauseW::new(self, 20)
    }
    #[doc = "Bit 21 - CAN2_PAUSE"]
    #[inline(always)]
    pub fn can2_pause(&mut self) -> Can2PauseW<'_, CtrlSpec> {
        Can2PauseW::new(self, 21)
    }
    #[doc = "Bit 25 - TMR12_PAUSE"]
    #[inline(always)]
    pub fn tmr12_pause(&mut self) -> Tmr12PauseW<'_, CtrlSpec> {
        Tmr12PauseW::new(self, 25)
    }
    #[doc = "Bit 26 - TMR13_PAUSE"]
    #[inline(always)]
    pub fn tmr13_pause(&mut self) -> Tmr13PauseW<'_, CtrlSpec> {
        Tmr13PauseW::new(self, 26)
    }
    #[doc = "Bit 27 - TMR14_PAUSE"]
    #[inline(always)]
    pub fn tmr14_pause(&mut self) -> Tmr14PauseW<'_, CtrlSpec> {
        Tmr14PauseW::new(self, 27)
    }
    #[doc = "Bit 28 - TMR9_PAUSE"]
    #[inline(always)]
    pub fn tmr9_pause(&mut self) -> Tmr9PauseW<'_, CtrlSpec> {
        Tmr9PauseW::new(self, 28)
    }
    #[doc = "Bit 29 - TMR10_PAUSE"]
    #[inline(always)]
    pub fn tmr10_pause(&mut self) -> Tmr10PauseW<'_, CtrlSpec> {
        Tmr10PauseW::new(self, 29)
    }
    #[doc = "Bit 30 - TMR11_PAUSE"]
    #[inline(always)]
    pub fn tmr11_pause(&mut self) -> Tmr11PauseW<'_, CtrlSpec> {
        Tmr11PauseW::new(self, 30)
    }
    #[doc = "Bit 31 - I2C3_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn i2c3_smbus_timeout(&mut self) -> I2c3SmbusTimeoutW<'_, CtrlSpec> {
        I2c3SmbusTimeoutW::new(self, 31)
    }
}
#[doc = "DEBUG_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
