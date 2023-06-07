use std::alloc::Layout;

#[no_mangle]
pub extern "C" fn host_alloc(size: usize, align: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, align);
    if let Ok(layout) = layout {
        unsafe { std::alloc::alloc(layout) }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn host_alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, align);
    if let Ok(layout) = layout {
        unsafe { std::alloc::alloc_zeroed(layout) }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
/// # Safety
/// This function REQUIREs the CALLER to check that the pointer was allocated by this allocator,
/// and that the size and alignment are the same as were originally used!
pub unsafe extern "C" fn host_dealloc(ptr: *mut u8, size: usize, align: usize) {
    let layout = Layout::from_size_align(size, align);
    match layout {
        Ok(layout) => std::alloc::dealloc(ptr, layout),
        Err(err) => panic!("Bad layout passed to deallocator: {err}"),
    }
}

/// # Safety
/// TODO
#[no_mangle]
pub unsafe extern "C" fn host_realloc(
    ptr: *mut u8,
    size: usize,
    new_size: usize,
    align: usize,
) -> *mut u8 {
    let layout = Layout::from_size_align(size, align).unwrap();
    std::alloc::realloc(ptr, layout, new_size)
}
