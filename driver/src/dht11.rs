use core::{
    fmt,
    fmt::{Debug, Display},
};

use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin, PinState},
};

pub struct Dht11<HE, P, D>
    where
        P: InputPin<Error = HE> + OutputPin<Error = HE>,
        D: DelayNs,
{
    pin:   P,
    delay: D,
}

impl<HE, P, D> Dht11<HE, P, D>
    where
        P: InputPin<Error = HE> + OutputPin<Error = HE>,
        D: DelayNs,
{
    pub fn new(pin: P, delay: D) -> Self {
        Self { pin, delay }
    }

    pub fn read(&mut self) -> Result<Reading, DhtError<HE>> {
        let mut buf = [0_u8; 5];
        // 重启 dht11
        self.pin.set_low()?;
        self.delay.delay_us(3000);
        self.pin.set_high()?;
        self.delay.delay_us(25);

        // 等 dht11 信号
        self.wait_signal(85,PinState::High,DhtError::NotPresent)?;
        self.wait_signal(85,PinState::Low,DhtError::NotPresent)?;

        // 开始接收数据
        for bit in 0..40 {
            self.wait_signal(55,PinState::High,DhtError::Timeout)?;
            let elapsed = self.wait_signal(55,PinState::Low,DhtError::Timeout)?;
            if elapsed > 30 {
                let byte = bit / 8;
                let shift = 7 - bit % 8;
                buf[byte] |= 1 << shift;
            }
        }
        // 校验数据
        let checksum = (buf[0..4].iter().fold(0u16, |acc, next| acc + *next as u16) & 0xff) as u8;
        if buf[4] != checksum {
            return Err(DhtError::ChecksumMismatch(buf[4], checksum));
        }

        // 湿度范围在 0%-100%
        let humidity = buf[0] as f32 + buf[1] as f32 / 10.0;
        if !(0.0..100.0).contains(&humidity) {
            return Err(DhtError::InvalidData);
        }

        // 符号位有1为零下温度
        let mut temperature = buf[2] as f32 + buf[3] as f32 / 10.0;
        if buf[2] & 0x80 != 0 {
            temperature = -temperature;
        }

        Ok(Reading { humidity, temperature })
    }

    pub fn wait_signal(&mut self, timeout_us: u32, state: PinState, error: DhtError<HE>) -> Result<u32, DhtError<HE>> {
        for i in 0..timeout_us {
            let state = match state {
                PinState::Low => self.pin.is_low()?,
                PinState::High => self.pin.is_high()?,
            };
            if state {
                return Ok(i);
            }
            self.delay.delay_us(1);
        }
        Err(error)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Reading {
    pub humidity:    f32,
    pub temperature: f32,
}

impl Display for Reading {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C  {:.1}%RH", self.temperature, self.humidity)
    }
}

#[derive(Debug, Clone)]
pub enum DhtError<HE> {
    ChecksumMismatch(u8, u8),
    PinError(HE),
    InvalidData,
    NotPresent,
    Timeout,
}

impl<HE> From<HE> for DhtError<HE> {
    fn from(error: HE) -> Self {
        DhtError::PinError(error)
    }
}

impl<HE: Debug> Display for DhtError<HE> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DhtError::*;
        match self {
            ChecksumMismatch(expected, calculated) => write!(
                f,
                "Data read was corrupt (expected checksum {:x}, calculated {:x})",
                expected, calculated
            ),
            NotPresent => write!(f, "DHT device not found"),
            InvalidData => f.write_str("Received data is out of range"),
            Timeout => f.write_str("Timed out waiting for a read"),
            PinError(err) => write!(f, "HAL pin error: {:?}", err),
        }
    }
}
