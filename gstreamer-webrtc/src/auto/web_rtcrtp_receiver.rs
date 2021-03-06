// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gst_web_rtc_sys;
use WebRTCDTLSTransport;

glib_wrapper! {
    pub struct WebRTCRTPReceiver(Object<gst_web_rtc_sys::GstWebRTCRTPReceiver, gst_web_rtc_sys::GstWebRTCRTPReceiverClass, WebRTCRTPReceiverClass>);

    match fn {
        get_type => || gst_web_rtc_sys::gst_webrtc_rtp_receiver_get_type(),
    }
}

impl WebRTCRTPReceiver {
    pub fn new() -> WebRTCRTPReceiver {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(gst_web_rtc_sys::gst_webrtc_rtp_receiver_new()) }
    }

    pub fn set_rtcp_transport(&self, transport: &WebRTCDTLSTransport) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_rtp_receiver_set_rtcp_transport(
                self.to_glib_none().0,
                transport.to_glib_none().0,
            );
        }
    }

    pub fn set_transport(&self, transport: &WebRTCDTLSTransport) {
        unsafe {
            gst_web_rtc_sys::gst_webrtc_rtp_receiver_set_transport(
                self.to_glib_none().0,
                transport.to_glib_none().0,
            );
        }
    }
}

impl Default for WebRTCRTPReceiver {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Send for WebRTCRTPReceiver {}
unsafe impl Sync for WebRTCRTPReceiver {}
