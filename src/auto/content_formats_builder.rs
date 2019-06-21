// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ContentFormats;
use gdk_sys;
use glib;
use glib::translate::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ContentFormatsBuilder(Shared<gdk_sys::GdkContentFormatsBuilder>);

    match fn {
        ref => |ptr| gdk_sys::gdk_content_formats_builder_ref(ptr),
        unref => |ptr| gdk_sys::gdk_content_formats_builder_unref(ptr),
        get_type => || gdk_sys::gdk_content_formats_builder_get_type(),
    }
}

impl ContentFormatsBuilder {
    pub fn new() -> ContentFormatsBuilder {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_builder_new())
        }
    }

    pub fn add_formats(&self, formats: &ContentFormats) {
        unsafe {
            gdk_sys::gdk_content_formats_builder_add_formats(self.to_glib_none().0, formats.to_glib_none().0);
        }
    }

    pub fn add_gtype(&self, type_: glib::types::Type) {
        unsafe {
            gdk_sys::gdk_content_formats_builder_add_gtype(self.to_glib_none().0, type_.to_glib());
        }
    }

    pub fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            gdk_sys::gdk_content_formats_builder_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    pub fn free_to_formats(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_builder_free_to_formats(self.to_glib_none().0))
        }
    }

    pub fn to_formats(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_full(gdk_sys::gdk_content_formats_builder_to_formats(self.to_glib_none().0))
        }
    }
}

impl Default for ContentFormatsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
