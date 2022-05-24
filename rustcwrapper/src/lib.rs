
pub mod wrapper {
    pub mod myrustlib;
    pub mod plugin_wrapper;
}


// Import the Rust struct to wrap.
extern crate rustplugin;
use rustplugin::myrustlib::my_rust_struct::MyRustStruct;

use wrapper::plugin_wrapper::PluginWrapper;
use std::sync::Arc;

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut PluginWrapper {
    let plugin = Box::new(MyRustStruct::new());
    let plugin = PluginWrapper { plugin };
    Arc::into_raw(Arc::new(plugin)) as *mut PluginWrapper
}