use crate::board_game::{
    action::{Action, ActionId},
    card::CardId,
};
use amethyst::{
    assets::Handle,
    ecs::{
        prelude::{Entity, WorldExt},
        Entities, WriteStorage,
    },
    prelude::*,
    ui::{Anchor, FontAsset, UiImage, UiText, UiTransform},
};

pub struct ActionUi {
    pub id: usize,
    pub text_entity: Entity,
    pub background_entity: Entity,
    pub available_target: bool,
    pub selected: bool,
    pub hovering: bool,
    pub card_id: Option<CardId>,

    pub width: f32,
    pub height: f32,
    pub border: f32,

    pub action_id: Option<ActionId>,
}
impl ActionUi {
    pub fn new(
        world: &mut World,
        font: &Handle<FontAsset>,
        width: f32,
        height: f32,
        x: f32,
        y: f32,
        id: usize,
    ) -> Self {
        let border = 10.0;
        let text_entity = world
            .create_entity()
            .with({
                let mut transform = UiTransform::new(
                    "P1".to_string(),
                    Anchor::BottomLeft,
                    Anchor::BottomLeft,
                    x + border,
                    y + border,
                    3.,
                    width - border - border,
                    height - border - border,
                );
                transform.opaque = false;
                transform
            })
            .with(UiText::new(
                font.clone(),
                "Action".to_string(),
                [1., 1., 1., 1.],
                24.,
            ))
            .with(UiImage::SolidColor([1., 0., 0., 0.5]))
            .build();
        let background_entity = world
            .create_entity()
            .with(UiTransform::new(
                "P1".to_string(),
                Anchor::BottomLeft,
                Anchor::BottomLeft,
                x as f32,
                y as f32,
                2.,
                width as f32,
                height as f32,
            ))
            .with(UiImage::SolidColor([0., 0., 0., 0.5]))
            .build();
        ActionUi {
            text_entity,
            background_entity,
            available_target: false,
            selected: false,
            hovering: false,
            card_id: None,
            width: width,
            height: height,
            border,
            action_id: None,
            id,
        }
    }

    pub fn delete(&mut self, world: &Entities) {
        world
            .delete(self.text_entity)
            .expect("Failed to remove Game Screen");
        world
            .delete(self.background_entity)
            .expect("Failed to remove Game Screen");
    }
    pub fn set_hovering(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        hovering: bool,
    ) {
        self.hovering = hovering;
        if let Some(text_image) = ui_image.get_mut(self.text_entity) {
            if let UiImage::SolidColor(a) = text_image {
                if hovering {
                    a[0] = 1.0;
                    a[1] = 1.0;
                    a[2] = 1.0;
                    a[3] = 0.5;
                } else {
                    a[0] = 1.0;
                    a[1] = 0.0;
                    a[2] = 0.0;
                    a[3] = 0.5;
                }
            }
        }
    }

    pub fn set_selected(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        selected: bool,
    ) {
        self.selected = selected;
        self.update_background_color(ui_text, ui_image);
    }
    pub fn clear_action(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
    ) {
        if let Some(ui_text) = ui_text.get_mut(self.text_entity) {
            ui_text.text = "".to_string();
        }
        self.action_id = None;
    }
    pub fn set_action(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        action: &Action,
    ) {
        if let Some(ui_text) = ui_text.get_mut(self.text_entity) {
            ui_text.text = action.name.clone();
        }
        self.action_id = Some(action.id);
    }

    pub fn set_visible(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        ui_transform: &mut WriteStorage<UiTransform>,
        visible: bool,
    ) {
        if visible {
            if let Some(ui_text) = ui_transform.get_mut(self.background_entity) {
                ui_text.opaque = true;
                ui_text.width = self.width;
                ui_text.height = self.height;
            }
            if let Some(ui_text) = ui_transform.get_mut(self.text_entity) {
                ui_text.width = self.width - self.border - self.border;
                ui_text.height = self.height - self.border - self.border;
            }
        } else {
            if let Some(ui_text) = ui_transform.get_mut(self.background_entity) {
                ui_text.opaque = false;
                ui_text.width = 0.0;
                ui_text.height = 0.0;
            }
            if let Some(ui_text) = ui_transform.get_mut(self.text_entity) {
                ui_text.width = 0.0;
                ui_text.height = 0.0;
            }
        }
    }

    pub fn event_entity(&self) -> Entity {
        self.background_entity
    }

    fn update_background_color(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
    ) {
        if let Some(background) = ui_image.get_mut(self.background_entity) {
            if let UiImage::SolidColor(a) = background {
                if self.selected {
                    a[0] = 0.0;
                    a[1] = 1.0;
                    a[2] = 0.0;
                    a[3] = 0.5;
                } else {
                    a[0] = 0.0;
                    a[1] = 0.0;
                    a[2] = 0.0;
                    a[3] = 0.5;
                }
            }
        }
    }
}
