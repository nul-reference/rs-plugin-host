use std::alloc::GlobalAlloc;
use std::alloc::Layout;

#[no_mangle]
pub extern "C" fn shared_alloc(size: libc::size_t, align: libc::size_t) -> *mut u8 {
    let layout = Layout::from_size_align(size, align);
    if let Ok(layout) = layout {
        unsafe { std::alloc::System.alloc(layout) }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn shared_alloc_zeroed(size: libc::size_t, align: libc::size_t) -> *mut u8 {
    let layout = Layout::from_size_align(size, align);
    if let Ok(layout) = layout {
        unsafe { std::alloc::System.alloc_zeroed(layout) }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
/// # Safety
/// This function REQUIREs the CALLER to check that the pointer was allocated by this allocator,
/// and that the size and alignment are the same as were originally used!
pub unsafe extern "C" fn shared_dealloc(ptr: *mut u8, size: libc::size_t, align: libc::size_t) {
    let layout = Layout::from_size_align(size, align);
    match layout {
        Ok(layout) => std::alloc::System.dealloc(ptr, layout),
        Err(err) => panic!("Bad layout passed to deallocator: {err}"),
    }
}

/// # Safety
/// TODO
#[no_mangle]
pub unsafe extern "C" fn shared_realloc(
    ptr: *mut u8,
    size: libc::size_t,
    new_size: libc::size_t,
    align: libc::size_t,
) -> *mut u8 {
    let layout = Layout::from_size_align(size, align).unwrap();
    std::alloc::System.realloc(ptr, layout, new_size)
}
