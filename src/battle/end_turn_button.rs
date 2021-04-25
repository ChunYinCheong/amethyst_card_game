use amethyst::{
    assets::Handle,
    ecs::{
        prelude::{Entity, WorldExt},
        Entities, WriteStorage,
    },
    prelude::*,
    ui::{Anchor, FontAsset, UiImage, UiText, UiTransform},
};
pub struct EndTurnButton {
    text_entity: Entity,
}
impl EndTurnButton {
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
                "End Turn".to_string(),
                [1., 1., 1., 1.],
                50.,
            ))
            .with(UiImage::SolidColor([0., 0., 1., 0.5]))
            .build();
        EndTurnButton { text_entity }
    }

    pub fn delete(&mut self, world: &Entities) {
        world
            .delete(self.text_entity)
            .expect("Failed to remove Game Screen");
    }
    pub fn set_hovering(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        hovering: bool,
    ) {
        // self.hovering = hovering;
        if let Some(text_image) = ui_image.get_mut(self.text_entity) {
            if let UiImage::SolidColor(a) = text_image {
                if hovering {
                    a[0] = 1.0;
                    a[1] = 1.0;
                    a[2] = 1.0;
                    a[3] = 0.5;
                } else {
                    a[0] = 0.0;
                    a[1] = 0.0;
                    a[2] = 1.0;
                    a[3] = 0.5;
                }
            }
        }
    }
    pub fn event_entity(&self) -> Entity {
        self.text_entity
    }
}
