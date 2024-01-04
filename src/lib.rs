//! A platform agnostic rust driver for the ACS37800 current sensor.

#![no_std]
#![allow(dead_code)]
#![deny(missing_docs)]
#![deny(warnings)]

mod registers;

use embedded_hal::blocking::i2c;
use crate::registers::Registers::*;
use crate::registers::*;

pub enum CurrentSensingRange {
    I30Amps = 30,
    I90Amps = 90,
}

pub struct Acs37800<I2C, E>
    where
        I2C: i2c::Write<Error=E> + i2c::Read<Error=E>,
{
    i2c: I2C,
    pub addr: u8,
    r_iso: f32,
    r_sense: f32,
    current_sensing_range: CurrentSensingRange,
    reg0b: Reg0b,
    reg0c: Reg0c,
    reg0d: Reg0d,
    reg0e: Reg0e,
    reg0f: Reg0f,
}

impl<I2C, E> Acs37800<I2C, E>
    where
        I2C: i2c::Write<Error=E> + i2c::Read<Error=E>,
{
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Acs37800 {
            i2c,
            addr,
            r_iso: 1000000.0,
            r_sense: 16900.0,
            current_sensing_range: CurrentSensingRange::I30Amps,
            reg0b: Reg0b::new(),
            reg0c: Reg0c::new(),
            reg0d: Reg0d::new(),
            reg0e: Reg0e::new(),
            reg0f: Reg0f::new(),
        }
    }

    pub fn with_r_iso(mut self, r_iso: f32) -> Self {
        self.r_iso = r_iso;
        self
    }

    pub fn with_r_sense(mut self, r_sense: f32) -> Self {
        self.r_sense = r_sense;
        self
    }

    pub fn with_current_sensing_range(mut self, range: CurrentSensingRange) -> Self {
        self.current_sensing_range = range;
        self
    }

    pub fn init(&mut self) -> Option<u32> {
        // read serial
        self.get_access_code()
    }

    fn convert_voltage(&mut self, v: u32) -> f32 {
        let mut v = v as f32 / 27500.0;
        v *= 250.0;
        v /= 1000.0;
        v * (self.r_iso + self.r_iso + self.r_sense) / self.r_sense
    }

    fn convert_current(&mut self, current: u32) -> f32 {
        match self.current_sensing_range {
            CurrentSensingRange::I30Amps => { current as f32 / 27500.0 * 30.0 }
            CurrentSensingRange::I90Amps => { current as f32 / 27500.0 * 90.0 }
        }
    }

    fn convert_power(&mut self, power: u32) -> f32 {
        // Datasheet: 3.08 LSB/mW for the 30A version and 1.03 LSB/mW for the 90A version
        let power = match self.current_sensing_range {
            CurrentSensingRange::I30Amps => { power as f32 / 3.08 }
            CurrentSensingRange::I90Amps => { power as f32 / 1.03 }
        };
        power * (self.r_iso + self.r_iso + self.r_sense) / self.r_sense / 1000.0
    }

    pub fn get_voltage_rmw(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadRMS.addr(), &mut buffer)?;
        let values = RegVI::from_bytes(buffer);
        Ok(self.convert_voltage(values.voltage().into()))
    }

    pub fn get_current_rms(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadRMS.addr(), &mut buffer)?;
        let values = RegVI::from_bytes(buffer);
        Ok(self.convert_current(values.current().into()))
    }
    pub fn get_power_active(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadPower.addr(), &mut buffer)?;
        let values = RegPower::from_bytes(buffer);
        Ok(self.convert_power(values.power().into()))
    }

    pub fn get_power_imag(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadPower.addr(), &mut buffer)?;
        let values = RegPower::from_bytes(buffer);
        Ok(self.convert_power(values.pimag().into()))
    }

    pub fn get_power_info(&mut self) -> Result<RegApparentPower, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadApparentPower.addr(), &mut buffer)?;
        Ok(RegApparentPower::from_bytes(buffer))
    }

    pub fn get_sample_num(&mut self) -> Result<u32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadNumSamples.addr(), &mut buffer)?;
        let num_samples = RegNumSamples::from_bytes(buffer);
        Ok(num_samples.numptsout().into())
    }

    pub fn get_voltage_avg_1_min(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadRMS1Min.addr(), &mut buffer)?;
        let values = RegVI::from_bytes(buffer);
        Ok(self.convert_voltage(values.voltage().into()))
    }

    pub fn get_current_avg_1_min(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadRMS1Min.addr(), &mut buffer)?;
        let values = RegVI::from_bytes(buffer);
        Ok(self.convert_current(values.current().into()))
    }
    pub fn get_power_avg_1_min(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadPower1Min.addr(), &mut buffer)?;
        let values = RegPower::from_bytes(buffer);
        Ok(self.convert_power(values.power().into()))
    }

    pub fn get_voltage_avg_1_sec(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadRMS1Sec.addr(), &mut buffer)?;
        let values = RegVI::from_bytes(buffer);
        Ok(self.convert_voltage(values.voltage().into()))
    }

    pub fn get_current_avg_1_sec(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadRMS1Sec.addr(), &mut buffer)?;
        let values = RegVI::from_bytes(buffer);
        Ok(self.convert_current(values.current().into()))
    }
    pub fn get_power_avg_1_sec(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadPower1Sec.addr(), &mut buffer)?;
        let values = RegPower::from_bytes(buffer);
        Ok(self.convert_power(values.power().into()))
    }
    pub fn get_instant_voltage(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadRaw.addr(), &mut buffer)?;
        let values = RegVI::from_bytes(buffer);
        Ok(self.convert_voltage(values.voltage().into()))
    }

    pub fn get_instant_current(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadRaw.addr(), &mut buffer)?;
        let values = RegVI::from_bytes(buffer);
        Ok(self.convert_current(values.current().into()))
    }

    pub fn get_instant_power(&mut self) -> Result<f32, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadPowerInstant.addr(), &mut buffer)?;
        let power = RegPower::from_bytes(buffer);
        Ok(self.convert_power(power.power().into()))
    }

    pub fn get_status(&mut self) -> Result<RegStatus, E> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadStatus.addr(), &mut buffer)?;
        Ok(RegStatus::from_bytes(buffer))
    }

    pub fn reset_fault_latch(&mut self) -> Result<(), E> {
        let mut temp = RegStatus::new();
        temp.set_faultlatched(true);
        self.write_register(ReadStatus.addr(), &mut temp.into_bytes())?;
        Ok(())
    }

    pub fn get_access_code(&mut self) -> Option<u32> {
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_register(ReadAccessCode.addr(), &mut buffer).ok()?;
        let mut access_code = buffer[0] as u32;
        access_code |= (buffer[1] as u32) << 8;
        access_code |= (buffer[2] as u32) << 16;
        access_code |= (buffer[3] as u32) << 24;
        if access_code != 0 {
            Some(access_code)
        } else {
            None
        }
    }

    pub fn get_customer_access(&mut self) -> Result<bool, E> {
        let mut buffer: [u8; 1] = [0];
        self.read_register(ReadCustomerAccess.addr(), &mut buffer)?;
        Ok(buffer[0] == 1)
    }

    fn read_register<'a>(&'a mut self, reg: u8, buffer: &'a mut [u8]) -> Result<&mut [u8], E> {
        self.write(&[reg])?;
        self.read(buffer)?;
        Ok(buffer)
    }

    fn read<'a>(&'a mut self, buffer: &'a mut [u8]) -> Result<&mut [u8], E> {
        self.i2c.read(self.addr, buffer)?;
        Ok(buffer)
    }

    fn write_register<'a>(&'a mut self, reg: u8, data: &'a mut [u8]) -> Result<(), E> {
        self.write(&[reg])?;
        self.write(data)?;
        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<(), E> {
        self.i2c.write(self.addr, data)
    }
}

