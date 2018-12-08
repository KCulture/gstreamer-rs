// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use GLAPI;
use GLDisplay;
use GLPlatform;
use GLSLProfile;
use GLSLVersion;
use GLWindow;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use gst;
use gst_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct GLContext(Object<ffi::GstGLContext, ffi::GstGLContextClass>): [
        gst::Object => gst_ffi::GstObject,
    ];

    match fn {
        get_type => || ffi::gst_gl_context_get_type(),
    }
}

impl GLContext {
    pub fn new<P: IsA<GLDisplay>>(display: &P) -> GLContext {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gst_gl_context_new(display.to_glib_none().0))
        }
    }

    pub fn get_current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_gl_context_get_current())
        }
    }

    pub fn get_current_gl_api(platform: GLPlatform) -> (GLAPI, u32, u32) {
        assert_initialized_main_thread!();
        unsafe {
            let mut major = mem::uninitialized();
            let mut minor = mem::uninitialized();
            let ret = from_glib(ffi::gst_gl_context_get_current_gl_api(platform.to_glib(), &mut major, &mut minor));
            (ret, major, minor)
        }
    }
}

unsafe impl Send for GLContext {}
unsafe impl Sync for GLContext {}

pub trait GLContextExt: 'static {
    fn activate(&self, activate: bool) -> Result<(), glib::error::BoolError>;

    fn can_share(&self, other_context: &GLContext) -> bool;

    fn check_feature(&self, feature: &str) -> bool;

    fn check_framebuffer_status(&self, fbo_target: u32) -> bool;

    fn check_gl_version(&self, api: GLAPI, maj: i32, min: i32) -> bool;

    fn clear_framebuffer(&self);

    fn clear_shader(&self);

    fn create<'a, P: Into<Option<&'a GLContext>>>(&self, other_context: P) -> Result<(), Error>;

    fn destroy(&self);

    fn fill_info(&self) -> Result<(), Error>;

    fn get_display(&self) -> GLDisplay;

    fn get_gl_api(&self) -> GLAPI;

    fn get_gl_platform(&self) -> GLPlatform;

    fn get_gl_platform_version(&self) -> (i32, i32);

    fn get_gl_version(&self) -> (i32, i32);

    fn get_window(&self) -> Option<GLWindow>;

    fn is_shared(&self) -> bool;

    fn set_shared_with(&self, share: &GLContext);

    fn set_window(&self, window: &GLWindow) -> bool;

    fn supports_glsl_profile_version(&self, version: GLSLVersion, profile: GLSLProfile) -> bool;

    fn swap_buffers(&self);
}

impl<O: IsA<GLContext>> GLContextExt for O {
    fn activate(&self, activate: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_gl_context_activate(self.to_glib_none().0, activate.to_glib()), "Failed to activate OpenGL context")
        }
    }

    fn can_share(&self, other_context: &GLContext) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_can_share(self.to_glib_none().0, other_context.to_glib_none().0))
        }
    }

    fn check_feature(&self, feature: &str) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_check_feature(self.to_glib_none().0, feature.to_glib_none().0))
        }
    }

    fn check_framebuffer_status(&self, fbo_target: u32) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_check_framebuffer_status(self.to_glib_none().0, fbo_target))
        }
    }

    fn check_gl_version(&self, api: GLAPI, maj: i32, min: i32) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_check_gl_version(self.to_glib_none().0, api.to_glib(), maj, min))
        }
    }

    fn clear_framebuffer(&self) {
        unsafe {
            ffi::gst_gl_context_clear_framebuffer(self.to_glib_none().0);
        }
    }

    fn clear_shader(&self) {
        unsafe {
            ffi::gst_gl_context_clear_shader(self.to_glib_none().0);
        }
    }

    fn create<'a, P: Into<Option<&'a GLContext>>>(&self, other_context: P) -> Result<(), Error> {
        let other_context = other_context.into();
        let other_context = other_context.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_gl_context_create(self.to_glib_none().0, other_context.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn destroy(&self) {
        unsafe {
            ffi::gst_gl_context_destroy(self.to_glib_none().0);
        }
    }

    fn fill_info(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gst_gl_context_fill_info(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_display(&self) -> GLDisplay {
        unsafe {
            from_glib_full(ffi::gst_gl_context_get_display(self.to_glib_none().0))
        }
    }

    fn get_gl_api(&self) -> GLAPI {
        unsafe {
            from_glib(ffi::gst_gl_context_get_gl_api(self.to_glib_none().0))
        }
    }

    fn get_gl_platform(&self) -> GLPlatform {
        unsafe {
            from_glib(ffi::gst_gl_context_get_gl_platform(self.to_glib_none().0))
        }
    }

    fn get_gl_platform_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::uninitialized();
            let mut minor = mem::uninitialized();
            ffi::gst_gl_context_get_gl_platform_version(self.to_glib_none().0, &mut major, &mut minor);
            (major, minor)
        }
    }

    fn get_gl_version(&self) -> (i32, i32) {
        unsafe {
            let mut maj = mem::uninitialized();
            let mut min = mem::uninitialized();
            ffi::gst_gl_context_get_gl_version(self.to_glib_none().0, &mut maj, &mut min);
            (maj, min)
        }
    }

    fn get_window(&self) -> Option<GLWindow> {
        unsafe {
            from_glib_full(ffi::gst_gl_context_get_window(self.to_glib_none().0))
        }
    }

    fn is_shared(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_is_shared(self.to_glib_none().0))
        }
    }

    fn set_shared_with(&self, share: &GLContext) {
        unsafe {
            ffi::gst_gl_context_set_shared_with(self.to_glib_none().0, share.to_glib_none().0);
        }
    }

    fn set_window(&self, window: &GLWindow) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_set_window(self.to_glib_none().0, window.to_glib_full()))
        }
    }

    fn supports_glsl_profile_version(&self, version: GLSLVersion, profile: GLSLProfile) -> bool {
        unsafe {
            from_glib(ffi::gst_gl_context_supports_glsl_profile_version(self.to_glib_none().0, version.to_glib(), profile.to_glib()))
        }
    }

    fn swap_buffers(&self) {
        unsafe {
            ffi::gst_gl_context_swap_buffers(self.to_glib_none().0);
        }
    }
}