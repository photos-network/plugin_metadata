//! Photos.network plugin implementation to extract metadata of an image based on the filename or 
//! its built-in exif information.
//!
use abi_stable::{
    export_root_module,
    external_types::crossbeam_channel::RSender,
    prefix_type::PrefixTypeTrait,
    sabi_extern_fn,
    sabi_trait::prelude::TD_Opaque,
    std_types::{ROk, RResult, RString},
};

use photos_network_plugin::{PluginCommand, PluginId};
use photos_network_plugin::Error;
use photos_network_plugin::Plugin;
use photos_network_plugin::Plugin_TO;
use photos_network_plugin::PluginType;
use photos_network_plugin::PluginFactory_Ref;
use photos_network_plugin::PluginFactory;

#[export_root_module]
fn instantiate_root_module() -> PluginFactory_Ref {
    PluginFactory { new }.leak_into_prefix()
}

/// create a new instance of the plugin by the plugin factory
#[sabi_extern_fn]
pub fn new(_sender: RSender<PluginCommand>, plugin_id: PluginId) -> RResult<PluginType, Error> {
    let this = MetadataPlugin { plugin_id };
    ROk(Plugin_TO::from_value(this, TD_Opaque))
}

struct MetadataPlugin {
    plugin_id: RString
}

impl Plugin for MetadataPlugin {
    fn on_core_init(&self) -> RResult<RString, Error> {
        ROk(RString::from(""))
    }

    fn on_core_started(&self) -> RResult<RString, Error> {
        ROk(RString::from(""))
    }
}
