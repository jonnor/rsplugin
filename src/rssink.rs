//  Copyright (C) 2016 Sebastian Dröge <sebastian@centricular.com>
//                2016 Luis de Bethencourt <luisbg@osg.samsung.com>
//
//  This library is free software; you can redistribute it and/or
//  modify it under the terms of the GNU Library General Public
//  License as published by the Free Software Foundation; either
//  version 2 of the License, or (at your option) any later version.
//
//  This library is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//  Library General Public License for more details.
//
//  You should have received a copy of the GNU Library General Public
//  License along with this library; if not, write to the
//  Free Software Foundation, Inc., 51 Franklin St, Fifth Floor,
//  Boston, MA 02110-1301, USA.

use libc::c_char;
use std::ffi::{CStr, CString};
use std::slice;
use std::ptr;

use utils::*;

pub trait Sink: Sync + Send {
    fn set_uri(&mut self, uri_str: Option<&str>) -> bool;
    fn get_uri(&self) -> Option<String>;
    fn start(&mut self) -> bool;
    fn stop(&mut self) -> bool;
    fn render(&mut self, data: &[u8]) -> GstFlowReturn;
}

#[no_mangle]
pub extern "C" fn sink_drop(ptr: *mut Box<Sink>) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn sink_set_uri(ptr: *mut Box<Sink>, uri_ptr: *const c_char) -> GBoolean{
    let source: &mut Box<Sink> = unsafe { &mut *ptr };

    if uri_ptr.is_null() {
        GBoolean::from_bool(source.set_uri(None))
    } else {
        let uri = unsafe { CStr::from_ptr(uri_ptr) };
        GBoolean::from_bool(source.set_uri(Some(uri.to_str().unwrap())))
    }
}

#[no_mangle]
pub extern "C" fn sink_get_uri(ptr: *const Box<Sink>) -> *mut c_char {
    let source: &Box<Sink> = unsafe { &*ptr };

    match source.get_uri() {
        Some(uri) =>
            CString::new(uri.into_bytes()).unwrap().into_raw(),
        None =>
            ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn sink_render(ptr: *mut Box<Sink>, data_ptr: *const u8, data_len: usize) -> GstFlowReturn {
    let source: &mut Box<Sink> = unsafe { &mut *ptr };

    let data = unsafe { slice::from_raw_parts(data_ptr, data_len) };
    source.render(data)
}

#[no_mangle]
pub extern "C" fn sink_start(ptr: *mut Box<Sink>) -> GBoolean {
    let source: &mut Box<Sink> = unsafe { &mut *ptr };

    GBoolean::from_bool(source.start())
}

#[no_mangle]
pub extern "C" fn sink_stop(ptr: *mut Box<Sink>) -> GBoolean {
    let source: &mut Box<Sink> = unsafe { &mut *ptr };

    GBoolean::from_bool(source.stop())
}
