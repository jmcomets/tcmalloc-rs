#![feature(allocator_api)]
// #![feature(global_allocator)]

use std::alloc::{GlobalAlloc, Layout, Opaque};

use std::os::raw::c_void;

#[link(name = "tcmalloc")]
extern "C" {
    pub fn tc_memalign(alignment: usize, size: usize) -> *mut c_void;
    pub fn tc_free(ptr: *mut c_void);
    // buggy, but why is that?
    // pub fn tc_free_sized(ptr: *mut c_void, size: usize);
}

pub struct TCMalloc;

unsafe impl GlobalAlloc for TCMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
        tc_memalign(layout.align(), layout.size()) as *mut Opaque
    }

    unsafe fn dealloc(&self, ptr: *mut Opaque, _layout: Layout) {
        tc_free(ptr as *mut c_void);
        // tc_free_sized(ptr as *mut c_void, layout.size());
    }
}

