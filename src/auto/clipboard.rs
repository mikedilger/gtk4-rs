// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ContentFormats;
use ContentProvider;
use Display;
use Texture;
use gdk_sys;
use glib::StaticType;
use glib::Value;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Clipboard(Object<gdk_sys::GdkClipboard, ClipboardClass>);

    match fn {
        get_type => || gdk_sys::gdk_clipboard_get_type(),
    }
}

impl Clipboard {
    pub fn get_content(&self) -> Option<ContentProvider> {
        unsafe {
            from_glib_none(gdk_sys::gdk_clipboard_get_content(self.to_glib_none().0))
        }
    }

    pub fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(gdk_sys::gdk_clipboard_get_display(self.to_glib_none().0))
        }
    }

    pub fn get_formats(&self) -> Option<ContentFormats> {
        unsafe {
            from_glib_none(gdk_sys::gdk_clipboard_get_formats(self.to_glib_none().0))
        }
    }

    pub fn is_local(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_clipboard_is_local(self.to_glib_none().0))
        }
    }

    //pub fn read_async<P: FnOnce(Result<(/*Ignored*/gio::InputStream, GString), Error>) + Send + 'static>(&self, mime_types: &str, io_priority: /*Ignored*/glib::Priority, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call gdk_sys:gdk_clipboard_read_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn read_async_future(&self, mime_types: &str, io_priority: /*Ignored*/glib::Priority) -> Box_<dyn future::Future<Output = Result<(/*Ignored*/gio::InputStream, GString), Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let mime_types = String::from(mime_types);
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.read_async(
        //        &mime_types,
        //        io_priority,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    //pub fn read_text_async<P: FnOnce(Result<GString, Error>) + Send + 'static>(&self, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call gdk_sys:gdk_clipboard_read_text_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn read_text_async_future(&self) -> Box_<dyn future::Future<Output = Result<GString, Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.read_text_async(
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    //pub fn read_texture_async<P: FnOnce(Result<Texture, Error>) + Send + 'static>(&self, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call gdk_sys:gdk_clipboard_read_texture_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn read_texture_async_future(&self) -> Box_<dyn future::Future<Output = Result<Texture, Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.read_texture_async(
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    //pub fn read_value_async<P: FnOnce(Result</*Ignored*/glib::Value, Error>) + Send + 'static>(&self, type_: glib::types::Type, io_priority: /*Ignored*/glib::Priority, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call gdk_sys:gdk_clipboard_read_value_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn read_value_async_future(&self, type_: glib::types::Type, io_priority: /*Ignored*/glib::Priority) -> Box_<dyn future::Future<Output = Result</*Ignored*/glib::Value, Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.read_value_async(
        //        type_,
        //        io_priority,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    //pub fn set(&self, type_: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call gdk_sys:gdk_clipboard_set() }
    //}

    pub fn set_content<P: IsA<ContentProvider>>(&self, provider: Option<&P>) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_clipboard_set_content(self.to_glib_none().0, provider.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    pub fn set_text(&self, text: &str) {
        unsafe {
            gdk_sys::gdk_clipboard_set_text(self.to_glib_none().0, text.to_glib_none().0);
        }
    }

    pub fn set_texture<P: IsA<Texture>>(&self, texture: &P) {
        unsafe {
            gdk_sys::gdk_clipboard_set_texture(self.to_glib_none().0, texture.as_ref().to_glib_none().0);
        }
    }

    //pub fn set_valist(&self, type_: glib::types::Type, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call gdk_sys:gdk_clipboard_set_valist() }
    //}

    //pub fn set_value(&self, value: /*Ignored*/&glib::Value) {
    //    unsafe { TODO: call gdk_sys:gdk_clipboard_set_value() }
    //}

    //pub fn store_async<P: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: /*Ignored*/glib::Priority, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: P) {
    //    unsafe { TODO: call gdk_sys:gdk_clipboard_store_async() }
    //}

    //#[cfg(feature = "futures")]
    //pub fn store_async_future(&self, io_priority: /*Ignored*/glib::Priority) -> Box_<dyn future::Future<Output = Result<(), Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.store_async(
        //        io_priority,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}

    pub fn get_property_local(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"local\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn connect_changed<F: Fn(&Clipboard) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&Clipboard) + 'static>(this: *mut gdk_sys::GdkClipboard, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_content_notify<F: Fn(&Clipboard) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_content_trampoline<F: Fn(&Clipboard) + 'static>(this: *mut gdk_sys::GdkClipboard, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::content\0".as_ptr() as *const _,
                Some(transmute(notify_content_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_formats_notify<F: Fn(&Clipboard) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_formats_trampoline<F: Fn(&Clipboard) + 'static>(this: *mut gdk_sys::GdkClipboard, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::formats\0".as_ptr() as *const _,
                Some(transmute(notify_formats_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_local_notify<F: Fn(&Clipboard) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_trampoline<F: Fn(&Clipboard) + 'static>(this: *mut gdk_sys::GdkClipboard, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::local\0".as_ptr() as *const _,
                Some(transmute(notify_local_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Clipboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clipboard")
    }
}
