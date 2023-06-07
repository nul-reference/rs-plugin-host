// use dlopen2::wrapper::{Container, WrapperApi};

pub(crate) mod allocator;
pub(crate) mod tag_registry;

fn main() {
    let exec_path = std::env::current_exe().unwrap();
    let plugin_path = exec_path.parent().unwrap();

    println!("Loading plugins from {plugin_path:?}");
    // let increment_path = lib_path.join(format!(
    //     "{}increment.{}",
    //     dlopen2::utils::PLATFORM_FILE_PREFIX,
    //     dlopen2::utils::PLATFORM_FILE_EXTENSION
    // ));
    // let increment_lib: Container<Api> =
    //     unsafe { Container::load(increment_path).expect("library loaded") };
    // let increment_info = increment_lib.initialize();
    // std::mem::drop(increment_lib);
    // println!("Loaded command: {}", increment_info.name());
}
