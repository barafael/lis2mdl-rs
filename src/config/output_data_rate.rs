use crate::bits::ConfigurationRegisterABits as Bits;

pub enum OutputDataRate {
    Hz10,
    Hz20,
    Hz50,
    Hz100,
}

impl From<OutputDataRate> for Bits {
    fn from(rate: OutputDataRate) -> Self {
        match rate {
            OutputDataRate::Hz10 => Bits::empty(),
            OutputDataRate::Hz20 => Bits::ODR0,
            OutputDataRate::Hz50 => Bits::ODR1,
            OutputDataRate::Hz100 => Bits::ODR0 | Bits::ODR1,
        }
    }
}
