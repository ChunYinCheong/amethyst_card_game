use crate::{
    board_game::{
        card::{Card, CardPosition},
        status_effect::StatusEffect,
    },
    board_texture::BoardTexture,
};
use amethyst::{
    assets::Handle,
    ecs::{
        prelude::{Entity, WorldExt},
        Entities, WriteStorage,
    },
    prelude::*,
    shred::Read,
    ui::{Anchor, FontAsset, UiImage, UiText, UiTransform},
};

pub struct CardUi {
    pub text_entity: Entity,
    pub texture_entity: Entity,
    pub background_entity: Entity,
    pub animation_entity: Entity,
    pub hp_entity: Entity,
    pub mp_entity: Entity,
    pub status_effect_entity: Entity,
    pub available_target: bool,
    pub selected: bool,
    pub targeted: bool,
    pub hovering: bool,
    pub possible_target: bool,
    pub card_id: Option<usize>,
    pub position: CardPosition,
    pub position_index: usize,
}

impl CardUi {
    pub fn new(
        world: &mut World,
        font: &Handle<FontAsset>,
        width: i32,
        height: i32,
        x: i32,
        y: i32,
        position: CardPosition,
        position_index: usize,
    ) -> Self {
        let border = 4;
        let text_entity = world
            .create_entity()
            .with({
                let mut transform = UiTransform::new(
                    "P1".to_string(),
                    Anchor::BottomLeft,
                    Anchor::BottomLeft,
                    (x + border) as f32,
                    (y + border) as f32,
                    2.5,
                    (width - border - border) as f32,
                    (height - border - border) as f32,
                );
                transform.opaque = false;
                transform
            })
            .with(UiText::new(
                font.clone(),
                "0".to_string(),
                [1., 1., 1., 0.],
                24.,
            ))
            .with(UiImage::SolidColor([0., 0., 0., 0.]))
            .build();
        let texture_entity = world
            .create_entity()
            .with({
                let mut transform = UiTransform::new(
                    "P1".to_string(),
                    Anchor::BottomLeft,
                    Anchor::BottomLeft,
                    (x + border) as f32,
                    (y + border) as f32,
                    2.,
                    (width - border - border) as f32,
                    (height - border - border) as f32,
                );
                transform.opaque = false;
                transform
            })
            .with(UiImage::SolidColor([0., 0., 0., 0.]))
            .build();
        let hp_entity = world
            .create_entity()
            .with({
                let mut transform = UiTransform::new(
                    "P1".to_string(),
                    Anchor::BottomLeft,
                    Anchor::BottomLeft,
                    (x + border) as f32,
                    (y + border) as f32,
                    3.,
                    (width - border - border) as f32,
                    (height - border - border) as f32,
                );
                transform.opaque = false;
                transform
            })
            .with({
                let mut text = UiText::new(font.clone(), "0".to_string(), [0., 1., 0., 1.], 24.);
                text.align = Anchor::BottomLeft;
                text
            })
            .build();
        let mp_entity = world
            .create_entity()
            .with({
                let mut transform = UiTransform::new(
                    "P1".to_string(),
                    Anchor::BottomLeft,
                    Anchor::BottomLeft,
                    (x + border) as f32,
                    (y + border) as f32,
                    3.,
                    (width - border - border) as f32,
                    (height - border - border) as f32,
                );
                transform.opaque = false;
                transform
            })
            .with({
                let mut text = UiText::new(font.clone(), "0".to_string(), [0., 0., 1., 1.], 24.);
                text.align = Anchor::BottomRight;
                text
            })
            .build();
        let background_entity = world
            .create_entity()
            .with(UiTransform::new(
                "P1".to_string(),
                Anchor::BottomLeft,
                Anchor::BottomLeft,
                x as f32,
                y as f32,
                1.,
                width as f32,
                height as f32,
            ))
            .with(UiImage::SolidColor([0., 0., 0., 0.5]))
            .build();
        let animation_entity = world
            .create_entity()
            .with({
                let mut transform = UiTransform::new(
                    "P1".to_string(),
                    Anchor::BottomLeft,
                    Anchor::BottomLeft,
                    x as f32,
                    y as f32,
                    10.,
                    width as f32,
                    height as f32,
                );
                transform.opaque = false;
                transform
            })
            .with(UiImage::SolidColor([0., 0., 0., 0.]))
            .build();
        let status_effect_entity = world
            .create_entity()
            .with({
                let mut transform = UiTransform::new(
                    "P1".to_string(),
                    Anchor::BottomLeft,
                    Anchor::BottomLeft,
                    (x + border) as f32,
                    (y + height - (height / 4) - border) as f32,
                    3.,
                    (width / 4) as f32,
                    (height / 4) as f32,
                );
                transform.opaque = false;
                transform
            })
            .with(UiImage::SolidColor([0., 0., 0., 0.]))
            .with(UiText::new(
                font.clone(),
                "0".to_string(),
                [1., 1., 1., 1.],
                24.,
            ))
            .build();
        CardUi {
            text_entity,
            texture_entity,
            background_entity,
            animation_entity,
            available_target: false,
            selected: false,
            targeted: false,
            hovering: false,
            possible_target: false,
            card_id: None,
            hp_entity,
            mp_entity,
            position,
            position_index,
            status_effect_entity,
        }
    }

    pub fn delete(&mut self, world: &Entities) {
        world
            .delete(self.text_entity)
            .expect("Failed to remove Game Screen");
        world
            .delete(self.texture_entity)
            .expect("Failed to remove Game Screen");
        world
            .delete(self.background_entity)
            .expect("Failed to remove Game Screen");
        world
            .delete(self.animation_entity)
            .expect("Failed to remove Game Screen");
        world
            .delete(self.hp_entity)
            .expect("Failed to remove Game Screen");
        world
            .delete(self.mp_entity)
            .expect("Failed to remove Game Screen");
        world
            .delete(self.status_effect_entity)
            .expect("Failed to remove Game Screen");
    }
    pub fn set_card(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        card: &Card,
        texture: &Read<BoardTexture>,
    ) {
        if let Some(ui_text) = ui_text.get_mut(self.text_entity) {
            ui_text.text = card.name.clone();
        }
        if let Some(ui_text) = ui_text.get_mut(self.hp_entity) {
            ui_text.text = card.hp.to_string();
        }
        if let Some(ui_text) = ui_text.get_mut(self.mp_entity) {
            ui_text.text = card.mp.to_string();
        }
        if let Some(ui_image) = ui_image.get_mut(self.texture_entity) {
            *ui_image = UiImage::Texture(texture.get(&card.texture_path));
        }
        if let Some(ui_image) = ui_image.get_mut(self.status_effect_entity) {
            *ui_image = UiImage::SolidColor([0., 0., 0., 0.]);
        }
        if let Some(ui_text) = ui_text.get_mut(self.status_effect_entity) {
            ui_text.text = "".to_string();
        }
        for (se, d) in &card.status_effects {
            if *d > 0 {
                match se {
                    StatusEffect::Sleep => {}
                    StatusEffect::Poison => {}
                    StatusEffect::Suppression => {
                        if let Some(ui_image) = ui_image.get_mut(self.status_effect_entity) {
                            *ui_image = UiImage::Texture(
                                texture.get("board_game/status_effect/suppression.png"),
                            );
                        }
                        if let Some(ui_text) = ui_text.get_mut(self.status_effect_entity) {
                            ui_text.text = d.to_string();
                        }
                    }
                    StatusEffect::Burn => {}
                }
            }
        }

        self.card_id = Some(card.id);
    }
    pub fn clear_card(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        texture: &Read<BoardTexture>,
    ) {
        if let Some(ui_text) = ui_text.get_mut(self.text_entity) {
            ui_text.text = "".to_string();
        }
        if let Some(ui_text) = ui_text.get_mut(self.hp_entity) {
            ui_text.text = "".to_string();
        }
        if let Some(ui_text) = ui_text.get_mut(self.mp_entity) {
            ui_text.text = "".to_string();
        }
        if let Some(ui_image) = ui_image.get_mut(self.texture_entity) {
            *ui_image = UiImage::Texture(texture.get("board_game/card/empty.png"));
        }
        if let Some(ui_text) = ui_text.get_mut(self.status_effect_entity) {
            ui_text.text = "".to_string();
        }
        if let Some(ui_image) = ui_image.get_mut(self.status_effect_entity) {
            *ui_image = UiImage::SolidColor([0., 0., 0., 0.]);
        }

        self.card_id = None;
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
                    a[0] = 0.0;
                    a[1] = 0.0;
                    a[2] = 0.0;
                    a[3] = 0.0;
                }
            }
        }
    }

    pub fn set_targeted(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        targeted: bool,
    ) {
        self.targeted = targeted;
        self.update_background(ui_text, ui_image);
    }
    pub fn set_selected(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        selected: bool,
    ) {
        self.selected = selected;
        self.update_background(ui_text, ui_image);
    }
    pub fn set_possible_target(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        possible_target: bool,
    ) {
        self.possible_target = possible_target;
        self.update_background(ui_text, ui_image);
    }

    pub fn set_place_card_animation(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        time: f32,
    ) -> bool {
        if let Some(text_image) = ui_image.get_mut(self.animation_entity) {
            if let UiImage::SolidColor(a) = text_image {
                a[0] = 1.0;
                a[1] = 1.0;
                a[2] = 1.0;
                a[3] = if time >= 1.0 { 0.0 } else { time };
            }
        }
        time > 1.0
    }
    pub fn set_action_animation(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        time: f32,
    ) -> bool {
        if let Some(text_image) = ui_image.get_mut(self.animation_entity) {
            if let UiImage::SolidColor(a) = text_image {
                a[0] = 1.0;
                a[1] = 1.0;
                a[2] = 1.0;
                a[3] = if time >= 0.5 { 0.0 } else { time * 2.0 };
            }
        }
        time > 0.5
    }
    pub fn set_attack_animation(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        time: f32,
    ) -> bool {
        if let Some(text_image) = ui_image.get_mut(self.animation_entity) {
            if let UiImage::SolidColor(a) = text_image {
                a[0] = 1.0;
                a[1] = 0.0;
                a[2] = 0.0;
                a[3] = if time >= 0.5 { 0.0 } else { 2.0 * (0.5 - time) };
            }
        }
        time > 0.5
    }
    pub fn set_heal_animation(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        time: f32,
    ) -> bool {
        if let Some(text_image) = ui_image.get_mut(self.animation_entity) {
            if let UiImage::SolidColor(a) = text_image {
                a[0] = 0.0;
                a[1] = 1.0;
                a[2] = 0.0;
                a[3] = if time >= 0.5 { 0.0 } else { time * 2.0 };
            }
        }
        time > 0.5
    }

    fn update_background(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
    ) {
        if let Some(background) = ui_image.get_mut(self.background_entity) {
            if let UiImage::SolidColor(a) = background {
                if self.possible_target {
                    a[0] = 1.0;
                    a[1] = 1.0;
                    a[2] = 0.0;
                    a[3] = 0.5;
                } else if self.selected {
                    a[0] = 0.0;
                    a[1] = 1.0;
                    a[2] = 0.0;
                    a[3] = 0.5;
                } else if self.targeted {
                    a[0] = 1.0;
                    a[1] = 0.0;
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
    pub fn event_entity(&self) -> Entity {
        self.background_entity
    }
}
