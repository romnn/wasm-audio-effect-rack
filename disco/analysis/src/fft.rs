use anyhow::Result;
use core::convert::{TryInto};
use microfft::real;
use microfft::Complex32;
use ndarray::prelude::*;
use num::{traits::FloatConst, Float, NumCast};
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FFTError {
    msg: String,
}

impl FFTError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl error::Error for FFTError {}

impl fmt::Display for FFTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fft error: {}", self.msg)
    }
}

pub trait FFT<T> {
    type Item;
    fn fft(&self) -> Result<Array1<Self::Item>>;
}

impl<T> FFT<Complex32> for Array1<T>
where
    T: Float + FloatConst + Clone,
{
    type Item = Complex32;

    #[inline(always)]
    fn fft(&self) -> Result<Array1<Self::Item>> {
        let buffer = self.mapv(|v| NumCast::from(v).unwrap_or(0f32)).to_vec();
        match buffer.len() {
            2 => {
                let mut buffer: [_; 2] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_2(&mut buffer).to_vec())
            }
            4 => {
                let mut buffer: [_; 4] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_4(&mut buffer).to_vec())
            }
            8 => {
                let mut buffer: [_; 8] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_8(&mut buffer).to_vec())
            }
            16 => {
                let mut buffer: [_; 16] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_16(&mut buffer).to_vec())
            }
            32 => {
                let mut buffer: [_; 32] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_32(&mut buffer).to_vec())
            }
            64 => {
                let mut buffer: [_; 64] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_64(&mut buffer).to_vec())
            }
            128 => {
                let mut buffer: [_; 128] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_128(&mut buffer).to_vec())
            }
            256 => {
                let mut buffer: [_; 256] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_256(&mut buffer).to_vec())
            }
            512 => {
                let mut buffer: [_; 512] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_512(&mut buffer).to_vec())
            }
            1024 => {
                let mut buffer: [_; 1024] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_1024(&mut buffer).to_vec())
            }
            2048 => {
                let mut buffer: [_; 2048] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_2048(&mut buffer).to_vec())
            }
            4096 => {
                let mut buffer: [_; 4096] = buffer.as_slice().try_into().unwrap();
                Ok(real::rfft_4096(&mut buffer).to_vec())
            }

            _ => Err(FFTError::new(format!(
                "sample buffer length must be power of two between 2 and 4096, but is {}",
                self.len()
            ))
            .into()),
        }
        .and_then(|mut res| {
            // manually add the nyquist frequency
            // if false {
            let nyquist_fr_pos_val = res[0].im;
            res[0].im = 0.0;
            res.push(Complex32::new(nyquist_fr_pos_val, 0.0));
            // }
            Ok(Array1::from(res))
        })
    }
}
