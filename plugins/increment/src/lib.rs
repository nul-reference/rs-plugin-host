#[no_mangle]
extern "C" fn initialize() -> api::PluginInfo {
    let name = "increment";
    let provides = vec!["count_change"];
    api::PluginInfo::new(name, provides)
}
