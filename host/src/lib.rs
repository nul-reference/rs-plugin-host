pub mod allocator;
pub mod plugin;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct HostVTable {
    pub register_tag: unsafe extern "C" fn(tag_name: *const std::ffi::c_char) -> usize,
    pub lookup_tag: unsafe extern "C" fn(
        tag_name: *const std::ffi::c_char,
        tag_id: std::mem::MaybeUninit<usize>,
    ) -> bool,
    pub get_tag_by_id: unsafe extern "C" fn(
        tag_id: usize,
        tag_name: std::mem::MaybeUninit<*const std::ffi::c_char>,
    ) -> bool,
}
