use host::plugin::{register_tag, Info, Plugin};

struct IncrementPlugin {
    command_tag: usize,
    count_change_tag: usize,
    host_vtable: host::HostVTable,
}

static PLUGIN: std::sync::OnceLock<IncrementPlugin> = std::sync::OnceLock::new();

impl Plugin for IncrementPlugin {
    #[no_mangle]
    extern "C" fn initialize(host_vtable: host::HostVTable) -> Info {
        let command_tag = register_tag(host_vtable, "command");
        let count_change_tag = register_tag(host_vtable, "count_change");

        debug_assert!(PLUGIN
            .set(IncrementPlugin {
                command_tag,
                count_change_tag,
                host_vtable
            })
            .is_ok());

        Info::new("increment", vec![command_tag, count_change_tag])
    }
}
