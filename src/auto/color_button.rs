// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use ColorChooser;
use Container;
use Widget;
use gdk;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ColorButton(Object<gtk_sys::GtkColorButton, gtk_sys::GtkColorButtonClass, ColorButtonClass>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable, ColorChooser;

    match fn {
        get_type => || gtk_sys::gtk_color_button_get_type(),
    }
}

impl ColorButton {
    pub fn new() -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_color_button_new()).unsafe_cast()
        }
    }

    pub fn new_with_rgba(rgba: &gdk::RGBA) -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_color_button_new_with_rgba(rgba.to_glib_none().0)).unsafe_cast()
        }
    }
}

impl Default for ColorButton {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_COLOR_BUTTON: Option<&ColorButton> = None;

pub trait ColorButtonExt: 'static {
    fn get_title(&self) -> Option<GString>;

    //fn set_color(&self, color: /*Ignored*/&gdk::Color);

    fn set_title(&self, title: &str);

    fn get_property_alpha(&self) -> u32;

    fn set_property_alpha(&self, alpha: u32);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_show_editor(&self) -> bool;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_show_editor(&self, show_editor: bool);

    fn connect_color_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_show_editor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ColorButton>> ColorButtonExt for O {
    fn get_title(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_color_button_get_title(self.as_ref().to_glib_none().0))
        }
    }

    //fn set_color(&self, color: /*Ignored*/&gdk::Color) {
    //    unsafe { TODO: call gtk_sys:gtk_color_button_set_color() }
    //}

    fn set_title(&self, title: &str) {
        unsafe {
            gtk_sys::gtk_color_button_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn get_property_alpha(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"alpha\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_alpha(&self, alpha: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"alpha\0".as_ptr() as *const _, Value::from(&alpha).to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_property_show_editor(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-editor\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_property_show_editor(&self, show_editor: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"show-editor\0".as_ptr() as *const _, Value::from(&show_editor).to_glib_none().0);
        }
    }

    fn connect_color_set<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"color-set\0".as_ptr() as *const _,
                Some(transmute(color_set_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::alpha\0".as_ptr() as *const _,
                Some(transmute(notify_alpha_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rgba\0".as_ptr() as *const _,
                Some(transmute(notify_rgba_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_show_editor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-editor\0".as_ptr() as *const _,
                Some(transmute(notify_show_editor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::title\0".as_ptr() as *const _,
                Some(transmute(notify_title_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-alpha\0".as_ptr() as *const _,
                Some(transmute(notify_use_alpha_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn color_set_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkColorButton, f: glib_sys::gpointer)
where P: IsA<ColorButton> {
    let f: &F = &*(f as *const F);
    f(&ColorButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_alpha_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkColorButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ColorButton> {
    let f: &F = &*(f as *const F);
    f(&ColorButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rgba_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkColorButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ColorButton> {
    let f: &F = &*(f as *const F);
    f(&ColorButton::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_show_editor_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkColorButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ColorButton> {
    let f: &F = &*(f as *const F);
    f(&ColorButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_title_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkColorButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ColorButton> {
    let f: &F = &*(f as *const F);
    f(&ColorButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_alpha_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkColorButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<ColorButton> {
    let f: &F = &*(f as *const F);
    f(&ColorButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for ColorButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ColorButton")
    }
}
