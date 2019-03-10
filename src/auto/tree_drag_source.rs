// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use SelectionData;
use TreePath;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct TreeDragSource(Interface<gtk_sys::GtkTreeDragSource>);

    match fn {
        get_type => || gtk_sys::gtk_tree_drag_source_get_type(),
    }
}

pub const NONE_TREE_DRAG_SOURCE: Option<&TreeDragSource> = None;

pub trait TreeDragSourceExt: 'static {
    fn drag_data_delete(&self, path: &mut TreePath) -> bool;

    fn drag_data_get(&self, path: &mut TreePath, selection_data: &mut SelectionData) -> bool;

    fn row_draggable(&self, path: &mut TreePath) -> bool;
}

impl<O: IsA<TreeDragSource>> TreeDragSourceExt for O {
    fn drag_data_delete(&self, path: &mut TreePath) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_drag_source_drag_data_delete(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0))
        }
    }

    fn drag_data_get(&self, path: &mut TreePath, selection_data: &mut SelectionData) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_drag_source_drag_data_get(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0, selection_data.to_glib_none_mut().0))
        }
    }

    fn row_draggable(&self, path: &mut TreePath) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_drag_source_row_draggable(self.as_ref().to_glib_none().0, path.to_glib_none_mut().0))
        }
    }
}

impl fmt::Display for TreeDragSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeDragSource")
    }
}
