// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gst_base_sys;

bitflags! {
    pub struct BaseParseFrameFlags: u32 {
        const NEW_FRAME = 1;
        const NO_FRAME = 2;
        const CLIP = 4;
        const DROP = 8;
        const QUEUE = 16;
    }
}

#[doc(hidden)]
impl ToGlib for BaseParseFrameFlags {
    type GlibType = gst_base_sys::GstBaseParseFrameFlags;

    fn to_glib(&self) -> gst_base_sys::GstBaseParseFrameFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gst_base_sys::GstBaseParseFrameFlags> for BaseParseFrameFlags {
    fn from_glib(value: gst_base_sys::GstBaseParseFrameFlags) -> BaseParseFrameFlags {
        skip_assert_initialized!();
        BaseParseFrameFlags::from_bits_truncate(value)
    }
}
