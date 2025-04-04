// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{RecentData, RecentInfo};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, ptr};

glib::wrapper! {
    #[doc(alias = "GtkRecentManager")]
    pub struct RecentManager(Object<ffi::GtkRecentManager, ffi::GtkRecentManagerClass>);

    match fn {
        type_ => || ffi::gtk_recent_manager_get_type(),
    }
}

impl RecentManager {
    pub const NONE: Option<&'static RecentManager> = None;

    #[doc(alias = "gtk_recent_manager_new")]
    pub fn new() -> RecentManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_recent_manager_new()) }
    }

    #[doc(alias = "gtk_recent_manager_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> RecentManager {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_recent_manager_get_default()) }
    }
}

impl Default for RecentManager {
    fn default() -> Self {
        Self::new()
    }
}

pub trait RecentManagerExt: 'static {
    #[doc(alias = "gtk_recent_manager_add_full")]
    fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool;

    #[doc(alias = "gtk_recent_manager_add_item")]
    fn add_item(&self, uri: &str) -> bool;

    #[doc(alias = "gtk_recent_manager_get_items")]
    #[doc(alias = "get_items")]
    fn items(&self) -> Vec<RecentInfo>;

    #[doc(alias = "gtk_recent_manager_has_item")]
    fn has_item(&self, uri: &str) -> bool;

    #[doc(alias = "gtk_recent_manager_lookup_item")]
    fn lookup_item(&self, uri: &str) -> Result<Option<RecentInfo>, glib::Error>;

    #[doc(alias = "gtk_recent_manager_move_item")]
    fn move_item(&self, uri: &str, new_uri: Option<&str>) -> Result<(), glib::Error>;

    #[doc(alias = "gtk_recent_manager_purge_items")]
    fn purge_items(&self) -> Result<i32, glib::Error>;

    #[doc(alias = "gtk_recent_manager_remove_item")]
    fn remove_item(&self, uri: &str) -> Result<(), glib::Error>;

    fn filename(&self) -> Option<glib::GString>;

    fn size(&self) -> i32;

    #[doc(alias = "changed")]
    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "size")]
    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<RecentManager>> RecentManagerExt for O {
    fn add_full(&self, uri: &str, recent_data: &RecentData) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_add_full(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                recent_data.to_glib_none().0,
            ))
        }
    }

    fn add_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_add_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn items(&self) -> Vec<RecentInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_recent_manager_get_items(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_manager_has_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
            ))
        }
    }

    fn lookup_item(&self, uri: &str) -> Result<Option<RecentInfo>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_recent_manager_lookup_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn move_item(&self, uri: &str, new_uri: Option<&str>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gtk_recent_manager_move_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                new_uri.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn purge_items(&self) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::gtk_recent_manager_purge_items(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn remove_item(&self, uri: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gtk_recent_manager_remove_item(
                self.as_ref().to_glib_none().0,
                uri.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn filename(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "filename")
    }

    fn size(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "size")
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P: IsA<RecentManager>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRecentManager,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_size_trampoline<P: IsA<RecentManager>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkRecentManager,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(RecentManager::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for RecentManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("RecentManager")
    }
}
