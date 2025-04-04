// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{GLUniformType, Renderer};
use glib::{prelude::*, translate::*};
use std::{fmt, ptr};

glib::wrapper! {
    #[doc(alias = "GskGLShader")]
    pub struct GLShader(Object<ffi::GskGLShader, ffi::GskGLShaderClass>);

    match fn {
        type_ => || ffi::gsk_gl_shader_get_type(),
    }
}

impl GLShader {
    #[doc(alias = "gsk_gl_shader_new_from_bytes")]
    #[doc(alias = "new_from_bytes")]
    pub fn from_bytes(sourcecode: &glib::Bytes) -> GLShader {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_gl_shader_new_from_bytes(
                sourcecode.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_new_from_resource")]
    #[doc(alias = "new_from_resource")]
    pub fn from_resource(resource_path: &str) -> GLShader {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_gl_shader_new_from_resource(
                resource_path.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GLShader`] objects.
    ///
    /// This method returns an instance of [`GLShaderBuilder`](crate::builders::GLShaderBuilder) which can be used to create [`GLShader`] objects.
    pub fn builder() -> GLShaderBuilder {
        GLShaderBuilder::default()
    }

    #[doc(alias = "gsk_gl_shader_compile")]
    pub fn compile(&self, renderer: &impl IsA<Renderer>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::gsk_gl_shader_compile(
                self.to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
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

    #[doc(alias = "gsk_gl_shader_find_uniform_by_name")]
    pub fn find_uniform_by_name(&self, name: &str) -> i32 {
        unsafe {
            ffi::gsk_gl_shader_find_uniform_by_name(self.to_glib_none().0, name.to_glib_none().0)
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_bool")]
    #[doc(alias = "get_arg_bool")]
    pub fn arg_bool(&self, args: &glib::Bytes, idx: i32) -> bool {
        unsafe {
            from_glib(ffi::gsk_gl_shader_get_arg_bool(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_float")]
    #[doc(alias = "get_arg_float")]
    pub fn arg_float(&self, args: &glib::Bytes, idx: i32) -> f32 {
        unsafe {
            ffi::gsk_gl_shader_get_arg_float(self.to_glib_none().0, args.to_glib_none().0, idx)
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_int")]
    #[doc(alias = "get_arg_int")]
    pub fn arg_int(&self, args: &glib::Bytes, idx: i32) -> i32 {
        unsafe { ffi::gsk_gl_shader_get_arg_int(self.to_glib_none().0, args.to_glib_none().0, idx) }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_uint")]
    #[doc(alias = "get_arg_uint")]
    pub fn arg_uint(&self, args: &glib::Bytes, idx: i32) -> u32 {
        unsafe {
            ffi::gsk_gl_shader_get_arg_uint(self.to_glib_none().0, args.to_glib_none().0, idx)
        }
    }

    #[doc(alias = "gsk_gl_shader_get_args_size")]
    #[doc(alias = "get_args_size")]
    pub fn args_size(&self) -> usize {
        unsafe { ffi::gsk_gl_shader_get_args_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_get_n_textures")]
    #[doc(alias = "get_n_textures")]
    pub fn n_textures(&self) -> i32 {
        unsafe { ffi::gsk_gl_shader_get_n_textures(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_get_n_uniforms")]
    #[doc(alias = "get_n_uniforms")]
    pub fn n_uniforms(&self) -> i32 {
        unsafe { ffi::gsk_gl_shader_get_n_uniforms(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_get_resource")]
    #[doc(alias = "get_resource")]
    pub fn resource(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gsk_gl_shader_get_resource(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_gl_shader_get_source")]
    #[doc(alias = "get_source")]
    pub fn source(&self) -> glib::Bytes {
        unsafe { from_glib_none(ffi::gsk_gl_shader_get_source(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_gl_shader_get_uniform_name")]
    #[doc(alias = "get_uniform_name")]
    pub fn uniform_name(&self, idx: i32) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gsk_gl_shader_get_uniform_name(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_get_uniform_offset")]
    #[doc(alias = "get_uniform_offset")]
    pub fn uniform_offset(&self, idx: i32) -> i32 {
        unsafe { ffi::gsk_gl_shader_get_uniform_offset(self.to_glib_none().0, idx) }
    }

    #[doc(alias = "gsk_gl_shader_get_uniform_type")]
    #[doc(alias = "get_uniform_type")]
    pub fn uniform_type(&self, idx: i32) -> GLUniformType {
        unsafe {
            from_glib(ffi::gsk_gl_shader_get_uniform_type(
                self.to_glib_none().0,
                idx,
            ))
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GLShader`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GLShaderBuilder {
    resource: Option<String>,
    source: Option<glib::Bytes>,
}

impl GLShaderBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`GLShaderBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`GLShader`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GLShader {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref resource) = self.resource {
            properties.push(("resource", resource));
        }
        if let Some(ref source) = self.source {
            properties.push(("source", source));
        }
        glib::Object::new::<GLShader>(&properties)
    }

    pub fn resource(mut self, resource: &str) -> Self {
        self.resource = Some(resource.to_string());
        self
    }

    pub fn source(mut self, source: &glib::Bytes) -> Self {
        self.source = Some(source.clone());
        self
    }
}

impl fmt::Display for GLShader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLShader")
    }
}
