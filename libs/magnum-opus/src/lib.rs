//! Safe Rust bindings for libopus

#![allow(clippy::style)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate magnum_opus_sys as ffi;

use libc::{c_int, c_uchar};
use std::ffi::CStr;
use std::fmt;
use std::error;
use std::marker::PhantomData;

// Re-export all opus constants
pub use ffi::{
    OPUS_OK, OPUS_BAD_ARG, OPUS_BUFFER_TOO_SMALL, OPUS_INTERNAL_ERROR,
    OPUS_INVALID_PACKET, OPUS_UNIMPLEMENTED, OPUS_INVALID_STATE,
    OPUS_ALLOC_FAIL,
    
    OPUS_APPLICATION_VOIP, OPUS_APPLICATION_AUDIO,
    OPUS_APPLICATION_RESTRICTED_LOWDELAY,
    
    OPUS_SIGNAL_VOICE, OPUS_SIGNAL_MUSIC,
    OPUS_BANDWIDTH_NARROWBAND, OPUS_BANDWIDTH_MEDIUMBAND,
    OPUS_BANDWIDTH_WIDEBAND, OPUS_BANDWIDTH_SUPERWIDEBAND,
    OPUS_BANDWIDTH_FULLBAND,
    
    OPUS_FRAMESIZE_ARG, OPUS_FRAMESIZE_2_5_MS, OPUS_FRAMESIZE_5_MS,
    OPUS_FRAMESIZE_10_MS, OPUS_FRAMESIZE_20_MS, OPUS_FRAMESIZE_40_MS,
    OPUS_FRAMESIZE_60_MS, OPUS_FRAMESIZE_80_MS, OPUS_FRAMESIZE_100_MS,
    OPUS_FRAMESIZE_120_MS,
};

// Control codes
pub const OPUS_SET_APPLICATION_REQUEST: c_int = 4000;
pub const OPUS_GET_APPLICATION_REQUEST: c_int = 4001;
pub const OPUS_SET_BITRATE_REQUEST: c_int = 4002;
pub const OPUS_GET_BITRATE_REQUEST: c_int = 4003;
pub const OPUS_SET_MAX_BANDWIDTH_REQUEST: c_int = 4004;
pub const OPUS_GET_MAX_BANDWIDTH_REQUEST: c_int = 4005;
pub const OPUS_SET_VBR_REQUEST: c_int = 4006;
pub const OPUS_GET_VBR_REQUEST: c_int = 4007;
pub const OPUS_SET_BANDWIDTH_REQUEST: c_int = 4008;
pub const OPUS_GET_BANDWIDTH_REQUEST: c_int = 4009;
pub const OPUS_SET_COMPLEXITY_REQUEST: c_int = 4010;
pub const OPUS_GET_COMPLEXITY_REQUEST: c_int = 4011;
pub const OPUS_SET_INBAND_FEC_REQUEST: c_int = 4012;
pub const OPUS_GET_INBAND_FEC_REQUEST: c_int = 4013;
pub const OPUS_SET_PACKET_LOSS_PERC_REQUEST: c_int = 4014;
pub const OPUS_GET_PACKET_LOSS_PERC_REQUEST: c_int = 4015;
pub const OPUS_SET_DTX_REQUEST: c_int = 4016;
pub const OPUS_GET_DTX_REQUEST: c_int = 4017;
pub const OPUS_SET_VBR_CONSTRAINT_REQUEST: c_int = 4020;
pub const OPUS_GET_VBR_CONSTRAINT_REQUEST: c_int = 4021;
pub const OPUS_SET_FORCE_CHANNELS_REQUEST: c_int = 4022;
pub const OPUS_GET_FORCE_CHANNELS_REQUEST: c_int = 4023;
pub const OPUS_SET_SIGNAL_REQUEST: c_int = 4024;
pub const OPUS_GET_SIGNAL_REQUEST: c_int = 4025;
pub const OPUS_GET_LOOKAHEAD_REQUEST: c_int = 4027;
pub const OPUS_GET_SAMPLE_RATE_REQUEST: c_int = 4029;
pub const OPUS_GET_FINAL_RANGE_REQUEST: c_int = 4031;
pub const OPUS_GET_PITCH_REQUEST: c_int = 4033;
pub const OPUS_SET_GAIN_REQUEST: c_int = 4034;
pub const OPUS_GET_GAIN_REQUEST: c_int = 4045;
pub const OPUS_SET_LSB_DEPTH_REQUEST: c_int = 4036;
pub const OPUS_GET_LSB_DEPTH_REQUEST: c_int = 4037;
pub const OPUS_GET_LAST_PACKET_DURATION_REQUEST: c_int = 4039;
pub const OPUS_SET_EXPERT_FRAME_DURATION_REQUEST: c_int = 4040;
pub const OPUS_GET_EXPERT_FRAME_DURATION_REQUEST: c_int = 4041;
pub const OPUS_SET_PREDICTION_DISABLED_REQUEST: c_int = 4042;
pub const OPUS_GET_PREDICTION_DISABLED_REQUEST: c_int = 4043;
pub const OPUS_SET_PHASE_INVERSION_DISABLED_REQUEST: c_int = 4046;
pub const OPUS_GET_PHASE_INVERSION_DISABLED_REQUEST: c_int = 4047;
pub const OPUS_GET_IN_DTX_REQUEST: c_int = 4049;

/// Error codes returned by libopus
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    BadArg,
    BufferTooSmall,
    InternalError,
    InvalidPacket,
    Unimplemented,
    InvalidState,
    AllocFail,
    Unknown(c_int),
}

impl ErrorCode {
    fn from_int(code: c_int) -> ErrorCode {
        match code {
            OPUS_BAD_ARG => ErrorCode::BadArg,
            OPUS_BUFFER_TOO_SMALL => ErrorCode::BufferTooSmall,
            OPUS_INTERNAL_ERROR => ErrorCode::InternalError,
            OPUS_INVALID_PACKET => ErrorCode::InvalidPacket,
            OPUS_UNIMPLEMENTED => ErrorCode::Unimplemented,
            OPUS_INVALID_STATE => ErrorCode::InvalidState,
            OPUS_ALLOC_FAIL => ErrorCode::AllocFail,
            _ => ErrorCode::Unknown(code),
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorCode::BadArg => write!(f, "Bad argument"),
            ErrorCode::BufferTooSmall => write!(f, "Buffer too small"),
            ErrorCode::InternalError => write!(f, "Internal error"),
            ErrorCode::InvalidPacket => write!(f, "Invalid packet"),
            ErrorCode::Unimplemented => write!(f, "Unimplemented"),
            ErrorCode::InvalidState => write!(f, "Invalid state"),
            ErrorCode::AllocFail => write!(f, "Allocation failed"),
            ErrorCode::Unknown(code) => write!(f, "Unknown error code: {}", code),
        }
    }
}

impl error::Error for ErrorCode {}

/// Possible coding modes for the codec
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Application {
    /// Best for most VoIP/videoconference applications where listening quality and intelligibility matter most
    Voip = OPUS_APPLICATION_VOIP,
    /// Best for broadcast/high-fidelity application where the decoded audio should be as close as possible to the input
    Audio = OPUS_APPLICATION_AUDIO,
    /// Only use when lowest-achievable latency is what matters most
    LowDelay = OPUS_APPLICATION_RESTRICTED_LOWDELAY,
}

/// Signal types
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    /// Signal being encoded is voice
    Voice = OPUS_SIGNAL_VOICE,
    /// Signal being encoded is music
    Music = OPUS_SIGNAL_MUSIC,
}

/// Available audio channels
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channels {
    /// 1 channel
    Mono = 1,
    /// 2 channels
    Stereo = 2,
}

/// Available sampling rates
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleRate {
    /// 8kHz
    Hz8000 = 8000,
    /// 12kHz
    Hz12000 = 12000,
    /// 16kHz
    Hz16000 = 16000,
    /// 24kHz
    Hz24000 = 24000,
    /// 48kHz
    Hz48000 = 48000,
}

/// Audio bandwidth
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bandwidth {
    /// Narrowband (4kHz)
    Narrowband = OPUS_BANDWIDTH_NARROWBAND,
    /// Mediumband (6kHz)
    Mediumband = OPUS_BANDWIDTH_MEDIUMBAND,
    /// Wideband (8kHz)
    Wideband = OPUS_BANDWIDTH_WIDEBAND,
    /// Superwideband (12kHz)
    Superwideband = OPUS_BANDWIDTH_SUPERWIDEBAND,
    /// Fullband (20kHz)
    Fullband = OPUS_BANDWIDTH_FULLBAND,
}

/// Frame sizes
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Framesize {
    /// Select frame size based on argument
    Arg = OPUS_FRAMESIZE_ARG,
    /// 2.5ms
    Ms2_5 = OPUS_FRAMESIZE_2_5_MS,
    /// 5ms
    Ms5 = OPUS_FRAMESIZE_5_MS,
    /// 10ms
    Ms10 = OPUS_FRAMESIZE_10_MS,
    /// 20ms
    Ms20 = OPUS_FRAMESIZE_20_MS,
    /// 40ms
    Ms40 = OPUS_FRAMESIZE_40_MS,
    /// 60ms
    Ms60 = OPUS_FRAMESIZE_60_MS,
    /// 80ms
    Ms80 = OPUS_FRAMESIZE_80_MS,
    /// 100ms
    Ms100 = OPUS_FRAMESIZE_100_MS,
    /// 120ms
    Ms120 = OPUS_FRAMESIZE_120_MS,
}

/// Opus encoder
pub struct Encoder {
    ptr: *mut ffi::OpusEncoder,
    channels: Channels,
}

impl Encoder {
    /// Create a new encoder
    pub fn new(sample_rate: SampleRate, channels: Channels, application: Application) -> Result<Encoder, ErrorCode> {
        let mut error = 0;
        let ptr = unsafe {
            ffi::opus_encoder_create(sample_rate as i32, channels as i32, application as i32, &mut error)
        };
        
        if error != OPUS_OK || ptr.is_null() {
            Err(ErrorCode::from_int(error))
        } else {
            Ok(Encoder { ptr, channels })
        }
    }
    
    /// Encode a frame of audio
    pub fn encode(&mut self, input: &[i16], output: &mut [u8]) -> Result<usize, ErrorCode> {
        let len = unsafe {
            ffi::opus_encode(
                self.ptr,
                input.as_ptr(),
                (input.len() / self.channels as usize) as i32,
                output.as_mut_ptr(),
                output.len() as i32,
            )
        };
        
        if len < 0 {
            Err(ErrorCode::from_int(len))
        } else {
            Ok(len as usize)
        }
    }
    
    /// Encode a frame of float audio
    pub fn encode_float(&mut self, input: &[f32], output: &mut [u8]) -> Result<usize, ErrorCode> {
        let len = unsafe {
            ffi::opus_encode_float(
                self.ptr,
                input.as_ptr(),
                (input.len() / self.channels as usize) as i32,
                output.as_mut_ptr(),
                output.len() as i32,
            )
        };
        
        if len < 0 {
            Err(ErrorCode::from_int(len))
        } else {
            Ok(len as usize)
        }
    }
    
    /// Set encoder control parameter
    pub fn set_control(&mut self, request: i32, value: i32) -> Result<(), ErrorCode> {
        let ret = unsafe {
            ffi::opus_encoder_ctl(self.ptr, request, value)
        };
        
        if ret < 0 {
            Err(ErrorCode::from_int(ret))
        } else {
            Ok(())
        }
    }
    
    /// Get encoder control parameter
    pub fn get_control(&self, request: i32) -> Result<i32, ErrorCode> {
        let mut value = 0i32;
        let ret = unsafe {
            ffi::opus_encoder_ctl(self.ptr as *mut _, request, &mut value)
        };
        
        if ret < 0 {
            Err(ErrorCode::from_int(ret))
        } else {
            Ok(value)
        }
    }
    
    /// Reset the encoder state
    pub fn reset(&mut self) -> Result<(), ErrorCode> {
        let ret = unsafe {
            ffi::opus_encoder_ctl(self.ptr, 4028) // OPUS_RESET_STATE
        };
        
        if ret < 0 {
            Err(ErrorCode::from_int(ret))
        } else {
            Ok(())
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            ffi::opus_encoder_destroy(self.ptr);
        }
    }
}

unsafe impl Send for Encoder {}

/// Opus decoder
pub struct Decoder {
    ptr: *mut ffi::OpusDecoder,
    channels: Channels,
}

impl Decoder {
    /// Create a new decoder
    pub fn new(sample_rate: SampleRate, channels: Channels) -> Result<Decoder, ErrorCode> {
        let mut error = 0;
        let ptr = unsafe {
            ffi::opus_decoder_create(sample_rate as i32, channels as i32, &mut error)
        };
        
        if error != OPUS_OK || ptr.is_null() {
            Err(ErrorCode::from_int(error))
        } else {
            Ok(Decoder { ptr, channels })
        }
    }
    
    /// Decode an Opus packet
    pub fn decode(&mut self, input: &[u8], output: &mut [i16], fec: bool) -> Result<usize, ErrorCode> {
        let len = unsafe {
            ffi::opus_decode(
                self.ptr,
                input.as_ptr(),
                input.len() as i32,
                output.as_mut_ptr(),
                (output.len() / self.channels as usize) as i32,
                fec as i32,
            )
        };
        
        if len < 0 {
            Err(ErrorCode::from_int(len))
        } else {
            Ok(len as usize)
        }
    }
    
    /// Decode an Opus packet to float
    pub fn decode_float(&mut self, input: &[u8], output: &mut [f32], fec: bool) -> Result<usize, ErrorCode> {
        let len = unsafe {
            ffi::opus_decode_float(
                self.ptr,
                input.as_ptr(),
                input.len() as i32,
                output.as_mut_ptr(),
                (output.len() / self.channels as usize) as i32,
                fec as i32,
            )
        };
        
        if len < 0 {
            Err(ErrorCode::from_int(len))
        } else {
            Ok(len as usize)
        }
    }
    
    /// Set decoder control parameter
    pub fn set_control(&mut self, request: i32, value: i32) -> Result<(), ErrorCode> {
        let ret = unsafe {
            ffi::opus_decoder_ctl(self.ptr, request, value)
        };
        
        if ret < 0 {
            Err(ErrorCode::from_int(ret))
        } else {
            Ok(())
        }
    }
    
    /// Get decoder control parameter
    pub fn get_control(&self, request: i32) -> Result<i32, ErrorCode> {
        let mut value = 0i32;
        let ret = unsafe {
            ffi::opus_decoder_ctl(self.ptr as *mut _, request, &mut value)
        };
        
        if ret < 0 {
            Err(ErrorCode::from_int(ret))
        } else {
            Ok(value)
        }
    }
    
    /// Reset the decoder state
    pub fn reset(&mut self) -> Result<(), ErrorCode> {
        let ret = unsafe {
            ffi::opus_decoder_ctl(self.ptr, 4028) // OPUS_RESET_STATE
        };
        
        if ret < 0 {
            Err(ErrorCode::from_int(ret))
        } else {
            Ok(())
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe {
            ffi::opus_decoder_destroy(self.ptr);
        }
    }
}

unsafe impl Send for Decoder {}

/// Get the libopus version string
pub fn version() -> &'static str {
    unsafe {
        CStr::from_ptr(ffi::opus_get_version_string())
            .to_str()
            .unwrap_or("unknown")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let v = version();
        println!("libopus version: {}", v);
        assert!(!v.is_empty());
    }

    #[test]
    fn test_encoder() {
        let encoder = Encoder::new(SampleRate::Hz48000, Channels::Mono, Application::Voip);
        assert!(encoder.is_ok());
    }

    #[test]
    fn test_decoder() {
        let decoder = Decoder::new(SampleRate::Hz48000, Channels::Mono);
        assert!(decoder.is_ok());
    }
}
