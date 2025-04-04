// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

#[cfg(any(feature = "v4_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
use crate::GLAPI;
use crate::{Display, DrawContext, Surface};
#[cfg(any(feature = "v4_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{prelude::*, translate::*};
#[cfg(any(feature = "v4_6", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
use std::{boxed::Box as Box_, mem::transmute};
use std::{fmt, mem, ptr};

glib::wrapper! {
    #[doc(alias = "GdkGLContext")]
    pub struct GLContext(Object<ffi::GdkGLContext>) @extends DrawContext;

    match fn {
        type_ => || ffi::gdk_gl_context_get_type(),
    }
}

impl GLContext {
    pub const NONE: Option<&'static GLContext> = None;

    #[doc(alias = "gdk_gl_context_clear_current")]
    pub fn clear_current() {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gdk_gl_context_clear_current();
        }
    }

    #[doc(alias = "gdk_gl_context_get_current")]
    #[doc(alias = "get_current")]
    pub fn current() -> Option<GLContext> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_gl_context_get_current()) }
    }
}

pub trait GLContextExt: 'static {
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gdk_gl_context_get_allowed_apis")]
    #[doc(alias = "get_allowed_apis")]
    fn allowed_apis(&self) -> GLAPI;

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gdk_gl_context_get_api")]
    #[doc(alias = "get_api")]
    fn api(&self) -> GLAPI;

    #[doc(alias = "gdk_gl_context_get_debug_enabled")]
    #[doc(alias = "get_debug_enabled")]
    fn is_debug_enabled(&self) -> bool;

    #[doc(alias = "gdk_gl_context_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display>;

    #[doc(alias = "gdk_gl_context_get_forward_compatible")]
    #[doc(alias = "get_forward_compatible")]
    fn is_forward_compatible(&self) -> bool;

    #[doc(alias = "gdk_gl_context_get_required_version")]
    #[doc(alias = "get_required_version")]
    fn required_version(&self) -> (i32, i32);

    #[cfg_attr(feature = "v4_4", deprecated = "Since 4.4")]
    #[allow(deprecated)]
    #[doc(alias = "gdk_gl_context_get_shared_context")]
    #[doc(alias = "get_shared_context")]
    #[must_use]
    fn shared_context(&self) -> Option<GLContext>;

    #[doc(alias = "gdk_gl_context_get_surface")]
    #[doc(alias = "get_surface")]
    fn surface(&self) -> Option<Surface>;

    #[doc(alias = "gdk_gl_context_get_use_es")]
    #[doc(alias = "get_use_es")]
    fn uses_es(&self) -> bool;

    #[doc(alias = "gdk_gl_context_get_version")]
    #[doc(alias = "get_version")]
    fn version(&self) -> (i32, i32);

    #[doc(alias = "gdk_gl_context_is_legacy")]
    fn is_legacy(&self) -> bool;

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    #[doc(alias = "gdk_gl_context_is_shared")]
    fn is_shared(&self, other: &impl IsA<GLContext>) -> bool;

    #[doc(alias = "gdk_gl_context_make_current")]
    fn make_current(&self);

    #[doc(alias = "gdk_gl_context_realize")]
    fn realize(&self) -> Result<(), glib::Error>;

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "gdk_gl_context_set_allowed_apis")]
    fn set_allowed_apis(&self, apis: GLAPI);

    #[doc(alias = "gdk_gl_context_set_debug_enabled")]
    fn set_debug_enabled(&self, enabled: bool);

    #[doc(alias = "gdk_gl_context_set_forward_compatible")]
    fn set_forward_compatible(&self, compatible: bool);

    #[doc(alias = "gdk_gl_context_set_required_version")]
    fn set_required_version(&self, major: i32, minor: i32);

    #[doc(alias = "gdk_gl_context_set_use_es")]
    fn set_use_es(&self, use_es: i32);

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "allowed-apis")]
    fn connect_allowed_apis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    #[doc(alias = "api")]
    fn connect_api_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GLContext>> GLContextExt for O {
    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    fn allowed_apis(&self) -> GLAPI {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_allowed_apis(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    fn api(&self) -> GLAPI {
        unsafe { from_glib(ffi::gdk_gl_context_get_api(self.as_ref().to_glib_none().0)) }
    }

    fn is_debug_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_debug_enabled(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_display(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_forward_compatible(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_forward_compatible(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            ffi::gdk_gl_context_get_required_version(
                self.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            (major.assume_init(), minor.assume_init())
        }
    }

    #[allow(deprecated)]
    fn shared_context(&self) -> Option<GLContext> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_shared_context(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn surface(&self) -> Option<Surface> {
        unsafe {
            from_glib_none(ffi::gdk_gl_context_get_surface(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn uses_es(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_get_use_es(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            ffi::gdk_gl_context_get_version(
                self.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            (major.assume_init(), minor.assume_init())
        }
    }

    fn is_legacy(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_is_legacy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v4_4", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_4")))]
    fn is_shared(&self, other: &impl IsA<GLContext>) -> bool {
        unsafe {
            from_glib(ffi::gdk_gl_context_is_shared(
                self.as_ref().to_glib_none().0,
                other.as_ref().to_glib_none().0,
            ))
        }
    }

    fn make_current(&self) {
        unsafe {
            ffi::gdk_gl_context_make_current(self.as_ref().to_glib_none().0);
        }
    }

    fn realize(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gdk_gl_context_realize(self.as_ref().to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    fn set_allowed_apis(&self, apis: GLAPI) {
        unsafe {
            ffi::gdk_gl_context_set_allowed_apis(self.as_ref().to_glib_none().0, apis.into_glib());
        }
    }

    fn set_debug_enabled(&self, enabled: bool) {
        unsafe {
            ffi::gdk_gl_context_set_debug_enabled(
                self.as_ref().to_glib_none().0,
                enabled.into_glib(),
            );
        }
    }

    fn set_forward_compatible(&self, compatible: bool) {
        unsafe {
            ffi::gdk_gl_context_set_forward_compatible(
                self.as_ref().to_glib_none().0,
                compatible.into_glib(),
            );
        }
    }

    fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gdk_gl_context_set_required_version(self.as_ref().to_glib_none().0, major, minor);
        }
    }

    fn set_use_es(&self, use_es: i32) {
        unsafe {
            ffi::gdk_gl_context_set_use_es(self.as_ref().to_glib_none().0, use_es);
        }
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    fn connect_allowed_apis_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_allowed_apis_trampoline<
            P: IsA<GLContext>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GdkGLContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::allowed-apis\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_allowed_apis_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_6", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_6")))]
    fn connect_api_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_api_trampoline<P: IsA<GLContext>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkGLContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::api\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_api_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GLContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLContext")
    }
}
