use anyhow::Result;
use num::NumCast;
use serial::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::Read;
use std::marker::PhantomData;
use std::path;
use std::thread;
use std::time;

// Default settings of Arduino
// see: https://www.arduino.cc/en/Serial/Begin
pub const arduino_settings: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud115200,
    // baud_rate: serial::BaudOther(500_000),
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

// pub trait SerialConnection: serial::SerialDevice + io::Read + io::Write {}
pub trait SerialConnection: io::Read + io::Write {}
// pub trait SerialConnection: serial::SerialPort {}
// pub trait SerialConnection: serial::SerialPort + io::Read + io::Write {}
pub trait SerialConnectionInstruction: std::fmt::Debug + TryInto<i8> + TryFrom<i8> {}

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
    fn drain(&mut self) -> Result<usize, SerialControllerError>;

    fn read_i8(&mut self) -> Result<i8, SerialControllerError>;
    fn read_u8(&mut self) -> Result<u8, SerialControllerError>;
    fn read_i16(&mut self) -> Result<i16, SerialControllerError>;
    fn read_i32(&mut self) -> Result<i32, SerialControllerError>;

    fn write_i8(&mut self, value: i8) -> Result<usize, SerialControllerError>;
    fn write_u8(&mut self, value: u8) -> Result<usize, SerialControllerError>;
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
        port.set_timeout(time::Duration::from_secs(10))
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
        let instruction_code = self.read_i8()?;
        let instruction = instruction_code.try_into().ok();
        // println!("read instruction: {:?}", instruction);
        Ok(instruction)
    }

    fn drain(&mut self) -> Result<usize, SerialControllerError> {
        loop {
            self.read_u8()?;
        }
        // let mut buffer: Vec<u8> = Vec::new();
        // self.port.read_to_end(&mut buffer).map_err(|err| err.into())
    }

    fn write_instruction(&mut self, instruction: I) -> Result<usize, SerialControllerError> {
        let instruction_code = instruction
            .try_into()
            .map_err(|err| SerialControllerError::InvalidInstruction(err.to_string()))?;
        self.write_i8(instruction_code)
    }

    fn read_i8(&mut self) -> Result<i8, SerialControllerError> {
        // let mut read_buffer = [0u8; 1];
        // self.port.read_exact(&mut read_buffer)?;
        // Ok(read_buffer[0] as i8)
        let result = self.read_u8()?;
        Ok(result as i8)
    }

    fn read_u8(&mut self) -> Result<u8, SerialControllerError> {
        let mut read_buffer = [0u8; 1];
        self.port.read_exact(&mut read_buffer)?;
        Ok(read_buffer[0])
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

    fn write_u8(&mut self, value: u8) -> Result<usize, SerialControllerError> {
        let buffer = [value];
        let num_bytes = self.port.write(&buffer)?;
        Ok(num_bytes)
    }

    fn write_i8(&mut self, value: i8) -> Result<usize, SerialControllerError> {
        self.write_u8(value as u8)
        // let buffer = [value as u8];
        // let num_bytes = self.port.write(&buffer)?;
        // Ok(num_bytes)
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
    DATA = 3,
    ACK = 4,
    READY = 5,
    ERROR = 6,
    NOOP = 7,
}

impl TryFrom<i8> for LEDSerialControllerInstruction {
    type Error = SerialControllerError;
    fn try_from(instruction_code: i8) -> Result<LEDSerialControllerInstruction, Self::Error> {
        match instruction_code as i64 {
            x if x == LEDSerialControllerInstruction::CONNECT as i64 => {
                Ok(LEDSerialControllerInstruction::CONNECT)
            }
            x if x == LEDSerialControllerInstruction::ALREADY_CONNECTED as i64 => {
                Ok(LEDSerialControllerInstruction::ALREADY_CONNECTED)
            }
            x if x == LEDSerialControllerInstruction::INIT as i64 => {
                Ok(LEDSerialControllerInstruction::INIT)
            }
            x if x == LEDSerialControllerInstruction::ACK as i64 => {
                Ok(LEDSerialControllerInstruction::ACK)
            }
            x if x == LEDSerialControllerInstruction::DATA as i64 => {
                Ok(LEDSerialControllerInstruction::DATA)
            }
            x if x == LEDSerialControllerInstruction::READY as i64 => {
                Ok(LEDSerialControllerInstruction::READY)
            }
            x if x == LEDSerialControllerInstruction::ERROR as i64 => {
                Ok(LEDSerialControllerInstruction::ERROR)
            }
            x if x == LEDSerialControllerInstruction::NOOP as i64 => {
                Ok(LEDSerialControllerInstruction::NOOP)
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

#[derive(Debug)]
pub enum LEDSerialControllerError {
    InvalidParameter(String),
    MissingParameter(String),
    InvalidData(String),
    OutOfSync(String),
    NoAck(),
    SetupFailed(),
}

impl fmt::Display for LEDSerialControllerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidParameter(msg) => write!(f, "{}", msg),
            Self::MissingParameter(msg) => write!(f, "{}", msg),
            Self::InvalidData(msg) => write!(f, "{}", msg),
            Self::OutOfSync(msg) => write!(f, "out of sync: {}", msg),
            Self::NoAck() => write!(f, "did not receive acknowledgement"),
            Self::SetupFailed() => write!(f, "setup failed"),
        }
    }
}

impl error::Error for LEDSerialControllerError {}

// pub struct LEDSerialController<'a, R>
// pub struct LEDSerialController<R>
pub struct LEDSerialController
// where
// R: SerialConnection,
{
    // controller: SerialController<'a, R, LEDSerialControllerInstruction>,
    // controller: SerialController<R, LEDSerialControllerInstruction>,
    pub controller: SerialController<LEDSerialControllerInstruction>,
    pub min_light_count: i32,
    pub total_light_count: u64,
    pub lights: proto::grpc::Lights,
    pub config: serial::PortSettings,
}

// impl SerialConnection for fs::port {}

// impl<'a, R> LEDSerialController<'a, R>
impl LEDSerialController
// impl<R> LEDSerialController<R>
// where
//     R: SerialConnection,
{
    // pub fn new(port: &'a mut R, config) -> Self {
    pub fn new(lights: proto::grpc::Lights, config: serial::PortSettings) -> Result<Self> {
        let serial_port = path::PathBuf::from(lights.serial_port.clone());
        let controller = SerialController::open(serial_port, config)?;
        let total_light_count = lights.strips.iter().fold(0, |acc, strip| {
            let num_lights: u64 = NumCast::from(strip.num_lights).unwrap_or(0);
            acc + num_lights
        });
        let min_light_count = lights.strips.iter().fold(i32::MAX, |acc, strip| {
            let num_lights: i32 = NumCast::from(strip.num_lights).unwrap_or(0);
            // .and_then(|num_lights| NumCast::from(num_lights))
            // .ok_or(LEDSerialControllerError::MissingParameter(
            //     "missing num lights",
            // ))?;
            acc.min(num_lights)
        });

        // if !lights
        //     .strips
        //     .iter()
        //     .all(|strip| i8::MIN <= strip.pin as i8 && strip.pin as i8 <= i8::MAX)
        // {
        //     return Err(LEDSerialControllerError::InvalidParameter(format!(
        //         "at least one pin is out of i8 bounds",
        //         pin
        //     )).into());
        // };
        Ok(Self {
            controller,
            min_light_count,
            total_light_count,
            lights,
            config,
        })
    }

    fn read_instruction(
        &mut self,
    ) -> Result<Option<LEDSerialControllerInstruction>, SerialControllerError> {
        loop {
            match self.controller.read_instruction()? {
                // skip
                Some(LEDSerialControllerInstruction::ALREADY_CONNECTED) => {}
                Some(LEDSerialControllerInstruction::CONNECT) => {}
                instruction => {
                    return Ok(instruction);
                }
            }
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        // let mut connection_attempts = 0;
        loop {
            println!("waiting for device ...");
            self.controller
                .write_instruction(LEDSerialControllerInstruction::CONNECT)?;
            // connection_attempts += 1;
            // println!("wrote message");
            // println!("reading message");
            match self.controller.read_instruction()? {
                Some(LEDSerialControllerInstruction::ALREADY_CONNECTED) => break,
                _ => {} // Some(LEDSerialControllerInstruction::CONNECT) => break,
                        // None => {
                        //     // println!("received nothing :(");
                        //     panic!("received nothing :(");
                        // }
                        // Some(instruction) => {
                        //     println!("handshake: received instruction: {:?}", instruction);
                        // }
            }
            thread::sleep(time::Duration::from_secs(1));
        }
        // let ack = self.controller.read_instruction()?;
        println!("connected to device");
        // for _ in 0..connection_attempts - 1 {
        thread::sleep(time::Duration::from_secs(1));
        // println!("draining inputs");
        let _ = self.controller.drain();
        println!("drained input buffer");
        // loop {
        //     println!("{:?}", self.controller.read_instruction());
        // }
        Ok(())
    }

    pub fn configure(&mut self) -> Result<()> {
        self.controller
            .write_instruction(LEDSerialControllerInstruction::INIT)?;

        // send the number of led strips
        let num_strips: i32 = NumCast::from(self.lights.strips.len()).ok_or(
            LEDSerialControllerError::InvalidParameter(format!(
                "number of light strips ({}) does not fit i32",
                self.lights.strips.len()
            )),
        )?;
        self.controller.write_i32(num_strips)?;
        let check = self.controller.read_i32()?;
        if num_strips != check {
            return Err(LEDSerialControllerError::OutOfSync(format!(
                "received unexpected value from device setting num_strips ({} but expected {})",
                check, num_strips,
            ))
            .into());
        }

        // send the data pins and number of leds for each
        for (idx, strip) in self.lights.strips.iter().enumerate() {
            let pin: i8 =
                NumCast::from(strip.pin).ok_or(LEDSerialControllerError::InvalidParameter(
                    format!("data pin ({}) of strip {} does not fit i8", strip.pin, idx,),
                ))?;
            let num_lights = NumCast::from(strip.num_lights).ok_or(
                LEDSerialControllerError::InvalidParameter(format!(
                    "num leds ({}) of strip {} does not fit i32",
                    strip.num_lights, idx,
                )),
            )?;
            self.controller.write_i8(pin)?;
            let check = self.controller.read_i8()?;
            if pin != check {
                return Err(LEDSerialControllerError::OutOfSync(format!(
                    "received unexpected value from device setting pin for strip {} (got {} but expected {})",
                    idx, check, pin,
                ))
                .into());
            }
            // self.controller.write_i32(self.min_light_count)?;
            self.controller.write_i32(num_lights)?;
            if num_lights != self.controller.read_i32()? {
                return Err(LEDSerialControllerError::OutOfSync(format!(
                    "received unexpected value from device setting num_lights {} for strip {}",
                    num_lights, idx,
                ))
                .into());
            }
        }
        // loop {
        //     println!("waiting for ack from device ...");
        //     match self.controller.read_instruction()? {
        //         Some(LEDSerialControllerInstruction::ACK) => {
        //             break;
        //         }
        //         // Some(instruction) => {
        //         //     println!("received instruction: {:?}", instruction);
        //         // }
        //         _ => {
        //             // return Err(LEDSerialControllerError::SetupFailed().into());
        //         }
        //     };
        //     thread::sleep(time::Duration::from_secs(1));
        // }
        self.wait_for_ack();
        println!("device is configured...");
        Ok(())
    }

    pub fn update_color(&mut self, color: (u8, u8, u8)) -> Result<()> {
        // self.wait_for_ready();
        self.controller
            .write_instruction(LEDSerialControllerInstruction::DATA)?;
        self.controller.write_u8(color.0);
        self.controller.write_u8(color.1);
        self.controller.write_u8(color.2);
        self.wait_for_ack()
    }

    pub fn update_colors(&mut self, colors: Vec<u8>) -> Result<()> {
        // return Ok(());
        self.controller
            .write_instruction(LEDSerialControllerInstruction::DATA)?;

        println!("updating {} colors", colors.len());
        if colors.len() != NumCast::from(self.total_light_count * 3).unwrap() {
            return Err(LEDSerialControllerError::InvalidData(format!(
                "color buffer must have size total_light_count * 3 = {}, but got {}",
                self.total_light_count * 3,
                colors.len()
            ))
            .into());
        }

        self.controller.write_u8(colors[0]);
        self.controller.write_u8(colors[1]);
        self.controller.write_u8(colors[2]);
        // for color in colors {
        // self.controller.write_u8(color);
        // let check = self.controller.read_u8()?;
        // if color != check {
        //     return Err(LEDSerialControllerError::OutOfSync(format!(
        //         "received unexpected color from device (got {} but expected {})",
        //         check, color
        //     ))
        //     .into());
        // }
        // }
        println!("updated colors ...");
        self.wait_for_ack()
    }

    fn wait_for_instruction(&mut self, instruction: LEDSerialControllerInstruction) -> Result<()> {
        // println!("waiting for instruction {:?} from device ...", instruction);
        loop {
            match self.controller.read_instruction()? {
                Some(received) => {
                    if received == instruction {
                        return Ok(());
                    } else {
                        println!("need {:?}, but received: {:?}", instruction, received);
                    }
                }
                None => {}
            }
        }
    }

    fn wait_for_ack(&mut self) -> Result<()> {
        self.wait_for_instruction(LEDSerialControllerInstruction::ACK)
        // println!("waiting for ack from device ...");
        // loop {
        //     match self.controller.read_instruction()? {
        //         Some(LEDSerialControllerInstruction::ACK) => return Ok(()),
        //         // Some(LEDSerialControllerInstruction::ALREADY) => return Ok(()),
        //         Some(instruction) => {
        //             println!("need ack, but received: {:?}", instruction);
        //             // Err(LEDSerialControllerError::NoAck().into())
        //         }
        //         None => {
        //             // Err(LEDSerialControllerError::NoAck().into());
        //         }
        //     }
        // }
    }

    fn wait_for_ready(&mut self) -> Result<()> {
        self.wait_for_instruction(LEDSerialControllerInstruction::READY)
    }
}
