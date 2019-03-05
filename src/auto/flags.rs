// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_sys;
use pango_sys;

bitflags! {
    pub struct FontMask: u32 {
        const FAMILY = 1;
        const STYLE = 2;
        const VARIANT = 4;
        const WEIGHT = 8;
        const STRETCH = 16;
        const SIZE = 32;
        const GRAVITY = 64;
        const VARIATIONS = 128;
    }
}

#[doc(hidden)]
impl ToGlib for FontMask {
    type GlibType = pango_sys::PangoFontMask;

    fn to_glib(&self) -> pango_sys::PangoFontMask {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<pango_sys::PangoFontMask> for FontMask {
    fn from_glib(value: pango_sys::PangoFontMask) -> FontMask {
        FontMask::from_bits_truncate(value)
    }
}

impl StaticType for FontMask {
    fn static_type() -> Type {
        unsafe { from_glib(pango_sys::pango_font_mask_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FontMask {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FontMask {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_flags(value.to_glib_none().0))
    }
}

impl SetValue for FontMask {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib())
    }
}

