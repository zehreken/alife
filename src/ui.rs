pub struct Ui;

impl SimpleState for Ui {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

    }
}

fn initialize_ui (world: &mut World) {
    let font = world. read_resource::<Loader>().load(
        "font/square.ttf",
         TtfFormat,
         (),
         &world.read_resource(),
    );

    let text_transform = UiTransform::new(
        "info".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.0,
        -50.0,
        1.0,
        200.0,
        50.0,
    );

    let text_entity = world
        .create_entity()
        .with(text_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        ))
        .build();
}