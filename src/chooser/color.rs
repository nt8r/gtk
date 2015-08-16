// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::c_int;

use glib::translate::*;
use glib::types;
use gdk_ffi::GdkRGBA;
use ffi;

use object::{Object, Upcast};

use Orientation;

pub type ColorChooser = Object<ffi::GtkColorChooser>;

impl types::StaticType for ColorChooser {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_color_chooser_get_type()) }
    }
}

pub trait ColorChooserExt {
    fn get_rgba(&self) -> GdkRGBA;
    fn set_rgba(&self, color: GdkRGBA);
    fn get_use_alpha(&self) -> bool;
    fn set_use_alpha(&self, use_alpha: bool);
    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: Vec<GdkRGBA>);
}

impl<O: Upcast<ColorChooser>> ColorChooserExt for O {
    fn get_rgba(&self) -> GdkRGBA {
        let mut color = GdkRGBA {
            red: 0f64,
            green: 0f64,
            blue: 0f64,
            alpha: 0f64
        };
        unsafe { ffi::gtk_color_chooser_get_rgba(self.upcast().to_glib_none().0, &mut color) };
        color
    }

    fn set_rgba(&self, color: GdkRGBA) {
        unsafe { ffi::gtk_color_chooser_set_rgba(self.upcast().to_glib_none().0, &color) };
    }

    fn get_use_alpha(&self) -> bool {
        unsafe { from_glib(ffi::gtk_color_chooser_get_use_alpha(self.upcast().to_glib_none().0)) }
    }

    fn set_use_alpha(&self, use_alpha: bool) {
        unsafe {
            ffi::gtk_color_chooser_set_use_alpha(self.upcast().to_glib_none().0,
                use_alpha.to_glib())
        }
    }

    fn add_palette(&self, orientation: Orientation, colors_per_line: i32, colors: Vec<GdkRGBA>) {
        unsafe {
            ffi::gtk_color_chooser_add_palette(self.upcast().to_glib_none().0, orientation,
                colors_per_line, colors.len() as c_int, colors.as_ptr() as *mut GdkRGBA) }
    }
}
