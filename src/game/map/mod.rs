pub mod config;
pub mod loader;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

pub use config::*;
pub use loader::*;

pub struct MapSystemPlugin;

impl Plugin for MapSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<MapConfig>::new(&["map.ron"]))
            .add_plugins(MapLoaderPlugin);
    }
}
