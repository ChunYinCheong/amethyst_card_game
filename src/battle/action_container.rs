use super::action_ui::ActionUi;

use amethyst::{
    assets::Handle,
    ecs::{
        prelude::{Entity, WorldExt},
        Entities, WriteStorage,
    },
    prelude::*,
    ui::{Anchor, FontAsset, UiImage, UiText, UiTransform},
};

pub struct ActionContainer {
    pub text_entity: Entity,
    pub width: f32,
    pub height: f32,
    pub action_ui: Vec<ActionUi>,
}

impl ActionContainer {
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
                    1.,
                    width,
                    height,
                );
                // Container never handle event
                tran.opaque = false;
                tran
            })
            .with(UiImage::SolidColor([0., 0., 1., 0.5]))
            .build();
        let action_ui = (0..4)
            .map(|i| {
                ActionUi::new(
                    world,
                    &font,
                    width,
                    height / 4.,
                    x,
                    y + height / 4. * (i as f32),
                    i,
                )
            })
            .collect();
        ActionContainer {
            text_entity,
            width,
            height,
            action_ui,
        }
    }

    pub fn delete(&mut self, world: &Entities) {
        world
            .delete(self.text_entity)
            .expect("Failed to remove Game Screen");
        for ui in &mut self.action_ui {
            ui.delete(world);
        }
    }

    pub fn set_visible(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        ui_transform: &mut WriteStorage<UiTransform>,
        visible: bool,
    ) {
        for ui in self.action_ui.iter_mut() {
            ui.set_visible(ui_text, ui_image, ui_transform, visible);
        }

        if let Some(ui_text) = ui_transform.get_mut(self.text_entity) {
            // Keep opaque = false, Container never handle event
            if visible {
                ui_text.width = self.width;
                ui_text.height = self.height;
            } else {
                ui_text.width = 0.0;
                ui_text.height = 0.0;
            }
        }
    }
}
