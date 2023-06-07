use std::{
    collections::HashMap,
    ffi::{c_char, CStr, CString},
    mem::MaybeUninit,
};

use once_cell::sync::Lazy;

#[derive(Default)]
struct RegistryState {
    map: HashMap<usize, CString>,
    next_id: usize,
}

static mut REGISTRY: Lazy<RegistryState> = Lazy::new(RegistryState::default);

#[no_mangle]
pub unsafe extern "C" fn register_tag(tag_name: *const c_char) -> usize {
    let tag = CStr::from_ptr(tag_name).to_owned();
    let mut id: MaybeUninit<usize> = MaybeUninit::uninit();

    if lookup_tag(tag_name, id) {
        id.assume_init()
    } else {
        id.write(REGISTRY.next_id);
        REGISTRY.next_id += 1;
        REGISTRY.map.insert(id.assume_init(), tag);
        id.assume_init()
    }
}

#[no_mangle]
pub unsafe extern "C" fn lookup_tag(
    tag_name: *const c_char,
    mut tag_id: MaybeUninit<usize>,
) -> bool {
    let tag = CStr::from_ptr(tag_name).to_owned();
    if let Some((k, _)) = REGISTRY.map.iter().find(|(_, v)| **v == tag) {
        tag_id.write(*k);
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_tag_by_id(
    tag_id: usize,
    mut tag_name: MaybeUninit<*const c_char>,
) -> bool {
    match REGISTRY.map.get(&tag_id) {
        Some(tag) => {
            tag_name.write(tag.as_c_str().as_ptr());
            true
        }
        None => false,
    }
}
