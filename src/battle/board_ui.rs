use super::{
    action_container::ActionContainer, big_icon::BigIcon, card_ui::CardUi,
    end_turn_button::EndTurnButton, log_button::LogButton, message_log::MessageLog,
    overlay::Overlay,
};
use crate::board_game::card::{Card, CardPosition};

use amethyst::{
    assets::{Handle, Loader},
    ecs::{prelude::WorldExt, Entities},
    prelude::*,
    ui::{FontAsset, TtfFormat, UiImage, UiText, UiTransform},
};

fn init_card_ui(
    card_uis: &mut Vec<CardUi>,
    world: &mut World,
    font: &Handle<FontAsset>,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    position: CardPosition,
    position_index: usize,
) -> usize {
    let ui = CardUi::new(world, &font, width, height, x, y, position, position_index);
    let id = card_uis.len();
    card_uis.push(ui);
    id
}

pub struct BoardUi {
    pub card_uis: Vec<CardUi>,

    pub player2_back_cards: Vec<usize>,
    pub player2_center_cards: Vec<usize>,
    pub player2_front_cards: Vec<usize>,
    pub player1_front_cards: Vec<usize>,
    pub player1_center_cards: Vec<usize>,
    pub player1_back_cards: Vec<usize>,

    pub player2_hand_cards: Vec<usize>,
    pub player1_hand_cards: Vec<usize>,
    pub message_log: MessageLog,
    pub action_container: ActionContainer,
    pub avatar: BigIcon,
    pub show_log_button: LogButton,
    pub end_turn_button: EndTurnButton,
    pub overlay: Overlay,
}

impl BoardUi {
    pub fn new(world: &mut World) -> Self {
        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        let width = 120;
        let height = 120;
        let space = 80;
        let mut card_uis = Vec::new();

        let player2_back_cards = (0..5)
            .map(|i| {
                init_card_ui(
                    &mut card_uis,
                    world,
                    &font,
                    width,
                    height,
                    i * width,
                    5 * height,
                    CardPosition::Back,
                    i as usize,
                )
            })
            .collect();
        let player2_center_cards = (0..5)
            .map(|i| {
                init_card_ui(
                    &mut card_uis,
                    world,
                    &font,
                    width,
                    height,
                    i * width,
                    4 * height,
                    CardPosition::Center,
                    i as usize,
                )
            })
            .collect();
        let player2_front_cards = (0..5)
            .map(|i| {
                init_card_ui(
                    &mut card_uis,
                    world,
                    &font,
                    width,
                    height,
                    i * width,
                    3 * height,
                    CardPosition::Front,
                    i as usize,
                )
            })
            .collect();
        let player1_front_cards = (0..5)
            .map(|i| {
                init_card_ui(
                    &mut card_uis,
                    world,
                    &font,
                    width,
                    height,
                    i * width,
                    2 * height,
                    CardPosition::Front,
                    i as usize,
                )
            })
            .collect();
        let player1_center_cards = (0..5)
            .map(|i| {
                init_card_ui(
                    &mut card_uis,
                    world,
                    &font,
                    width,
                    height,
                    i * width,
                    1 * height,
                    CardPosition::Center,
                    i as usize,
                )
            })
            .collect();
        let player1_back_cards = (0..5)
            .map(|i| {
                init_card_ui(
                    &mut card_uis,
                    world,
                    &font,
                    width,
                    height,
                    i * width,
                    0 * height,
                    CardPosition::Back,
                    i as usize,
                )
            })
            .collect();

        let player2_hand_cards = (0..5)
            .map(|i| {
                init_card_ui(
                    &mut card_uis,
                    world,
                    &font,
                    width,
                    height,
                    (i + 5) * width + space,
                    5 * height,
                    CardPosition::Hand,
                    i as usize,
                )
            })
            .collect();
        let player1_hand_cards = (0..5)
            .map(|i| {
                init_card_ui(
                    &mut card_uis,
                    world,
                    &font,
                    width,
                    height,
                    (i + 5) * width + space,
                    0 * height,
                    CardPosition::Hand,
                    i as usize,
                )
            })
            .collect();

        let message_log = MessageLog::new(world, &font, 360., 480., 680, 120);
        let mut action_container = ActionContainer::new(world, &font, 360., 480., 680., 120.);
        {
            let ui_text = &mut world.write_storage::<UiText>();
            let ui_image = &mut world.write_storage::<UiImage>();
            let ui_transform = &mut world.write_storage::<UiTransform>();

            action_container.set_visible(ui_text, ui_image, ui_transform, false);
        }

        let avatar = BigIcon::new(world, &font, 240., 240., 1040, 360);
        let show_log_button = LogButton::new(world, &font, 240., 120., 1040, 240);
        let end_turn_button = EndTurnButton::new(world, &font, 240., 120., 1040, 120);
        let overlay = Overlay::new(world, &font, 600.0, 240.0, 0.0, 240.0);
        BoardUi {
            card_uis,

            player2_back_cards,
            player2_center_cards,
            player2_front_cards,
            player1_front_cards,
            player1_center_cards,
            player1_back_cards,

            player2_hand_cards,
            player1_hand_cards,

            message_log,
            action_container,
            avatar,
            show_log_button,
            end_turn_button,
            overlay,
        }
    }

    pub fn delete(&mut self, world: &Entities) {
        let mut card_uis = [
            &self.player2_back_cards,
            &self.player2_center_cards,
            &self.player2_front_cards,
            &self.player1_front_cards,
            &self.player1_center_cards,
            &self.player1_back_cards,
            &self.player2_hand_cards,
            &self.player1_hand_cards,
        ];
        for card_ui in card_uis.iter().flat_map(|v| v.iter()) {
            self.card_uis[*card_ui].delete(world);
        }

        self.message_log.delete(world);
        self.action_container.delete(world);
        self.avatar.delete(world);
        self.show_log_button.delete(world);
        self.end_turn_button.delete(world);
        self.overlay.delete(world);
    }
    pub fn get_card_ui_by_card(&mut self, card: &Card) -> &mut CardUi {
        match &card.card_position {
            CardPosition::Deck => panic!("Place card in Deck!"),
            CardPosition::Hand => panic!("Place card in Hand!"),
            CardPosition::Grave => panic!("Place card in Grave!"),
            CardPosition::Front => {
                let v = if card.player_id == 0 {
                    &mut self.player1_front_cards
                } else {
                    &mut self.player2_front_cards
                };
                &mut self.card_uis[v[card.position_index.unwrap()]]
            }
            CardPosition::Center => {
                let v = if card.player_id == 0 {
                    &mut self.player1_center_cards
                } else {
                    &mut self.player2_center_cards
                };
                &mut self.card_uis[v[card.position_index.unwrap()]]
            }
            CardPosition::Back => {
                let v = if card.player_id == 0 {
                    &mut self.player1_back_cards
                } else {
                    &mut self.player2_back_cards
                };
                &mut self.card_uis[v[card.position_index.unwrap()]]
            }
        }
    }
}
