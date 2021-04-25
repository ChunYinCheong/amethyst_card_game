use amethyst::{
    assets::Handle,
    ecs::{
        prelude::{Entity, WorldExt},
        Entities, WriteStorage,
    },
    prelude::*,
    ui::{Anchor, FontAsset, UiImage, UiText, UiTransform},
};
pub struct Overlay {
    text_entity: Entity,
}
impl Overlay {
    pub fn new(
        world: &mut World,
        font: &Handle<FontAsset>,
        width: f32,
        height: f32,
        x: f32,
        y: f32,
    ) -> Self {
        let border = 10.;
        let text_entity = world
            .create_entity()
            .with({
                let mut tran = UiTransform::new(
                    "P1".to_string(),
                    Anchor::BottomLeft,
                    Anchor::BottomLeft,
                    x,
                    y,
                    100.,
                    width,
                    height,
                );
                tran.opaque = false;
                tran
            })
            .with(UiText::new(
                font.clone(),
                "End Turn".to_string(),
                [1., 1., 1., 0.],
                50.,
            ))
            .with(UiImage::SolidColor([0., 0., 0., 0.]))
            .build();
        Overlay { text_entity }
    }

    pub fn delete(&mut self, world: &Entities) {
        world
            .delete(self.text_entity)
            .expect("Failed to remove Game Screen");
    }
    pub fn set_animation(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        s: String,
        time: f32,
    ) {
        if let Some(ui_text) = ui_text.get_mut(self.text_entity) {
            ui_text.text = s;
            ui_text.color[3] = if time >= 1.0 {
                0.0
            } else if time >= 0.5 {
                1.0 - time
            } else {
                time
            };
        }

        if let Some(text_image) = ui_image.get_mut(self.text_entity) {
            if let UiImage::SolidColor(a) = text_image {
                a[0] = 0.0;
                a[1] = 0.0;
                a[2] = 0.0;
                a[3] = if time >= 1.0 {
                    0.0
                } else if time >= 0.5 {
                    1.0 - time
                } else {
                    time
                };
            }
        }
    }

    pub fn event_entity(&self) -> Entity {
        self.text_entity
    }
}
