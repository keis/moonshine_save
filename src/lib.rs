#![doc = include_str!("../README.md")]

pub mod load;
pub mod save;

mod utils;

/// Common elements for saving/loading world state.
pub mod prelude {
    pub use crate::load::{
        load_from_file, load_from_file_on_event, load_from_file_on_request, LoadError,
        LoadFromFileRequest, LoadPlugin, LoadSet, Loaded, Unload,
    };
    pub use crate::save::{
        save, save_all, save_all_with, save_default, save_default_with, save_with, Save, SaveError,
        SaveFilter, SaveIntoFileRequest, SavePlugin, SaveSet, Saved,
    };

    // TODO: Remove these.
    #[allow(deprecated)]
    pub use crate::save::{save_into_file, save_into_file_on_event, save_into_file_on_request};

    pub use bevy_ecs::{
        entity::{EntityMapper, MapEntities},
        reflect::ReflectMapEntities,
    };
}
