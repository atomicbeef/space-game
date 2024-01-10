use bevy::prelude::*;

use crate::app_setup::AssetInitialization;

#[derive(Resource)]
pub struct SkyboxHandle(pub Handle<Image>);

fn init_skybox(asset_server: Res<AssetServer>, mut skybox_handle: ResMut<SkyboxHandle>) {
    skybox_handle.0 = asset_server.load("skybox/skybox.ktx2");
}

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SkyboxHandle(Handle::default()))
            .add_systems(Startup, init_skybox.in_set(AssetInitialization));
    }
}
