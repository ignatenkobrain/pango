// This file was generated by gir (9f70278) from gir-files (0bcaef9)
// DO NOT EDIT

use Font;
use FontMetrics;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Fontset(Object<ffi::PangoFontset>);

    match fn {
        get_type => || ffi::pango_fontset_get_type(),
    }
}

pub trait FontsetExt {
    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/FontsetForeachFunc, data: P);

    fn get_font(&self, wc: u32) -> Option<Font>;

    fn get_metrics(&self) -> Option<FontMetrics>;
}

impl<O: IsA<Fontset>> FontsetExt for O {
    //fn foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/FontsetForeachFunc, data: P) {
    //    unsafe { TODO: call ffi::pango_fontset_foreach() }
    //}

    fn get_font(&self, wc: u32) -> Option<Font> {
        unsafe {
            from_glib_full(ffi::pango_fontset_get_font(self.to_glib_none().0, wc))
        }
    }

    fn get_metrics(&self) -> Option<FontMetrics> {
        unsafe {
            from_glib_full(ffi::pango_fontset_get_metrics(self.to_glib_none().0))
        }
    }
}
