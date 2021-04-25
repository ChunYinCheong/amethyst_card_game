use amethyst::{
    assets::Handle,
    ecs::{
        prelude::{Entity, WorldExt},
        Entities,
    },
    prelude::*,
    ui::{Anchor, FontAsset, UiImage, UiText, UiTransform},
};
pub struct BigIcon {
    text_entity: Entity,
}
impl BigIcon {
    pub fn new(
        world: &mut World,
        font: &Handle<FontAsset>,
        width: f32,
        height: f32,
        x: i32,
        y: i32,
    ) -> Self {
        let border = 10.;
        let text_entity = world
            .create_entity()
            .with(UiTransform::new(
                "P1".to_string(),
                Anchor::BottomLeft,
                Anchor::BottomLeft,
                x as f32,
                y as f32,
                1.,
                width,
                height,
            ))
            .with(UiText::new(
                font.clone(),
                "0".to_string(),
                [1., 1., 1., 1.],
                50.,
            ))
            .with(UiImage::SolidColor([0., 0., 1., 0.5]))
            .build();
        BigIcon { text_entity }
    }
    pub fn delete(&mut self, world: &Entities) {
        world
            .delete(self.text_entity)
            .expect("Failed to remove Game Screen");
    }
}
