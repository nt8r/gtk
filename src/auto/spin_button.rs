// This file was generated by gir (38add47) from gir-files (469db10)
// DO NOT EDIT

use Adjustment;
use Buildable;
use CellEditable;
use Editable;
use Entry;
use Orientable;
use SpinButtonUpdatePolicy;
use SpinType;
use Widget;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SpinButton(Object<ffi::GtkSpinButton, ffi::GtkSpinButtonClass>): Entry, Widget, Buildable, CellEditable, Editable, Orientable;

    match fn {
        get_type => || ffi::gtk_spin_button_get_type(),
    }
}

impl SpinButton {
    pub fn new<'a, P: Into<Option<&'a Adjustment>>>(adjustment: P, climb_rate: f64, digits: u32) -> SpinButton {
        assert_initialized_main_thread!();
        let adjustment = adjustment.into();
        let adjustment = adjustment.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_spin_button_new(adjustment.0, climb_rate, digits)).downcast_unchecked()
        }
    }

    pub fn new_with_range(min: f64, max: f64, step: f64) -> SpinButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_spin_button_new_with_range(min, max, step)).downcast_unchecked()
        }
    }
}

pub trait SpinButtonExt {
    fn configure<'a, P: Into<Option<&'a Adjustment>>>(&self, adjustment: P, climb_rate: f64, digits: u32);

    fn get_adjustment(&self) -> Adjustment;

    fn get_digits(&self) -> u32;

    fn get_increments(&self) -> (f64, f64);

    fn get_numeric(&self) -> bool;

    fn get_range(&self) -> (f64, f64);

    fn get_snap_to_ticks(&self) -> bool;

    fn get_update_policy(&self) -> SpinButtonUpdatePolicy;

    fn get_value(&self) -> f64;

    fn get_value_as_int(&self) -> i32;

    fn get_wrap(&self) -> bool;

    fn set_adjustment(&self, adjustment: &Adjustment);

    fn set_digits(&self, digits: u32);

    fn set_increments(&self, step: f64, page: f64);

    fn set_numeric(&self, numeric: bool);

    fn set_range(&self, min: f64, max: f64);

    fn set_snap_to_ticks(&self, snap_to_ticks: bool);

    fn set_update_policy(&self, policy: SpinButtonUpdatePolicy);

    fn set_value(&self, value: f64);

    fn set_wrap(&self, wrap: bool);

    fn spin(&self, direction: SpinType, increment: f64);

    fn update(&self);

    fn get_property_climb_rate(&self) -> f64;

    fn set_property_climb_rate(&self, climb_rate: f64);

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_numeric_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_snap_to_ticks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_update_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SpinButton> + IsA<glib::object::Object>> SpinButtonExt for O {
    fn configure<'a, P: Into<Option<&'a Adjustment>>>(&self, adjustment: P, climb_rate: f64, digits: u32) {
        let adjustment = adjustment.into();
        let adjustment = adjustment.to_glib_none();
        unsafe {
            ffi::gtk_spin_button_configure(self.to_glib_none().0, adjustment.0, climb_rate, digits);
        }
    }

    fn get_adjustment(&self) -> Adjustment {
        unsafe {
            from_glib_none(ffi::gtk_spin_button_get_adjustment(self.to_glib_none().0))
        }
    }

    fn get_digits(&self) -> u32 {
        unsafe {
            ffi::gtk_spin_button_get_digits(self.to_glib_none().0)
        }
    }

    fn get_increments(&self) -> (f64, f64) {
        unsafe {
            let mut step = mem::uninitialized();
            let mut page = mem::uninitialized();
            ffi::gtk_spin_button_get_increments(self.to_glib_none().0, &mut step, &mut page);
            (step, page)
        }
    }

    fn get_numeric(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_numeric(self.to_glib_none().0))
        }
    }

    fn get_range(&self) -> (f64, f64) {
        unsafe {
            let mut min = mem::uninitialized();
            let mut max = mem::uninitialized();
            ffi::gtk_spin_button_get_range(self.to_glib_none().0, &mut min, &mut max);
            (min, max)
        }
    }

    fn get_snap_to_ticks(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_snap_to_ticks(self.to_glib_none().0))
        }
    }

    fn get_update_policy(&self) -> SpinButtonUpdatePolicy {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_update_policy(self.to_glib_none().0))
        }
    }

    fn get_value(&self) -> f64 {
        unsafe {
            ffi::gtk_spin_button_get_value(self.to_glib_none().0)
        }
    }

    fn get_value_as_int(&self) -> i32 {
        unsafe {
            ffi::gtk_spin_button_get_value_as_int(self.to_glib_none().0)
        }
    }

    fn get_wrap(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_spin_button_get_wrap(self.to_glib_none().0))
        }
    }

    fn set_adjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_spin_button_set_adjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    fn set_digits(&self, digits: u32) {
        unsafe {
            ffi::gtk_spin_button_set_digits(self.to_glib_none().0, digits);
        }
    }

    fn set_increments(&self, step: f64, page: f64) {
        unsafe {
            ffi::gtk_spin_button_set_increments(self.to_glib_none().0, step, page);
        }
    }

    fn set_numeric(&self, numeric: bool) {
        unsafe {
            ffi::gtk_spin_button_set_numeric(self.to_glib_none().0, numeric.to_glib());
        }
    }

    fn set_range(&self, min: f64, max: f64) {
        unsafe {
            ffi::gtk_spin_button_set_range(self.to_glib_none().0, min, max);
        }
    }

    fn set_snap_to_ticks(&self, snap_to_ticks: bool) {
        unsafe {
            ffi::gtk_spin_button_set_snap_to_ticks(self.to_glib_none().0, snap_to_ticks.to_glib());
        }
    }

    fn set_update_policy(&self, policy: SpinButtonUpdatePolicy) {
        unsafe {
            ffi::gtk_spin_button_set_update_policy(self.to_glib_none().0, policy.to_glib());
        }
    }

    fn set_value(&self, value: f64) {
        unsafe {
            ffi::gtk_spin_button_set_value(self.to_glib_none().0, value);
        }
    }

    fn set_wrap(&self, wrap: bool) {
        unsafe {
            ffi::gtk_spin_button_set_wrap(self.to_glib_none().0, wrap.to_glib());
        }
    }

    fn spin(&self, direction: SpinType, increment: f64) {
        unsafe {
            ffi::gtk_spin_button_spin(self.to_glib_none().0, direction.to_glib(), increment);
        }
    }

    fn update(&self) {
        unsafe {
            ffi::gtk_spin_button_update(self.to_glib_none().0);
        }
    }

    fn get_property_climb_rate(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "climb-rate".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_climb_rate(&self, climb_rate: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "climb-rate".to_glib_none().0, Value::from(&climb_rate).to_glib_none().0);
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::adjustment",
                transmute(notify_adjustment_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::climb-rate",
                transmute(notify_climb_rate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::digits",
                transmute(notify_digits_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_numeric_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::numeric",
                transmute(notify_numeric_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_snap_to_ticks_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::snap-to-ticks",
                transmute(notify_snap_to_ticks_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_update_policy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::update-policy",
                transmute(notify_update_policy_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::value",
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_wrap_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::wrap",
                transmute(notify_wrap_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_adjustment_trampoline<P>(this: *mut ffi::GtkSpinButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpinButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpinButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_climb_rate_trampoline<P>(this: *mut ffi::GtkSpinButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpinButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpinButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_digits_trampoline<P>(this: *mut ffi::GtkSpinButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpinButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpinButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_numeric_trampoline<P>(this: *mut ffi::GtkSpinButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpinButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpinButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_snap_to_ticks_trampoline<P>(this: *mut ffi::GtkSpinButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpinButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpinButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_update_policy_trampoline<P>(this: *mut ffi::GtkSpinButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpinButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpinButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::GtkSpinButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpinButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpinButton::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_wrap_trampoline<P>(this: *mut ffi::GtkSpinButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SpinButton> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SpinButton::from_glib_borrow(this).downcast_unchecked())
}
