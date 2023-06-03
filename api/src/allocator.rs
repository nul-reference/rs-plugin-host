use std::alloc::GlobalAlloc;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        shared_alloc(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        shared_dealloc(
            ptr,
            layout.size() as libc::size_t,
            layout.align() as libc::size_t,
        )
    }

    unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
        shared_alloc_zeroed(
            layout.size() as libc::size_t,
            layout.align() as libc::size_t,
        )
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: std::alloc::Layout, new_size: usize) -> *mut u8 {
        shared_realloc(
            ptr,
            layout.size() as libc::size_t,
            new_size,
            layout.align() as libc::size_t,
        )
    }
}

#[link(name = "allocator", kind = "dylib")]
extern "C" {
    fn shared_alloc(size: libc::size_t, align: libc::size_t) -> *mut u8;
    fn shared_alloc_zeroed(size: libc::size_t, align: libc::size_t) -> *mut u8;
    fn shared_dealloc(ptr: *mut u8, size: libc::size_t, align: libc::size_t);
    fn shared_realloc(
        ptr: *mut u8,
        size: libc::size_t,
        new_size: libc::size_t,
        align: libc::size_t,
    ) -> *mut u8;
}
