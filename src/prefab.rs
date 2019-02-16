#![allow(clippy::useless_let_if_seq)]

use crate::controls::{MyFlyMovement, MyFreeRotation};
use amethyst::assets::{AssetPrefab, PrefabData, ProgressCounter};
use amethyst::core::Transform;
use amethyst::derive::PrefabData;
use amethyst::ecs::Entity;
use amethyst::error::Error;
use amethyst::renderer::{CameraPrefab, LightPrefab};
use amethyst::utils::auto_fov::AutoFov;
use amethyst_gltf::{GltfSceneAsset, GltfSceneFormat};
use serde::{Deserialize, Serialize};

#[derive(Default, PrefabData, Deserialize, Serialize)]
#[serde(default)]
pub struct MyScenePrefab {
    transform: Option<Transform>,

    camera: Option<CameraPrefab>,
    auto_fov: Option<AutoFov>,

    light: Option<LightPrefab>,

    fly_movement: Option<MyFlyMovement>,
    free_rotation: Option<MyFreeRotation>,

    gltf: Option<AssetPrefab<GltfSceneAsset, GltfSceneFormat>>,
}
