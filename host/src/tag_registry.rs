use std::{
    ffi::{c_char, CStr, CString},
    mem::MaybeUninit,
};

static mut REGISTRY: once_cell::sync::Lazy<RegistryState> =
    once_cell::sync::Lazy::new(RegistryState::default);

#[derive(Default)]
struct RegistryState {
    string_map: std::collections::HashMap<CString, usize>,
    id_vec: Vec<*const c_char>,
}

#[no_mangle]
pub unsafe extern "C" fn register_tag(tag_name: *const c_char) -> usize {
    let tag = CStr::from_ptr(tag_name).to_owned();
    if let Some(id) = REGISTRY.string_map.get(&tag) {
        *id
    } else {
        let id = REGISTRY.id_vec.len();
        REGISTRY.id_vec.push(tag.as_ptr());
        REGISTRY.string_map.insert(tag, id);
        id
    }
}

#[no_mangle]
pub unsafe extern "C" fn lookup_tag(
    tag_name: *const c_char,
    mut tag_id: MaybeUninit<usize>,
) -> bool {
    let tag = CStr::from_ptr(tag_name).to_owned();
    if let Some(id) = REGISTRY.string_map.get(&tag) {
        tag_id.write(*id);
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
    if let Some(tag) = REGISTRY.id_vec.get(tag_id) {
        tag_name.write(*tag);
        true
    } else {
        false
    }
}
