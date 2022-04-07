use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("I2c communication failed"))]
    I2cError,
}
