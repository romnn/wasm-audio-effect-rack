use anyhow::Result;
use serial::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::thread;
use std::io::Read;
use std::marker::PhantomData;
use std::path;
use std::time;

// Default settings of Arduino
// see: https://www.arduino.cc/en/Serial/Begin
pub const arduino_settings: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud115200,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

// pub trait SerialConnection: serial::SerialDevice + io::Read + io::Write {}
pub trait SerialConnection: io::Read + io::Write {}
// pub trait SerialConnection: serial::SerialPort {}
// pub trait SerialConnection: serial::SerialPort + io::Read + io::Write {}
pub trait SerialConnectionInstruction: TryInto<i8> + TryFrom<i8> {}

#[derive(Debug)]
pub enum SerialControllerError {
    InvalidInstruction(String),
    IOError(io::Error),
    SerialPortUnavailable(serial::Error),
    SerialPortConfigError(serial::Error),
    ConnectionTimeout(),
}

// impl Into<SerialControllerError> for io::Error {
//     fn into(self) -> SerialControllerError {
//         SerialControllerError::IOError(self)
//     }
// }

impl From<io::Error> for SerialControllerError {
    fn from(err: io::Error) -> SerialControllerError {
        SerialControllerError::IOError(err)
    }
}

// impl From<i8> for LEDSerialControllerInstruction {
//     fn from(instruction: i8) -> SerialControllerError  {
//         SerialControllerError::IOError(err)
//     }
// }

impl fmt::Display for SerialControllerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SerialPortUnavailable(err) => write!(f, "serial port is not available: {}", err),
            Self::SerialPortConfigError(err) => {
                write!(f, "serial port configuration failed: {}", err)
            }
            Self::InvalidInstruction(msg) => write!(f, "invalid instruction: {}", msg),
            Self::IOError(err) => write!(f, "IO error: {}", err),
            Self::ConnectionTimeout() => write!(f, "serial connection attempt timeout"),
        }
    }
}

impl error::Error for SerialControllerError {}

// pub struct SerialController<'a, R, I>
// pub struct SerialController<R, I>
pub struct SerialController<I>
where
    // R: SerialConnection,
    I: SerialConnectionInstruction,
{
    // port: &'a mut R,
    // port: Box<dyn SerialConnection>,
    port: Box<dyn SerialConnection>,
    // port: serial::SystemPort,
    phantom: PhantomData<I>,
    // phantomR: PhantomData<R>,
}

// pub trait SerialControllerInterface<'a, R, I>
// pub trait SerialControllerInterface<R, I>
pub trait SerialControllerInterface<I>
where
    // R: SerialConnection,
    I: SerialConnectionInstruction,
{
    // fn new(port: &'a mut R) -> Self;
    fn open(
        // port: Box<R>,
        port: path::PathBuf,
        config: serial::PortSettings,
    ) -> Result<Self, SerialControllerError>
    where
        Self: Sized;

    fn new(port: Box<dyn SerialConnection>) -> Self
    where
        Self: Sized;
    //
    // fn connect(&mut self) -> Result<(), SerialControllerError>;
    fn read_instruction(&mut self) -> Result<Option<I>, SerialControllerError>;
    fn write_instruction(&mut self, instruction: I) -> Result<usize, SerialControllerError>;

    fn read_i8(&mut self) -> Result<i8, SerialControllerError>;
    fn read_i16(&mut self) -> Result<i16, SerialControllerError>;
    fn read_i32(&mut self) -> Result<i32, SerialControllerError>;

    fn write_i8(&mut self, value: i8) -> Result<usize, SerialControllerError>;
    fn write_i16(&mut self, value: i16) -> Result<usize, SerialControllerError>;
    fn write_i32(&mut self, value: i32) -> Result<usize, SerialControllerError>;
}

impl SerialConnection for serial::SystemPort {}
// impl<'a, R, I> SerialControllerInterface<'a, R, I> for SerialController<'a, R, I>
// impl<R, I> SerialControllerInterface<R, I> for SerialController<R, I>
impl<I> SerialControllerInterface<I> for SerialController<I>
where
    // R: SerialConnection,
    I: SerialConnectionInstruction,
    <I as TryInto<i8>>::Error: std::fmt::Display,
{
    // fn new(port: &'a mut R) -> Self {
    fn open(
        port: path::PathBuf,
        config: serial::PortSettings,
    ) -> Result<Self, SerialControllerError> {
        println!("opening serial port: {:?}", port);
        let mut port =
            serial::open(&port).map_err(|err| SerialControllerError::SerialPortUnavailable(err))?;
        port.configure(&config)
            .map_err(|err| SerialControllerError::SerialPortConfigError(err))?;
        port.set_timeout(time::Duration::from_secs(30))
            .map_err(|err| SerialControllerError::ConnectionTimeout())?;

        // let port: &(dyn io::Read + io::Write) = &port;
        // let port: &(dyn SerialConnection) = &port;
        let port: Box<dyn SerialConnection> = Box::new(port);
        // let port: Box<dyn SerialConnection> = Box::new(port);
        Ok(Self {
            port,
            // port: Box::new(port),
            phantom: PhantomData,
        })
    }

    fn new(port: Box<dyn SerialConnection>) -> Self {
        return Self {
            port,
            phantom: PhantomData,
        };
    }

    // fn connect(&mut self) -> Result<(), SerialControllerError> {
    //     self.port
    //         .set_timeout(time::Duration::from_secs(30))
    //         .map_err(|err| SerialControllerError::ConnectionTimeout())?;
    //     Ok(())
    // }

    fn read_instruction(&mut self) -> Result<Option<I>, SerialControllerError> {
        let instruction = self.read_i8()?;
        Ok(instruction.try_into().ok())
    }

    fn write_instruction(&mut self, instruction: I) -> Result<usize, SerialControllerError> {
        let instruction_code = instruction
            .try_into()
            .map_err(|err| SerialControllerError::InvalidInstruction(err.to_string()))?;
        self.write_i8(instruction_code)
    }

    fn read_i8(&mut self) -> Result<i8, SerialControllerError> {
        let mut read_buffer = [0u8; 1];
        self.port.read_exact(&mut read_buffer)?;
        Ok(read_buffer[0] as i8)
    }

    fn read_i16(&mut self) -> Result<i16, SerialControllerError> {
        let mut read_buffer = [0u8; 2];
        self.port.read_exact(&mut read_buffer)?;
        let number: u16 =
            ((read_buffer[0] as u16) & 0xff) | ((read_buffer[1] as u16) << 8 & 0xff00);
        Ok(number as i16)
    }

    fn read_i32(&mut self) -> Result<i32, SerialControllerError> {
        let mut read_buffer = [0u8; 4];
        self.port.read_exact(&mut read_buffer)?;
        let number: u32 = ((read_buffer[0] as u32) & 0xff)
            | ((read_buffer[1] as u32) << 8 & 0xff00)
            | ((read_buffer[2] as u32) << 16 & 0xff0000)
            | ((read_buffer[3] as u32) << 24 & 0xff000000);
        Ok(number as i32)
    }

    fn write_i8(&mut self, value: i8) -> Result<usize, SerialControllerError> {
        let buffer = [value as u8];
        let num_bytes = self.port.write(&buffer)?;
        Ok(num_bytes)
    }

    fn write_i16(&mut self, value: i16) -> Result<usize, SerialControllerError> {
        let buffer = [(value & 0xff) as u8, (value >> 8 & 0xff) as u8];
        let num_bytes = self.port.write(&buffer)?;
        Ok(num_bytes)
    }

    fn write_i32(&mut self, value: i32) -> Result<usize, SerialControllerError> {
        let buffer = [
            (value & 0xff) as u8,
            (value >> 8 & 0xff) as u8,
            (value >> 16 & 0xff) as u8,
            (value >> 24 & 0xff) as u8,
        ];
        let num_bytes = self.port.write(&buffer)?;
        Ok(num_bytes)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum LEDSerialControllerInstruction {
    CONNECT = 0,
    ALREADY_CONNECTED = 1,
    INIT = 2,
    ACK = 3,
    DATA = 4,
}

impl TryFrom<i8> for LEDSerialControllerInstruction {
    type Error = SerialControllerError;
    fn try_from(instruction_code: i8) -> Result<LEDSerialControllerInstruction, Self::Error> {
        match instruction_code as i64 {
            x if x == LEDSerialControllerInstruction::INIT as i64 => {
                Ok(LEDSerialControllerInstruction::INIT)
            }
            x if x == LEDSerialControllerInstruction::ACK as i64 => {
                Ok(LEDSerialControllerInstruction::ACK)
            }
            x if x == LEDSerialControllerInstruction::DATA as i64 => {
                Ok(LEDSerialControllerInstruction::DATA)
            }
            _ => Err(SerialControllerError::InvalidInstruction(format!(
                "unknown instruction code: {}",
                instruction_code
            ))),
        }
    }
}

impl TryFrom<LEDSerialControllerInstruction> for i8 {
    type Error = SerialControllerError;
    fn try_from(instruction: LEDSerialControllerInstruction) -> Result<i8, Self::Error> {
        let instruction_code = instruction as i8;
        if i8::MIN <= instruction_code && instruction_code <= i8::MAX {
            Ok(instruction_code as i8)
        } else {
            Err(SerialControllerError::InvalidInstruction(
                "the instruction code is out of the i8 range".to_string(),
            ))
        }
    }
}

impl SerialConnectionInstruction for LEDSerialControllerInstruction {}

// pub struct LEDSerialController<'a, R>
// pub struct LEDSerialController<R>
pub struct LEDSerialController
// where
// R: SerialConnection,
{
    // controller: SerialController<'a, R, LEDSerialControllerInstruction>,
    // controller: SerialController<R, LEDSerialControllerInstruction>,
    controller: SerialController<LEDSerialControllerInstruction>,
    lights: proto::grpc::Lights,
    config: serial::PortSettings,
}

// impl SerialConnection for fs::port {}

// impl<'a, R> LEDSerialController<'a, R>
impl LEDSerialController
// impl<R> LEDSerialController<R>
// where
//     R: SerialConnection,
{
    // pub fn new(port: &'a mut R, config) -> Self {
    pub fn new(
        lights: proto::grpc::Lights,
        config: serial::PortSettings,
    ) -> Result<Self, SerialControllerError> {
        let serial_port = path::PathBuf::from(lights.serial_port.clone());
        let controller = SerialController::open(serial_port, config)?;
        Ok(Self {
            controller,
            lights,
            config,
        })
    }

    pub fn connect(&mut self) -> Result<()> {
        loop {
            println!("waiting for device ...");
            self.controller
                .write_instruction(LEDSerialControllerInstruction::CONNECT)?;
            // let order = LEDSerialControllerInstruction::ANNOUNCE as i8;
            // write_i8(&mut port, order).unwrap();
            // let received_order = Order::from_i8(read_i8(&mut port).unwrap()).unwrap();
            match self.controller.read_instruction()? {
                Some(LEDSerialControllerInstruction::ALREADY_CONNECTED) => break,
                None => {}
                _ => {}
            }
            thread::sleep(time::Duration::from_secs(1));
        }
        println!("connected to device");
        Ok(())
    }
}
