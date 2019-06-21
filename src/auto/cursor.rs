// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Texture;
use gdk_sys;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Cursor(Object<gdk_sys::GdkCursor, CursorClass>);

    match fn {
        get_type => || gdk_sys::gdk_cursor_get_type(),
    }
}

impl Cursor {
    pub fn new_from_name(name: &str, fallback: Option<&Cursor>) -> Option<Cursor> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gdk_sys::gdk_cursor_new_from_name(name.to_glib_none().0, fallback.to_glib_none().0))
        }
    }

    pub fn new_from_texture<P: IsA<Texture>>(texture: &P, hotspot_x: i32, hotspot_y: i32, fallback: Option<&Cursor>) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_cursor_new_from_texture(texture.as_ref().to_glib_none().0, hotspot_x, hotspot_y, fallback.to_glib_none().0))
        }
    }

    pub fn get_fallback(&self) -> Option<Cursor> {
        unsafe {
            from_glib_none(gdk_sys::gdk_cursor_get_fallback(self.to_glib_none().0))
        }
    }

    pub fn get_hotspot_x(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_cursor_get_hotspot_x(self.to_glib_none().0)
        }
    }

    pub fn get_hotspot_y(&self) -> i32 {
        unsafe {
            gdk_sys::gdk_cursor_get_hotspot_y(self.to_glib_none().0)
        }
    }

    pub fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gdk_sys::gdk_cursor_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_texture(&self) -> Option<Texture> {
        unsafe {
            from_glib_none(gdk_sys::gdk_cursor_get_texture(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cursor")
    }
}
