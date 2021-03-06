// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use gst_player_sys;

glib_wrapper! {
    pub struct PlayerVideoRenderer(Interface<gst_player_sys::GstPlayerVideoRenderer>);

    match fn {
        get_type => || gst_player_sys::gst_player_video_renderer_get_type(),
    }
}

unsafe impl Send for PlayerVideoRenderer {}
unsafe impl Sync for PlayerVideoRenderer {}

pub const NONE_PLAYER_VIDEO_RENDERER: Option<&PlayerVideoRenderer> = None;

pub trait PlayerVideoRendererExt: 'static {}

impl<O: IsA<PlayerVideoRenderer>> PlayerVideoRendererExt for O {}
