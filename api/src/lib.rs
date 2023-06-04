use std::ffi::{c_char, CStr, CString};

pub mod allocator;

#[derive(Debug)]
#[repr(C)]
pub struct PluginInfo {
    name: *const c_char,
    provides: *const *const c_char,
}

impl PluginInfo {
    pub fn new(name: &str, provides: Vec<&str>) -> Self {
        let cstring: CString = CString::new(name).unwrap();
        let cstring: &'static mut CString = Box::leak(Box::new(cstring));
        let provides: Vec<*const c_char> = provides
            .iter()
            .map(|s| {
                let cs = CString::new(*s).unwrap();
                let cs = Box::leak(Box::new(cs));
                cs.as_ptr()
            })
            .collect();
        Self {
            name: cstring.as_ptr(),
            provides: provides.as_ptr(),
        }
    }

    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name) }.to_str().unwrap()
    }
}
