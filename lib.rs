#[crate_id="boehm#0.1"];
#[crate_type="lib"];
#[feature(globs)];

use std::{libc, mem};
use std::unstable::intrinsics;

#[allow(dead_code)]
pub mod ffi;

/// Initialise the GC. This should be called before using any other
/// functions and on the main thread for maximum portability (some
/// platforms don't require this to be called at all).
///
/// FIXME: initialise automagically somehow
/// FIXME: this should be doing the full equivalent of the GC_INIT()
/// macro.
pub fn init() {
    unsafe { ffi::GC_init(); }
}

/// Number of bytes in the garbage collection heap.
pub fn heap_size() -> uint {
    unsafe { ffi::GC_get_heap_size() as uint }
}

/// Force a garbage collection.
pub fn collect() {
    unsafe { ffi::GC_gcollect(); }
}

/// A garbage collected pointer.
#[no_send]
#[deriving(Clone)]
pub struct Gc<T> {
    priv ptr: *mut T
}

impl<T: 'static> Gc<T> {
    pub fn new(value: T) -> Gc<T> {
        unsafe {
            let p = ffi::GC_malloc(mem::size_of::<T>() as libc::size_t) as *mut T;
            if p.is_null() {
                fail!("Could not allocate")
            }
            intrinsics::move_val_init(&mut *p, value);
            Gc { ptr: p }
        }
    }

    pub fn borrow<'r>(&'r self) -> &'r T {
        unsafe {
            &*self.ptr
        }
    }
}
