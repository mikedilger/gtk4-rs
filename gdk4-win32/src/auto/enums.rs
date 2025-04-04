// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GdkWin32MessageFilterReturn")]
pub enum Win32MessageFilterReturn {
    #[doc(alias = "GDK_WIN32_MESSAGE_FILTER_CONTINUE")]
    Continue,
    #[doc(alias = "GDK_WIN32_MESSAGE_FILTER_REMOVE")]
    Remove,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Win32MessageFilterReturn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Win32MessageFilterReturn::{}",
            match *self {
                Self::Continue => "Continue",
                Self::Remove => "Remove",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for Win32MessageFilterReturn {
    type GlibType = ffi::GdkWin32MessageFilterReturn;

    fn into_glib(self) -> ffi::GdkWin32MessageFilterReturn {
        match self {
            Self::Continue => ffi::GDK_WIN32_MESSAGE_FILTER_CONTINUE,
            Self::Remove => ffi::GDK_WIN32_MESSAGE_FILTER_REMOVE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkWin32MessageFilterReturn> for Win32MessageFilterReturn {
    unsafe fn from_glib(value: ffi::GdkWin32MessageFilterReturn) -> Self {
        skip_assert_initialized!();
        match value {
            ffi::GDK_WIN32_MESSAGE_FILTER_CONTINUE => Self::Continue,
            ffi::GDK_WIN32_MESSAGE_FILTER_REMOVE => Self::Remove,
            value => Self::__Unknown(value),
        }
    }
}
