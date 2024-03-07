//! Registers of the ACS37800
extern crate modular_bitfield_to_value;

use modular_bitfield::bitfield;
use modular_bitfield::prelude::*;
use modular_bitfield_to_value::ToValue;

/// Implementation to convert register enum to u8 address
pub trait Address {
    /// convert register enum to u8 address
    fn addr(self) -> u8;
}


/// Register addresses of the ACS37800
#[derive(Debug, Copy, Clone)]
#[allow(dead_code, non_camel_case_types)]
pub enum Registers {
    EEPROM_0B = 0x0B,
    EEPROM_0C = 0x0C,
    EEPROM_0D = 0x0D,
    EEPROM_0E = 0x0E,
    EEPROM_0F = 0x0F,
    SHADOW_0B = 0x1B,
    SHADOW_0C = 0x1C,
    SHADOW_0D = 0x1D,
    SHADOW_0E = 0x1E,
    SHADOW_0F = 0x1F,
    ReadRMS = 0x20,
    ReadPower = 0x21,
    ReadApparentPower = 0x22,
    ReadNumSamples = 0x25,
    ReadRMS1Sec = 0x26,
    ReadRMS1Min = 0x27,
    ReadPower1Sec = 0x28,
    ReadPower1Min = 0x29,
    ReadRaw = 0x2A,
    ReadPowerInstant = 0x2C,
    ReadStatus = 0x2D,
    AccessCode = 0x2F,
    ReadCustomerAccess = 0x30,
}

impl Address for Registers {
    fn addr(self) -> u8 {
        self as u8
    }
}

/// Register 0x0B
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct Reg0b {
    pub qvo_fine: B9,
    pub sns_fine: B10,
    pub crs_sns: B3,
    pub iavgselen: bool,
    pub pavgselen: bool,
    #[skip] reserved: B2,
    pub ecc: B6,
}


/// Register 0x0C
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct Reg0c {
    pub rms_avg_1: B7,
    pub rms_avg_2: B10,
    pub vchan_offset_code: B8,
    #[skip] reserved: B1,
    pub ecc: B6,
}

/// Register 0x0D
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct Reg0d {
    #[skip] reserved1: B7,
    pub ichan_del_en: B1,
    #[skip] reserved2: B1,
    pub chan_del_sel: B3,
    #[skip] reserved3: B1,
    pub fault: B8,
    pub fltdly: B3,
    #[skip] reserved4: B2,
    pub ecc: B6,
}

/// Register 0x0E
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct Reg0e {
    pub vevent_cycs: B6,
    #[skip] reserved1: B2,
    pub overvreg: B6,
    pub undervreg: B6,
    pub delaycnt_sel: B1,
    pub halfcycle_en: B1,
    pub squarewave_en: B1,
    pub zerocrosschansel: B1,
    pub zerocrossedgesel: B1,
    #[skip] reserved2: B1,
    pub ecc: B6,
}

/// Register 0x0F
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct Reg0f {
    #[skip] reserved1: B2,
    pub i2c_slv_addr: B7,
    pub i2c_dis_slv_addr: B1,
    pub dio_0_sel: B2,
    pub dio_1_sel: B2,
    pub n: B10,
    pub bypass_n_en: B1,
    #[skip] reserved2: B1,
    pub ecc: B6,
}

/// Register Voltage Current
/// (also used for averaged and instant reads)
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct RegVI {
    pub voltage: B16,
    pub current: B16,
}

/// Register Power
/// (also used for averaged and instant reads)
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct RegPower {
    pub power: B16,
    pub pimag: B16,
}

/// Register Apparent Power
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct RegApparentPower {
    pub papparent: B16,
    pub pfactor: B11,
    pub poasngle: B1,
    pub pospf: B1,
    #[skip] reserved: B3,
}


/// Register Number of Samples
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct RegNumSamples {
    pub numptsout: B10,
    #[skip] reserved: B22,
}


/// Register Status
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct RegStatus {
    pub zerocrossout: bool,
    pub faultout: bool,
    pub faultlatched: bool,
    pub overvoltage: bool,
    pub undervoltage: bool,
    #[skip] reserved: B27,
}
