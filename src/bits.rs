use bitflags::bitflags;

// Register bit fields as defined in Datasheet Chapter 8. Doc comments taken from register descriptions with minor edits.
bitflags! {
    #[allow(non_camel_case_types, dead_code)]
    pub(crate) struct ConfigurationRegisterABits: u8 {
        /// "Enables the magnetometer temperature compensation. Default value: 0
        /// (0: temperature compensation disabled; 1: temperature compensation enabled)")
        const COMP_TEMP_EN = 0b1000_0000;

        /// Reboot magnetometer memory content. Default value: 0
        /// (0: normal mode; 1: reboot memory content)
        const REBOOT       = 0b0100_0000;

        /// When this bit is set, the configuration registers and user registers are reset.
        /// Flash registers keep their values. Default value: 0
        const SOFT_RST     = 0b0010_0000;

        ///  Enables low-power mode. Default value: 0
        /// 0: high-resolution mode 1: low-power mode enabled
        const LP           = 0b0001_0000;

        /// Output data rate configuration (see Table 25: Output data rate configuration)
        /// Default value: 00
        const ODR1         = 0b0000_1000;

        /// Output data rate configuration (see Table 25: Output data rate configuration)
        /// Default value: 00
        const ODR0         = 0b0000_0100;

        /// These bits select the mode of operation of the device
        /// (see Datasheet Table 26: Mode of operation). Default value: 1
        const MD1          = 0b0000_0010;

        /// These bits select the mode of operation of the device
        /// (see Datasheet Table 26: Mode of operation). Default value: 1
        const MD0          = 0b0000_0001;
    }

    #[allow(non_camel_case_types, dead_code)]
    #[derive(Default)]
    pub(crate) struct ConfigurationRegisterBBits: u8 {
        /// Enables offset cancellation in single measurement mode. The OFF_CANC bit
        /// must be set to 1 when enabling offset cancellation in single measurement mode.
        /// Default value: 0
        /// (0: offset cancellation in single measurement mode disabled;
        /// 1: offset cancellation in single measurement mode enabled)
        const OFF_CANC_ONE_SHOT = 0b0001_0000;

        /// If ‘1’, the interrupt block recognition checks data after the hard-iron correction to
        /// discover the interrupt. Default value: 0
        const INT_ON_DATAOFF    = 0b0000_1000;

        /// Selects the frequency of the set pulse. Default value: 0
        /// (0: set pulse is released every 63 ODR;
        /// 1: set pulse is released only at power-on after PD condition)
        const SET_FREQ          = 0b0000_0100;

        /// Enables offset cancellation. Default value: 0
        /// (0: offset cancellation disabled; 1: offset cancellation enabled)
        const OFF_CANC          = 0b0000_0010;

        /// Enables low-pass filter (see Table 29). Default value: 0
        /// (0: digital filter disabled; 1: digital filter enabled)
        const LPF               = 0b0000_0001;
    }

    #[allow(non_camel_case_types, dead_code)]
    #[derive(Default)]
    pub(crate) struct ConfigurationRegisterCBits: u8 {
        ///  If '1', the INTERRUPT signal (INT bit in INT_SOURCE_REG (64h)) is driven
        /// on the INT/DRDY pin. The INT/DRDY pin is configured in push-pull output mode.
        /// Default value: 0
        const INTONPIN  = 0b0100_0000;

        /// If ‘1’, the I2C interface is inhibited. Only the SPI interface can be used.
        const I2CDIS    = 0b0010_0000;

        /// If enabled, reading of incorrect data is avoided when the user reads asynchronously. In fact if the read request arrives during an update of the output data, a
        /// latch is possible, reading incoherent high and low parts of the same register.
        /// Only one part is updated and the other one remains old.
        const BDU       = 0b0001_0000;

        /// If ‘1’, an inversion of the low and high parts of the data occurs.
        const BLE       = 0b0000_1000;

        /// Set to '1' to enable SDO line on pin 7.
        const W4SPI     = 0b0000_0100;

        /// If ‘1’, the self-test is enabled.
        const SELFTEST  = 0b0000_0010;

        /// If '1', the data-ready signal (Zyxda bit in STATUS_REG (67h)) is driven on the
        /// INT/DRDY pin. The INT/DRDY pin is configured in push-pull output mode.
        /// Default value: 0
        const DRDYONPIN = 0b0000_0001;
    }

    #[allow(non_camel_case_types, dead_code)]
    pub(crate) struct InterruptControlRegBits: u8 {
        /// Enables the interrupt detection for the X-axis. Default value: 1
        /// (1: enabled; 0: disabled)
        const XIEN = 0b1000_0000;

        /// Enables the interrupt detection for the Y-axis. Default value: 1
        /// (1: enabled; 0: disabled)
        const YIEN = 0b0100_0000;

        /// Enables the interrupt detection for the Z-axis. Default value: 1
        /// (1: enabled; 0: disabled)
        const ZIEN = 0b0010_0000;

        /// Controls the polarity of the INT bit (INT_SOURCE_REG (64h)) when an interrupt occurs.
        /// Default: 0
        /// If IEA = 0, then INT = 0 signals an interrupt
        /// If IEA = 1, then INT = 1 signals an interrupt
        const IEA  = 0b0000_0100;

        /// Controls whether the INT bit (INT_SOURCE_REG (64h)) is latched or pulsed. Default: 0
        /// If IEL = 0, then INT is pulsed.
        /// If IEL = 1, then INT is latched.
        /// Once latched, INT remains in the same state until INT_SOURCE_REG (64h) is read
        const IEL  = 0b0000_0010;

        /// Interrupt enable. When set, enables the interrupt generation. The INT bit
        /// is in INT_SOURCE_REG (64h). Default: 0
        const IEN  = 0b0000_0001;
    }

    #[allow(non_camel_case_types, dead_code)]
    pub(crate) struct IntSourceRegBits: u8 {
        /// X-axis value exceeds the threshold positive side
        const PTHSX = 0b1000_0000;

        /// Y-axis value exceeds the threshold positive side
        const PTHSY = 0b0100_0000;

        /// Z-axis value exceeds the threshold positive side
        const PTHSZ = 0b0010_0000;

        /// X-axis value exceeds the threshold negative side
        const NTHSX = 0b0001_0000;

        /// Y-axis value exceeds the threshold negative side
        const NTHSY = 0b0000_1000;

        /// Z-axis value exceeds the threshold negative side
        const NTHSZ = 0b0000_0100;

        /// MROI flag generation is always enabled. This flag is reset by reading INT_SOURCE_REG (64h).
        const MROI  = 0b0000_0010;

        /// This bit signals when the interrupt event occurs.
        const INT   = 0b0000_0001;
    }

    #[allow(non_camel_case_types, dead_code)]
    #[derive(Default)]
    pub(crate) struct StatusRegBits: u8 {
        /// X-, Y- and Z-axis data overrun. Default value: 0
        /// (0: no overrun has occurred; 1: a new set of data has overwritten the previous set).
        const ZYXOR = 0b1000_0000;

        /// Z-axis data overrun. Default value: 0
        /// (0: no overrun has occurred; 1: new data for the Z-axis has overwritten the previous data).
        const ZOR   = 0b0100_0000;

        /// Y-axis data overrun. Default value: 0
        /// (0: no overrun has occurred; 1: new data for the Y-axis has overwritten the previous data).
        const YOR   = 0b0010_0000;

        /// X-axis data overrun. Default value: 0
        /// (0: no overrun has occurred; 1: new data for the X-axis has overwritten the previous data)
        const XOR   = 0b0001_0000;

        /// X-, Y- and Z-axis new data available. Default value: 0
        /// (0: a new set of data is not yet available; 1: a new set of data is available).
        const ZYXDA = 0b0000_1000;

        /// Z-axis new data available. Default value: 0
        /// (0: a new data for the Z-axis is not yet available; 1: a new data for the Z-axis is available)
        const ZDA   = 0b0000_0100;

        /// Y-axis new data available. Default value: 0
        /// (0: a new data for the Y-axis is not yet available; 1: a new data for the Y-axis is available)
        const YDA   = 0b0000_0010;

        /// X-axis new data available. Default value: 0
        /// (0: a new data for the X-axis is not yet available; 1: a new data for the X-axis is available)
        const XDA   = 0b0000_0001;
    }
}

impl Default for ConfigurationRegisterABits {
    fn default() -> Self {
        Self { bits: 0b0000_0011 }
    }
}

impl Default for InterruptControlRegBits {
    fn default() -> Self {
        Self { bits: 0b11100000 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn configuration_register_a() {
        let register_a = ConfigurationRegisterABits::default();
        assert_eq!(register_a.bits, 0b0000_0011);

        let register_a = ConfigurationRegisterABits::LP | ConfigurationRegisterABits::COMP_TEMP_EN;
        assert_eq!(register_a.bits, 0b1001_0000);

        let md = ConfigurationRegisterABits::MD0 | ConfigurationRegisterABits::MD1;
        assert_eq!(md.bits, 0b0000_0011);

        let reg_a = ConfigurationRegisterABits::from_bits(0b0000_0010).unwrap();
        assert!(matches!(reg_a, ConfigurationRegisterABits::MD1));
    }
}
