use std::{
    collections::HashMap,
    ffi::{c_char, CStr, CString},
    mem::MaybeUninit,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

use once_cell::sync::Lazy;

static TAGLIST: Lazy<Arc<Mutex<HashMap<CString, usize>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
static NEXT_TAG_ID: AtomicUsize = AtomicUsize::new(0);

/// Register a new tag for a message type
///
/// This function allows for registration of a message tag. Tags are checked to ensure multiple
/// attempted registrations will return the same ID. This search _is_ case sensitive.
///
/// # Safety
/// The caller must ensure the pointer passed to this function is not null, and points
/// to a valid UTF-8 string.
///
/// # Returns
/// This function returns the ID of the registered tag as a size_t. If the tag was previously registered,
/// it will return the previous ID given.
pub unsafe extern "C" fn register_tag(tag_name: *const c_char) -> usize {
    let tag = ptr_to_cstr(tag_name);

    let mut taglist = TAGLIST.lock().unwrap();
    match taglist.get(&tag) {
        Some(id) => *id,
        None => {
            let tag_id = NEXT_TAG_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            taglist.insert(tag, tag_id);
            tag_id
        }
    }
}

/// Look up a presumably existing tag to find its ID.
///
/// Not it's likely more performant to store the ID returned on registration, however this function still
/// exists to enable looking up a tag ID using the string name.
///
/// # Safety
/// The caller must ensure the pointer passed to this function is not null, and points
/// to a valid UTF-8 string.
///
/// # Returns
/// This function with overwrite the provided tag_id field with the tag id if found. It will return true
/// on success, and false on failure.
pub unsafe extern "C" fn lookup_tag(
    tag_name: *const c_char,
    mut tag_id: MaybeUninit<usize>,
) -> bool {
    let tag = ptr_to_cstr(tag_name);
    match TAGLIST.lock().unwrap().get(&tag) {
        Some(id) => {
            tag_id.write(*id);
            true
        }
        None => false,
    }
}

/// Look up the name for a tag, given ID.
///
/// Absolutely avoid this function on the "hot path," as this function can be as slow as O(n). However,
/// for debugging code, this will provide the name of the tag ID. The pointer placed in the `tag` field is
/// a reference to the backing store, so it _must not_ be modified, otherwise UB abound!
///
/// # Safety
/// The caller must ensure the pointer passed to this function is not null, and points
/// to a valid UTF-8 string.
///
/// # Returns
/// This function with overwrite the provided `tag` field with a pointer to the name if found. It will return true
/// on success, and false on failure.
pub unsafe extern "C" fn get_tag_by_id(
    tag_id: usize,
    mut tag_name: MaybeUninit<*const c_char>,
) -> bool {
    match TAGLIST
        .lock()
        .unwrap()
        .iter()
        .find(|(_, tid)| **tid == tag_id)
    {
        Some((tag, _)) => {
            tag_name.write(tag.as_c_str().as_ptr());
            true
        }
        None => false,
    }
}

unsafe fn ptr_to_cstr(ptr: *const c_char) -> CString {
    CStr::from_ptr(ptr).to_owned()
}
