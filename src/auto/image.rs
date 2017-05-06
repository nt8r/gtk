// This file was generated by gir (32b0f11) from gir-files (71d73f0)
// DO NOT EDIT

use ImageType;
use Misc;
use Widget;
#[cfg(feature = "v3_10")]
use cairo;
use ffi;
use gdk_pixbuf;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std;

glib_wrapper! {
    pub struct Image(Object<ffi::GtkImage>): Misc, Widget;

    match fn {
        get_type => || ffi::gtk_image_get_type(),
    }
}

impl Image {
    pub fn new() -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new()).downcast_unchecked()
        }
    }

    pub fn new_from_animation<P: IsA<gdk_pixbuf::PixbufAnimation>>(animation: &P) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_animation(animation.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_file<P: AsRef<std::path::Path>>(filename: P) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_file(filename.as_ref().to_glib_none().0)).downcast_unchecked()
        }
    }

    //pub fn new_from_gicon<P: IsA</*Ignored*/gio::Icon>>(icon: &P, size: i32) -> Image {
    //    unsafe { TODO: call ffi::gtk_image_new_from_gicon() }
    //}

    pub fn new_from_icon_name(icon_name: &str, size: i32) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_icon_name(icon_name.to_glib_none().0, size)).downcast_unchecked()
        }
    }

    //pub fn new_from_icon_set(icon_set: /*Ignored*/&IconSet, size: i32) -> Image {
    //    unsafe { TODO: call ffi::gtk_image_new_from_icon_set() }
    //}

    pub fn new_from_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(pixbuf: P) -> Image {
        assert_initialized_main_thread!();
        let pixbuf = pixbuf.into();
        let pixbuf = pixbuf.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_pixbuf(pixbuf.0)).downcast_unchecked()
        }
    }

    pub fn new_from_resource(resource_path: &str) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_resource(resource_path.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_from_stock(stock_id: &str, size: i32) -> Image {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_stock(stock_id.to_glib_none().0, size)).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn new_from_surface<'a, P: Into<Option<&'a cairo::Surface>>>(surface: P) -> Image {
        assert_initialized_main_thread!();
        let surface = surface.into();
        unsafe {
            Widget::from_glib_none(ffi::gtk_image_new_from_surface(mut_override(surface.to_glib_none().0))).downcast_unchecked()
        }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::gtk_image_clear(self.to_glib_none().0);
        }
    }

    pub fn get_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation> {
        unsafe {
            from_glib_none(ffi::gtk_image_get_animation(self.to_glib_none().0))
        }
    }

    //pub fn get_gicon(&self, gicon: /*Ignored*/gio::Icon) -> i32 {
    //    unsafe { TODO: call ffi::gtk_image_get_gicon() }
    //}

    //pub fn get_icon_name(&self, icon_name: /*Unimplemented*/String) -> i32 {
    //    unsafe { TODO: call ffi::gtk_image_get_icon_name() }
    //}

    //pub fn get_icon_set(&self, icon_set: /*Ignored*/IconSet) -> i32 {
    //    unsafe { TODO: call ffi::gtk_image_get_icon_set() }
    //}

    pub fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_image_get_pixbuf(self.to_glib_none().0))
        }
    }

    pub fn get_pixel_size(&self) -> i32 {
        unsafe {
            ffi::gtk_image_get_pixel_size(self.to_glib_none().0)
        }
    }

    //pub fn get_stock(&self, stock_id: /*Unimplemented*/String) -> i32 {
    //    unsafe { TODO: call ffi::gtk_image_get_stock() }
    //}

    pub fn get_storage_type(&self) -> ImageType {
        unsafe {
            from_glib(ffi::gtk_image_get_storage_type(self.to_glib_none().0))
        }
    }

    pub fn set_from_animation<P: IsA<gdk_pixbuf::PixbufAnimation>>(&self, animation: &P) {
        unsafe {
            ffi::gtk_image_set_from_animation(self.to_glib_none().0, animation.to_glib_none().0);
        }
    }

    pub fn set_from_file<P: AsRef<std::path::Path>>(&self, filename: P) {
        unsafe {
            ffi::gtk_image_set_from_file(self.to_glib_none().0, filename.as_ref().to_glib_none().0);
        }
    }

    //pub fn set_from_gicon<P: IsA</*Ignored*/gio::Icon>>(&self, icon: &P, size: i32) {
    //    unsafe { TODO: call ffi::gtk_image_set_from_gicon() }
    //}

    pub fn set_from_icon_name(&self, icon_name: &str, size: i32) {
        unsafe {
            ffi::gtk_image_set_from_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0, size);
        }
    }

    //pub fn set_from_icon_set(&self, icon_set: /*Ignored*/&IconSet, size: i32) {
    //    unsafe { TODO: call ffi::gtk_image_set_from_icon_set() }
    //}

    pub fn set_from_pixbuf<'a, P: Into<Option<&'a gdk_pixbuf::Pixbuf>>>(&self, pixbuf: P) {
        let pixbuf = pixbuf.into();
        let pixbuf = pixbuf.to_glib_none();
        unsafe {
            ffi::gtk_image_set_from_pixbuf(self.to_glib_none().0, pixbuf.0);
        }
    }

    pub fn set_from_resource<'a, P: Into<Option<&'a str>>>(&self, resource_path: P) {
        let resource_path = resource_path.into();
        let resource_path = resource_path.to_glib_none();
        unsafe {
            ffi::gtk_image_set_from_resource(self.to_glib_none().0, resource_path.0);
        }
    }

    pub fn set_from_stock(&self, stock_id: &str, size: i32) {
        unsafe {
            ffi::gtk_image_set_from_stock(self.to_glib_none().0, stock_id.to_glib_none().0, size);
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn set_from_surface(&self, surface: &cairo::Surface) {
        unsafe {
            ffi::gtk_image_set_from_surface(self.to_glib_none().0, mut_override(surface.to_glib_none().0));
        }
    }

    pub fn set_pixel_size(&self, pixel_size: i32) {
        unsafe {
            ffi::gtk_image_set_pixel_size(self.to_glib_none().0, pixel_size);
        }
    }

    pub fn get_property_file(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "file".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_file(&self, file: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "file".to_glib_none().0, Value::from(file).to_glib_none().0);
        }
    }

    //pub fn set_property_gicon(&self, gicon: /*Ignored*/Option<&gio::Icon>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "gicon".to_glib_none().0, Value::from(gicon).to_glib_none().0);
    //    }
    //}

    pub fn set_property_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-name".to_glib_none().0, Value::from(icon_name).to_glib_none().0);
        }
    }

    //pub fn set_property_icon_set(&self, icon_set: /*Ignored*/Option<&IconSet>) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-set".to_glib_none().0, Value::from(icon_set).to_glib_none().0);
    //    }
    //}

    pub fn get_property_icon_size(&self) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon-size".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_icon_size(&self, icon_size: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-size".to_glib_none().0, Value::from(&icon_size).to_glib_none().0);
        }
    }

    pub fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pixbuf".to_glib_none().0, Value::from(pixbuf).to_glib_none().0);
        }
    }

    pub fn get_property_pixbuf_animation(&self) -> Option<gdk_pixbuf::PixbufAnimation> {
        let mut value = Value::from(None::<&gdk_pixbuf::PixbufAnimation>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pixbuf-animation".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_pixbuf_animation(&self, pixbuf_animation: Option<&gdk_pixbuf::PixbufAnimation>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pixbuf-animation".to_glib_none().0, Value::from(pixbuf_animation).to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_property_resource(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "resource".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    #[cfg(feature = "v3_8")]
    pub fn set_property_resource(&self, resource: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "resource".to_glib_none().0, Value::from(resource).to_glib_none().0);
        }
    }

    pub fn set_property_stock(&self, stock: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stock".to_glib_none().0, Value::from(stock).to_glib_none().0);
        }
    }

    pub fn get_property_use_fallback(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "use-fallback".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    pub fn set_property_use_fallback(&self, use_fallback: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "use-fallback".to_glib_none().0, Value::from(&use_fallback).to_glib_none().0);
        }
    }
}
