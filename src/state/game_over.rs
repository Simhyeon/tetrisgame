use amethyst::{
    prelude::*,
    ui::{ Anchor, UiButtonBuilder},
};

#[derive(Default)]
pub struct GameOverState;

impl SimpleState for GameOverState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = &data.world;

        UiButtonBuilder::<(), u32>::new("Game Over".to_string())
            .with_font_size(50.0)
            .with_position(-55.0, -450.0)
            .with_size(64.0 * 6.0, 64.0)
            .with_anchor(Anchor::TopMiddle)
            .with_text_color([255.0,255.0,255.0, 1.0])
            //.with_image(UiImage::SolidColor([0.8, 0.6, 0.3, 1.0]))
            //.with_hover_image(UiImage::SolidColor([0.1, 0.1, 0.1, 0.5]))
            .build_from_world(&world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}
