#![warn(clippy::all, unused, rust_2018_idioms, rust_2018_compatibility)]

mod controls;
mod prefab;
mod states;

fn main() -> amethyst::Result<()> {
    use crate::controls::MyControlBundle;
    use crate::prefab::MyScenePrefab;
    use crate::states::MyLoadingState;
    use amethyst::assets::PrefabLoaderSystem;
    use amethyst::config::Config;
    use amethyst::core::TransformBundle;
    use amethyst::input::InputBundle;
    use amethyst::renderer::DrawPbmSeparate;
    use amethyst::ui::UiBundle;
    use amethyst::utils::auto_fov::AutoFovSystem;
    use amethyst::{Application, GameDataBuilder, LoggerConfig};
    use amethyst_gltf::GltfSceneLoaderSystem;

    let app_dir = amethyst::utils::application_root_dir()?;

    amethyst::start_logger(LoggerConfig::load_no_fallback(
        app_dir.join("assets/config/log.ron"),
    )?);

    let game_data = GameDataBuilder::new()
        .with(
            PrefabLoaderSystem::<MyScenePrefab>::default(),
            "my_scene_loader",
            &[],
        )
        .with(
            GltfSceneLoaderSystem::default(),
            "gltf_loader",
            &["my_scene_loader"],
        )
        .with(AutoFovSystem, "auto_fov", &["my_scene_loader"])
        .with_bundle(
            InputBundle::<String, String>::new()
                .with_bindings_from_file(app_dir.join("assets/config/input.ron"))?,
        )?
        .with_bundle(MyControlBundle::new())?
        .with_bundle(
            TransformBundle::new().with_dep(&["my_control_rotation", "my_control_movement"]),
        )?
        .with_bundle(UiBundle::<String, String>::new())?
        .with_basic_renderer(
            app_dir.join("assets/config/display.ron"),
            DrawPbmSeparate::new(),
            true,
        )?;

    let mut game =
        Application::build(app_dir.join("assets"), MyLoadingState::new())?.build(game_data)?;
    game.run();

    Ok(())
}
