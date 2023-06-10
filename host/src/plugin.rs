use dlopen2::wrapper::WrapperApi;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Info {
    pub name: *const std::ffi::c_char,
    pub subscribe_tag_count: usize,
    pub subscribe_tags: *const usize,
}

impl Info {
    pub fn new(name: &str, subscribe_tags: Vec<usize>) -> Self {
        let name = std::ffi::CString::new(name).unwrap();
        // Note: This will indeed leak memory. However, as plugins are never unloaded,
        // it shouldn't be a big issue.
        let name = Box::leak(Box::new(name)).as_ptr();

        let subscribe_tag_count = subscribe_tags.len();
        let subscribe_tags = Box::leak(Box::new(subscribe_tags)).as_ptr();

        Self {
            name,
            subscribe_tag_count,
            subscribe_tags,
        }
    }

    pub fn name(&self) -> &str {
        unsafe { std::ffi::CStr::from_ptr(self.name) }
            .to_str()
            .unwrap()
    }
}

pub trait Plugin {
    extern "C" fn initialize(host_vtable: crate::HostVTable) -> Info;
}

pub fn register_tag(host_vtable: crate::HostVTable, tag_name: &str) -> usize {
    let tag_cname = std::ffi::CString::new(tag_name).unwrap();
    unsafe { (host_vtable.register_tag)(tag_cname.as_ptr()) }
}

#[derive(dlopen2::wrapper::WrapperApi)]
pub struct PluginApi {
    init: unsafe extern "C" fn(host_vtable: crate::HostVTable) -> Info,
}
