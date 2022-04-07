#![cfg_attr(not(test), no_std)]

//! Platform-agnostic driver for LIS2MDL magnetometer.
//!
//! Datasheet: <https://www.st.com/content/ccc/resource/technical/document/datasheet/group3/29/13/d1/e0/9a/4d/4f/30/DM00395193/files/DM00395193.pdf/jcr:content/translations/en.DM00395193.pdf>

use crate::error::Error;
use bits::{ConfigurationRegisterABits, ConfigurationRegisterBBits, ConfigurationRegisterCBits};
pub use config::output_data_rate::OutputDataRate;
use embedded_hal::blocking::i2c;

mod bits;
mod config;
mod error;
mod register;

pub const DEFAULT_I2C_ADDRESS: u8 = 0x1e;

pub struct Lis2mdl<I2c> {
    i2c: I2c,
    address: u8,
}

impl<I2c, E> Lis2mdl<I2c>
where
    I2c: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
    E: core::fmt::Debug,
{
    pub fn new(mut i2c: I2c, address: u8, rate: OutputDataRate) -> Result<Self, Error> {
        let config =
            ConfigurationRegisterABits::COMP_TEMP_EN | ConfigurationRegisterABits::from(rate);
        i2c.write(address, &[register::CFG_REG_A, config.bits()])
            .map_err(|_| Error::I2cError)?;

        i2c.write(
            address,
            &[register::CFG_REG_B, ConfigurationRegisterBBits::LPF.bits()],
        )
        .map_err(|_| Error::I2cError)?;

        let config = ConfigurationRegisterCBits::DRDYONPIN
            | ConfigurationRegisterCBits::INTONPIN
            | ConfigurationRegisterCBits::BDU;
        i2c.write(address, &[register::CFG_REG_C, config.bits()])
            .map_err(|_| Error::I2cError)?;

        Ok(Self { i2c, address })
    }

    pub fn release(self) -> I2c {
        self.i2c
    }

    pub fn get_chip_id(&mut self) -> Result<u8, Error> {
        let mut id = [0u8; 1];
        self.i2c
            .write_read(self.address, &[register::WHO_AM_I], &mut id)
            .map_err(|_| Error::I2cError)?;
        Ok(id[0])
    }

    pub fn read_temperature(&mut self) -> Result<i16, Error> {
        let mut temp = [0u8; 2];
        self.i2c
            .write_read(self.address, &[register::TEMP_OUT_L_REG], &mut temp)
            .map_err(|_| Error::I2cError)?;
        Ok(i16::from_le_bytes(temp))
    }

    pub fn read_magnetometer_data(&mut self) -> Result<(i16, i16, i16), Error> {
        let mut mag = [0u8; 6];
        self.i2c
            .write_read(self.address, &[register::OUTX_L_REG], &mut mag)
            .map_err(|_| Error::I2cError)?;

        let x = i16::from_le(mag[0] as i16 | (mag[1] as i16) << 8);
        let y = i16::from_le(mag[2] as i16 | (mag[3] as i16) << 8);
        let z = i16::from_le(mag[4] as i16 | (mag[5] as i16) << 8);

        Ok((x, y, z))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use embedded_hal_mock::i2c::Mock as I2cMock;

    /*
    #[test]
    fn aquire_release() {
        let mock = I2cMock::new(&[]);
        let lis = Lis2mdl::new(mock, 4);
        let mut released = lis.release();
        released.done();
    }
    */
}
