use crate::prefab::MyScenePrefab;
use amethyst::assets::{Completion, Handle, Prefab, ProgressCounter};
use amethyst::prelude::{GameData, SimpleState, SimpleTrans, StateData, StateEvent, Trans};
use amethyst::ui::UiPrefab;
use derivative::Derivative;

/// Loading state that shows the first
#[derive(Derivative)]
#[derivative(Default(new = "true"))]
pub struct MyLoadingState {
    progress: ProgressCounter,
    scene: Option<Handle<Prefab<MyScenePrefab>>>,
    info_ui: Option<Handle<UiPrefab>>,
}

impl SimpleState for MyLoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        use amethyst::assets::{PrefabLoader, RonFormat};
        use amethyst::ui::{UiCreator, UiLoader};

        // show "loading" text while loading assets
        data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/loading.ron", &mut self.progress);
        });

        // load the scene
        self.scene = Some(data.world.exec(|loader: PrefabLoader<'_, MyScenePrefab>| {
            loader.load("prefab/scene_prefab.ron", RonFormat, (), &mut self.progress)
        }));

        // load the info ui
        self.info_ui = Some(
            data.world
                .exec(|loader: UiLoader<'_>| loader.load("ui/info.ron", &mut self.progress)),
        );
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        use amethyst::ui::UiFinder;

        // the "loading" text should be removed
        if let Some(entity) = data
            .world
            .exec(|finder: UiFinder<'_>| finder.find("loading_text"))
        {
            let _ = data.world.delete_entity(entity);
        }
    }

    fn update(&mut self, _: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Complete => {
                log::info!("Moving to playing state");
                Trans::Switch(Box::new(MyPlayState::new(
                    self.scene.take().unwrap(),
                    self.info_ui.take().unwrap(),
                )))
            }
            Completion::Failed => {
                log::error!("Failed to load assets");
                Trans::Quit
            }
            Completion::Loading => Trans::None,
        }
    }
}

/// The main playing state
struct MyPlayState {
    scene: Handle<Prefab<MyScenePrefab>>,
    info_ui: Handle<UiPrefab>,
}

impl MyPlayState {
    #[inline]
    fn new(scene: Handle<Prefab<MyScenePrefab>>, info_ui: Handle<UiPrefab>) -> MyPlayState {
        MyPlayState { scene, info_ui }
    }
}

impl SimpleState for MyPlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        use amethyst::controls::HideCursor;
        use amethyst::ecs::Builder;

        data.world.create_entity().with(self.scene.clone()).build();
        data.world
            .create_entity()
            .with(self.info_ui.clone())
            .build();
        data.world.write_resource::<HideCursor>().hide = false;
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        use amethyst::controls::HideCursor;
        use amethyst::ecs::Builder;
        use amethyst::input::{is_close_requested, is_key_down};
        use amethyst::renderer::VirtualKeyCode;
        use amethyst::ui::{UiEventType, UiFinder};

        match event {
            // window events
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    // toggle the info ui if escape key is pressed
                    let info_ui = data
                        .world
                        .exec(|finder: UiFinder<'_>| finder.find("info_dialog"));
                    match info_ui {
                        Some(entity) => {
                            let _ = data.world.delete_entity(entity);
                            data.world.write_resource::<HideCursor>().hide = true;
                        }
                        None => {
                            data.world
                                .create_entity()
                                .with(self.info_ui.clone())
                                .build();
                            data.world.write_resource::<HideCursor>().hide = false;
                        }
                    }
                    Trans::None
                } else {
                    Trans::None
                }
            }

            // ui events
            StateEvent::Ui(event) => {
                if event.event_type == UiEventType::Click
                    && Some(event.target)
                        == data
                            .world
                            .exec(|finder: UiFinder<'_>| finder.find("info_button"))
                {
                    data.world
                        .exec(|finder: UiFinder<'_>| finder.find("info_dialog"))
                        .map(|e| data.world.delete_entity(e));
                    data.world.write_resource::<HideCursor>().hide = true;
                }
                Trans::None
            }
        }
    }
}
