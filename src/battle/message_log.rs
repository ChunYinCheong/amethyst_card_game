use amethyst::{
    assets::Handle,
    ecs::{
        prelude::{Entity, WorldExt},
        Entities, WriteStorage,
    },
    prelude::*,
    ui::{Anchor, FontAsset, LineMode, UiImage, UiText, UiTransform},
};

pub struct MessageLog {
    text_entity: Entity,
    width: f32,
    height: f32,
}
impl MessageLog {
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
            .with({
                let mut text = UiText::new(font.clone(), "0".to_string(), [1., 1., 1., 1.], 24.);
                text.align = Anchor::BottomLeft;
                text.line_mode = LineMode::Wrap;
                text
            })
            .with(UiImage::SolidColor([0.5, 0.5, 0.5, 0.5]))
            .build();
        MessageLog {
            text_entity,
            width,
            height,
        }
    }

    pub fn delete(&mut self, world: &Entities) {
        world
            .delete(self.text_entity)
            .expect("Failed to remove Game Screen");
    }
    pub fn set_text(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        text: String,
    ) {
        if let Some(ui_text) = ui_text.get_mut(self.text_entity) {
            ui_text.text = text;
        }
    }

    pub fn set_visible(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        ui_transform: &mut WriteStorage<UiTransform>,
        visible: bool,
    ) {
        if visible {
            if let Some(ui_text) = ui_transform.get_mut(self.text_entity) {
                ui_text.opaque = true;
                ui_text.width = self.width;
                ui_text.height = self.height;
            }
        } else {
            if let Some(ui_text) = ui_transform.get_mut(self.text_entity) {
                ui_text.opaque = false;
                ui_text.width = 0.0;
                ui_text.height = 0.0;
            }
        }
    }
}
