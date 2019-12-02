// Copyright (C) 2017 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use gst_audio_sys;

use std::ffi::CStr;
use std::fmt;
use std::str;

use glib::translate::{from_glib, FromGlib, ToGlib, ToGlibPtr};

impl ::AudioFormat {
    pub fn build_integer(
        sign: bool,
        endianness: ::AudioEndianness,
        width: i32,
        depth: i32,
    ) -> ::AudioFormat {
        assert_initialized_main_thread!();

        unsafe {
            from_glib(gst_audio_sys::gst_audio_format_build_integer(
                sign.to_glib(),
                endianness.to_glib(),
                width,
                depth,
            ))
        }
    }

    pub fn to_str<'a>(self) -> &'a str {
        if self == ::AudioFormat::Unknown {
            return "UNKNOWN";
        }

        unsafe {
            CStr::from_ptr(gst_audio_sys::gst_audio_format_to_string(self.to_glib()))
                .to_str()
                .unwrap()
        }
    }
}

impl str::FromStr for ::AudioFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        assert_initialized_main_thread!();

        unsafe {
            let fmt = ::AudioFormat::from_glib(gst_audio_sys::gst_audio_format_from_string(
                s.to_glib_none().0,
            ));
            if fmt == ::AudioFormat::Unknown {
                Err(())
            } else {
                Ok(fmt)
            }
        }
    }
}

impl fmt::Display for ::AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str((*self).to_str())
    }
}

pub const AUDIO_FORMAT_UNKNOWN: ::AudioFormat = ::AudioFormat::Unknown;
pub const AUDIO_FORMAT_ENCODED: ::AudioFormat = ::AudioFormat::Encoded;
pub const AUDIO_FORMAT_S8: ::AudioFormat = ::AudioFormat::S8;
pub const AUDIO_FORMAT_U8: ::AudioFormat = ::AudioFormat::U8;

#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S16: ::AudioFormat = ::AudioFormat::S16be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U16: ::AudioFormat = ::AudioFormat::U16be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S2432: ::AudioFormat = ::AudioFormat::S2432be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U2432: ::AudioFormat = ::AudioFormat::U2432be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S32: ::AudioFormat = ::AudioFormat::S32be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U32: ::AudioFormat = ::AudioFormat::U32be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S24: ::AudioFormat = ::AudioFormat::S24be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U24: ::AudioFormat = ::AudioFormat::U24be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S20: ::AudioFormat = ::AudioFormat::S20be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U20: ::AudioFormat = ::AudioFormat::U20be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_S18: ::AudioFormat = ::AudioFormat::S18be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_U18: ::AudioFormat = ::AudioFormat::U18be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_F32: ::AudioFormat = ::AudioFormat::F32be;
#[cfg(target_endian = "big")]
pub const AUDIO_FORMAT_F64: ::AudioFormat = ::AudioFormat::F64be;

#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S16: ::AudioFormat = ::AudioFormat::S16le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U16: ::AudioFormat = ::AudioFormat::U16le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S2432: ::AudioFormat = ::AudioFormat::S2432le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U2432: ::AudioFormat = ::AudioFormat::U2432le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S32: ::AudioFormat = ::AudioFormat::S32le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U32: ::AudioFormat = ::AudioFormat::U32le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S24: ::AudioFormat = ::AudioFormat::S24le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U24: ::AudioFormat = ::AudioFormat::U24le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S20: ::AudioFormat = ::AudioFormat::S20le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U20: ::AudioFormat = ::AudioFormat::U20le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_S18: ::AudioFormat = ::AudioFormat::S18le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_U18: ::AudioFormat = ::AudioFormat::U18le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_F32: ::AudioFormat = ::AudioFormat::F32le;
#[cfg(target_endian = "little")]
pub const AUDIO_FORMAT_F64: ::AudioFormat = ::AudioFormat::F64le;

#[cfg(test)]
mod tests {
    use gst;

    #[test]
    fn test_display() {
        gst::init().unwrap();

        format!("{}", ::AudioFormat::S16be);
    }
}
