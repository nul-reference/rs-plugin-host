use dlopen2::wrapper::Container;

pub mod tag_registry;

fn main() {
    let host_vtable = host::HostVTable {
        get_tag_by_id: tag_registry::get_tag_by_id,
        lookup_tag: tag_registry::lookup_tag,
        register_tag: tag_registry::register_tag,
    };

    let exec_path = std::env::current_exe().unwrap();
    let plugin_path = exec_path.parent().unwrap();

    let mut plugins: Vec<Container<host::plugin::PluginApi>> = Vec::new();
    // TODO: Make this smarter once closer to "release"
    println!("Loading plugins from {plugin_path:?}");
    let plugin_list = ["increment"];
    for plugin in plugin_list {
        let path = plugin_path.join(dlopen2::utils::platform_file_name(plugin));
        let lib = unsafe { Container::load(path).expect("load {path:?}") };
        plugins.push(lib);
    }
    println!("Plugins loaded. Running initialization.");

    for plugin in plugins {
        let info = unsafe { plugin.init(host_vtable) };
        println!("Initialized plugin \"{}\"", info.name());
    }
}
