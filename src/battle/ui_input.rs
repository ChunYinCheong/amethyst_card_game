use super::{
    action_container::ActionContainer, action_ui::ActionUi, big_icon::BigIcon, card_ui::CardUi,
    end_turn_button::EndTurnButton, input_state::InputState, log_button::LogButton,
    message_log::MessageLog,
};
use crate::board_game::{
    action::Action,
    board::Board,
    board_game::{BoardGame, Target},
    board_resource::BoardResource,
    card::{Card, CardPosition},
    game_move::GameMove,
    target_type::TargetType,
};
use crate::board_texture::BoardTexture;
use itertools::Itertools;

use crate::{battle::board_ui::BoardUi, menu::MainMenu};
use amethyst::{
    core::Time,
    ecs::{prelude::WorldExt, Entities, WriteStorage},
    prelude::*,
    shred::Read,
    ui::{UiEvent, UiEventType, UiImage, UiText, UiTransform},
};

pub struct UiAndInput {
    pub board_ui: BoardUi,
    pub input_state: InputState,
    pub board_index: usize,
    pub animation_time: f32,
    pub animation_end: bool,
}

impl UiAndInput {
    pub fn new(world: &mut World) -> Self {
        UiAndInput {
            board_ui: BoardUi::new(world),
            input_state: InputState::Normal,
            board_index: 0,
            animation_time: 0.,
            animation_end: false,
        }
    }
    pub fn delete(&mut self, world: &Entities) {
        self.board_ui.delete(world);
    }

    pub fn update(
        &mut self,
        board_game: &BoardGame,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        ui_transform: &mut WriteStorage<UiTransform>,
        time: &Time,
        texture: &Read<BoardTexture>,
    ) -> SimpleTrans {
        if let Some(history) = &board_game.histories.get(self.board_index) {
            let board = &history.board;
            let board_resource = &board_game.board_resource;
            {
                self.animation_time += time.delta_seconds();
            }
            // Update animation...
            // Change animation_end to true when end
            match &history.event.animation {
                crate::board_game::board_game::Animation::Init => {
                    self.board_ui.overlay.set_animation(
                        ui_text,
                        ui_image,
                        "Init Game".to_string(),
                        self.animation_time,
                    );
                    if self.animation_time > 1.0 {
                        self.animation_end = true;
                    }
                }
                crate::board_game::board_game::Animation::PlaceCard(id) => {
                    let card = &board_resource.cards[board.cards[*id]];
                    let card_ui = self.board_ui.get_card_ui_by_card(card);

                    if card_ui.set_place_card_animation(ui_text, ui_image, self.animation_time) {
                        self.animation_end = true;
                    }
                }
                crate::board_game::board_game::Animation::StartTurn(_) => {
                    self.board_ui.overlay.set_animation(
                        ui_text,
                        ui_image,
                        "Start Turn".to_string(),
                        self.animation_time,
                    );
                    if self.animation_time > 1.0 {
                        self.animation_end = true;
                    }
                }
                crate::board_game::board_game::Animation::EndTTurn(_) => {
                    self.board_ui.overlay.set_animation(
                        ui_text,
                        ui_image,
                        "End Turn".to_string(),
                        self.animation_time,
                    );
                    if self.animation_time > 1.0 {
                        self.animation_end = true;
                    }
                }
                crate::board_game::board_game::Animation::DoAction(id) => {
                    let card = &board_resource.cards[board.cards[*id]];
                    let card_ui = self.board_ui.get_card_ui_by_card(card);
                    if card_ui.set_action_animation(ui_text, ui_image, self.animation_time) {
                        self.animation_end = true;
                    }
                }
                crate::board_game::board_game::Animation::Attack(id) => {
                    let card = &board_resource.cards[board.cards[*id]];
                    let card_ui = self.board_ui.get_card_ui_by_card(card);
                    if card_ui.set_attack_animation(ui_text, ui_image, self.animation_time) {
                        self.animation_end = true;
                    }
                }
                crate::board_game::board_game::Animation::Heal(id) => {
                    let card = &board_resource.cards[board.cards[*id]];
                    let card_ui = self.board_ui.get_card_ui_by_card(card);
                    if card_ui.set_heal_animation(ui_text, ui_image, self.animation_time) {
                        self.animation_end = true;
                    }
                }
                crate::board_game::board_game::Animation::Die(_) => {
                    if self.animation_time > 1.0 {
                        self.animation_end = true;
                    }
                }
                crate::board_game::board_game::Animation::EndGame => {
                    self.board_ui.overlay.set_animation(
                        ui_text,
                        ui_image,
                        "End Game".to_string(),
                        self.animation_time,
                    );

                    if self.animation_time > 1.0 {
                        self.animation_end = true;
                        return Trans::Switch(Box::new(MainMenu::default()));
                    }
                }
            }
            if self.animation_end {
                // Update board and index, reset field
                Self::update_card_ui(
                    ui_text,
                    ui_image,
                    &mut self.board_ui.card_uis,
                    &mut self.board_ui.player2_back_cards,
                    board,
                    &board_resource.players[board.players[1]].backs,
                    &board_game.board_resource,
                    texture,
                );
                Self::update_card_ui(
                    ui_text,
                    ui_image,
                    &mut self.board_ui.card_uis,
                    &mut self.board_ui.player2_center_cards,
                    board,
                    &board_resource.players[board.players[1]].centers,
                    &board_game.board_resource,
                    texture,
                );
                Self::update_card_ui(
                    ui_text,
                    ui_image,
                    &mut self.board_ui.card_uis,
                    &mut self.board_ui.player2_front_cards,
                    board,
                    &board_resource.players[board.players[1]].fronts,
                    &board_game.board_resource,
                    texture,
                );
                Self::update_card_ui(
                    ui_text,
                    ui_image,
                    &mut self.board_ui.card_uis,
                    &mut self.board_ui.player1_front_cards,
                    board,
                    &board_resource.players[board.players[0]].fronts,
                    &board_game.board_resource,
                    texture,
                );
                Self::update_card_ui(
                    ui_text,
                    ui_image,
                    &mut self.board_ui.card_uis,
                    &mut self.board_ui.player1_center_cards,
                    board,
                    &board_resource.players[board.players[0]].centers,
                    &board_game.board_resource,
                    texture,
                );
                Self::update_card_ui(
                    ui_text,
                    ui_image,
                    &mut self.board_ui.card_uis,
                    &mut self.board_ui.player1_back_cards,
                    board,
                    &board_resource.players[board.players[0]].backs,
                    &board_game.board_resource,
                    texture,
                );
                Self::update_card_ui_vec(
                    ui_text,
                    ui_image,
                    &mut self.board_ui.card_uis,
                    &mut self.board_ui.player2_hand_cards,
                    board,
                    &board_resource.players[board.players[1]].hands,
                    &board_game.board_resource,
                    texture,
                );
                Self::update_card_ui_vec(
                    ui_text,
                    ui_image,
                    &mut self.board_ui.card_uis,
                    &mut self.board_ui.player1_hand_cards,
                    board,
                    &board_resource.players[board.players[0]].hands,
                    &board_game.board_resource,
                    texture,
                );

                let text = board_game.histories[..self.board_index + 1]
                    .iter()
                    .map(|h| &h.event.message)
                    .cloned()
                    .join("\n");
                self.board_ui.message_log.set_text(ui_text, ui_image, text);

                self.board_index += 1;
                self.animation_end = false;
                self.animation_time = 0.0;

                let state = if board.current_player_index == 0 {
                    InputState::Normal
                } else {
                    InputState::Blocked
                };
                self.input_state.exit(
                    ui_text,
                    ui_image,
                    ui_transform,
                    &mut self.board_ui,
                    board_game,
                );
                state.enter(
                    ui_text,
                    ui_image,
                    ui_transform,
                    &mut self.board_ui,
                    board_game,
                );
                self.input_state = state;
            } else {
                match self.input_state {
                    InputState::Blocked => (),
                    _ => {
                        let state = InputState::Blocked;
                        self.input_state.exit(
                            ui_text,
                            ui_image,
                            ui_transform,
                            &mut self.board_ui,
                            board_game,
                        );
                        state.enter(
                            ui_text,
                            ui_image,
                            ui_transform,
                            &mut self.board_ui,
                            board_game,
                        );
                        self.input_state = state;
                    }
                }
            }
        }
        Trans::None
    }
    fn update_card_ui(
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        card_uis: &mut Vec<CardUi>,
        card_ui_ids: &Vec<usize>,
        board: &Board,
        card_ids: &[Option<usize>],
        board_resource: &BoardResource,
        texture: &Read<BoardTexture>,
    ) {
        for i in 0..card_ui_ids.len() {
            let card_ui = &mut card_uis[card_ui_ids[i]];
            match card_ids[i] {
                Some(id) => {
                    let card = &board_resource.cards[board.cards[id]];
                    card_ui.set_card(ui_text, ui_image, card, texture);
                }
                None => {
                    card_ui.clear_card(ui_text, ui_image, texture);
                }
            }
        }
    }
    fn update_card_ui_vec(
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        card_uis: &mut Vec<CardUi>,
        card_ui_ids: &Vec<usize>,
        board: &Board,
        card_ids: &Vec<usize>,
        board_resource: &BoardResource,
        texture: &Read<BoardTexture>,
    ) {
        for i in 0..card_ui_ids.len() {
            let card_ui = &mut card_uis[card_ui_ids[i]];
            match card_ids.get(i) {
                Some(id) => {
                    let card = &board_resource.cards[board.cards[*id]];
                    card_ui.set_card(ui_text, ui_image, card, texture);
                }
                None => {
                    card_ui.clear_card(ui_text, ui_image, texture);
                }
            }
        }
    }

    pub fn right_click(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        ui_transform: &mut WriteStorage<UiTransform>,
        board_game: &BoardGame,
    ) {
        // Cancel current input state
        // ...
        match &mut self.input_state {
            InputState::Normal => {}
            InputState::SelectedCardInHand { selected_card_ui } => {
                self.input_state.exit(
                    ui_text,
                    ui_image,
                    ui_transform,
                    &mut self.board_ui,
                    board_game,
                );
                self.input_state = InputState::Normal;
                self.input_state.enter(
                    ui_text,
                    ui_image,
                    ui_transform,
                    &mut self.board_ui,
                    board_game,
                );
            }
            InputState::SelectedCardInField { selected_card_ui } => {
                self.input_state.exit(
                    ui_text,
                    ui_image,
                    ui_transform,
                    &mut self.board_ui,
                    board_game,
                );
                self.input_state = InputState::Normal;
                self.input_state.enter(
                    ui_text,
                    ui_image,
                    ui_transform,
                    &mut self.board_ui,
                    board_game,
                );
            }
            InputState::SelectingActionTarget {
                selected_card_ui,
                selected_action_ui,
                selected_targets_ui,
            } => {
                if selected_targets_ui.is_empty() {
                    let selected_card_ui = *selected_card_ui;
                    self.input_state.exit(
                        ui_text,
                        ui_image,
                        ui_transform,
                        &mut self.board_ui,
                        board_game,
                    );
                    self.input_state = InputState::SelectedCardInField {
                        selected_card_ui: selected_card_ui,
                    };
                    self.input_state.enter(
                        ui_text,
                        ui_image,
                        ui_transform,
                        &mut self.board_ui,
                        board_game,
                    );
                } else {
                    // Clear selection...
                    let id = selected_targets_ui.pop().unwrap();
                    self.board_ui.card_uis[id].set_targeted(ui_text, ui_image, false);
                }
            }
            InputState::Blocked => {}
        }
    }

    pub fn handle_event(
        &mut self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        ui_transform: &mut WriteStorage<UiTransform>,
        ui_event: &UiEvent,
        board_game: &BoardGame,
    ) -> Option<GameMove> {
        let card_ui_ids = [
            self.board_ui.player2_back_cards.clone(),
            self.board_ui.player2_center_cards.clone(),
            self.board_ui.player2_front_cards.clone(),
            self.board_ui.player1_front_cards.clone(),
            self.board_ui.player1_center_cards.clone(),
            self.board_ui.player1_back_cards.clone(),
            self.board_ui.player2_hand_cards.clone(),
            self.board_ui.player1_hand_cards.clone(),
        ];
        for card_ui_id in card_ui_ids.iter().flat_map(|v| v.iter()) {
            if ui_event.target == self.board_ui.card_uis[*card_ui_id].event_entity() {
                match ui_event.event_type {
                    UiEventType::Click => {
                        match &mut self.input_state {
                            InputState::Normal => {
                                if let Some(card_id) = self.board_ui.card_uis[*card_ui_id].card_id {
                                    let board = board_game.current_board();
                                    let pos = &board_game.board_resource.cards
                                        [board.cards[card_id]]
                                        .card_position;
                                    let state = if let CardPosition::Hand = pos {
                                        InputState::SelectedCardInHand {
                                            selected_card_ui: *card_ui_id,
                                        }
                                    } else {
                                        InputState::SelectedCardInField {
                                            selected_card_ui: *card_ui_id,
                                        }
                                    };
                                    self.input_state.exit(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    state.enter(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    self.input_state = state;
                                } else {
                                    let state = InputState::SelectedCardInField {
                                        selected_card_ui: *card_ui_id,
                                    };
                                    self.input_state.exit(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    state.enter(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    self.input_state = state;
                                }
                            }
                            InputState::SelectedCardInHand { selected_card_ui } => {
                                let selected_card_ui = *selected_card_ui;
                                if let Some(card_id) = self.board_ui.card_uis[*card_ui_id].card_id {
                                    let board = board_game.current_board();
                                    let pos = &board_game.board_resource.cards
                                        [board.cards[card_id]]
                                        .card_position;
                                    let state = if let CardPosition::Hand = pos {
                                        InputState::SelectedCardInHand {
                                            selected_card_ui: *card_ui_id,
                                        }
                                    } else {
                                        InputState::SelectedCardInField {
                                            selected_card_ui: *card_ui_id,
                                        }
                                    };
                                    self.input_state.exit(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    state.enter(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    self.input_state = state;
                                } else {
                                    // Place Card
                                    // return Some(GameMove::PlaceCard {
                                    //     card_id,
                                    //     position: (),
                                    //     position_index: (),
                                    // });
                                    let state = InputState::Normal;
                                    self.input_state.exit(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    state.enter(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    self.input_state = state;

                                    let card_id =
                                        self.board_ui.card_uis[selected_card_ui].card_id.unwrap();
                                    let card_ui = &self.board_ui.card_uis[*card_ui_id];
                                    return Some(GameMove::PlaceCard {
                                        card_id,
                                        position: card_ui.position.clone(),
                                        position_index: card_ui.position_index,
                                    });
                                }
                            }
                            InputState::SelectedCardInField { selected_card_ui } => {
                                let state = match self.board_ui.card_uis[*card_ui_id].position {
                                    CardPosition::Hand => Some(InputState::SelectedCardInHand {
                                        selected_card_ui: *card_ui_id,
                                    }),
                                    CardPosition::Front
                                    | CardPosition::Center
                                    | CardPosition::Back => Some(InputState::SelectedCardInField {
                                        selected_card_ui: *card_ui_id,
                                    }),
                                    CardPosition::Deck => None,
                                    CardPosition::Grave => None,
                                };
                                if let Some(state) = state {
                                    self.input_state.exit(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    state.enter(
                                        ui_text,
                                        ui_image,
                                        ui_transform,
                                        &mut self.board_ui,
                                        board_game,
                                    );
                                    self.input_state = state;
                                }
                            }
                            InputState::SelectingActionTarget {
                                selected_card_ui,
                                selected_action_ui,
                                selected_targets_ui,
                            } => {
                                let board = &board_game.histories.last().unwrap().board;
                                // Select targe for action
                                if let Some(id) = &self.board_ui.action_container.action_ui
                                    [*selected_action_ui]
                                    .action_id
                                {
                                    let action =
                                        &board_game.board_resource.actions[board.actions[*id]];
                                    let i = selected_targets_ui.len();

                                    let target_type = &board.target_types[action.targets[i]];
                                    // Check target valid
                                    // ...
                                    if self.board_ui.card_uis[*card_ui_id].card_id.is_none() {
                                        // No card
                                        return None;
                                    }
                                    // push to target list
                                    selected_targets_ui.push(*card_ui_id);
                                    // check have enough target
                                    if selected_targets_ui.len() == action.targets.len() {
                                        // if enough, do action and clear input state
                                        let targets = selected_targets_ui
                                            .clone()
                                            .iter()
                                            .map(|i| {
                                                Target::Card(
                                                    self.board_ui.card_uis[*i].card_id.unwrap(),
                                                )
                                            })
                                            .collect();
                                        let result = GameMove::DoAction {
                                            action: *id,
                                            targets,
                                        };

                                        let state = InputState::Normal;
                                        self.input_state.exit(
                                            ui_text,
                                            ui_image,
                                            ui_transform,
                                            &mut self.board_ui,
                                            board_game,
                                        );
                                        state.enter(
                                            ui_text,
                                            ui_image,
                                            ui_transform,
                                            &mut self.board_ui,
                                            board_game,
                                        );
                                        self.input_state = state;
                                        return Some(result);
                                    }
                                }
                            }
                            InputState::Blocked => {}
                        }
                    }
                    UiEventType::HoverStart => {
                        self.board_ui.card_uis[*card_ui_id].set_hovering(ui_text, ui_image, true);
                        // Show detail view for card, if have card
                    }
                    UiEventType::HoverStop => {
                        self.board_ui.card_uis[*card_ui_id].set_hovering(ui_text, ui_image, false);
                        // Hide detail view for card, if have card
                    }
                    _ => (),
                }
            }
        }

        for ui in &mut self.board_ui.action_container.action_ui {
            if ui_event.target == ui.event_entity() {
                match ui_event.event_type {
                    UiEventType::Click => {
                        match &self.input_state {
                            InputState::Normal => {}
                            InputState::SelectedCardInHand { selected_card_ui } => {}
                            InputState::SelectedCardInField { selected_card_ui } => {
                                let selected_card_ui = *selected_card_ui;
                                // Click Action
                                if let Some(action_id) = ui.action_id {
                                    let board = board_game.current_board();
                                    let action = &board_game.board_resource.actions
                                        [board.actions[action_id]];
                                    if action.targets.is_empty() {
                                        // Clear input state
                                        self.input_state.exit(
                                            ui_text,
                                            ui_image,
                                            ui_transform,
                                            &mut self.board_ui,
                                            board_game,
                                        );
                                        self.input_state = InputState::Normal;
                                        self.input_state.enter(
                                            ui_text,
                                            ui_image,
                                            ui_transform,
                                            &mut self.board_ui,
                                            board_game,
                                        );

                                        // Do action...
                                        return Some(GameMove::DoAction {
                                            action: action_id,
                                            targets: Vec::new(),
                                        });
                                    } else {
                                        let target_type = &board_game.board_resource.target_types
                                            [board_game.current_board().target_types
                                                [*action.targets.first().unwrap()]];
                                        let possible_targets: Vec<usize> = target_type
                                            .possible_targets(
                                                &board_game.histories.last().unwrap().board,
                                                &board_game.board_resource,
                                            )
                                            .iter()
                                            .filter_map(|p| match p {
                                                Target::Card(id) => Some(*id),
                                                _ => None,
                                            })
                                            .collect();
                                        for ui in &mut self.board_ui.card_uis {
                                            if let Some(card_id) = ui.card_id {
                                                if possible_targets.contains(&card_id) {
                                                    ui.set_possible_target(ui_text, ui_image, true);
                                                } else {
                                                    ui.set_possible_target(
                                                        ui_text, ui_image, false,
                                                    );
                                                }
                                            }
                                        }

                                        let uiid = ui.id;
                                        ui.set_selected(ui_text, ui_image, true);
                                        self.input_state.exit(
                                            ui_text,
                                            ui_image,
                                            ui_transform,
                                            &mut self.board_ui,
                                            board_game,
                                        );
                                        self.input_state = InputState::SelectingActionTarget {
                                            selected_card_ui: selected_card_ui,
                                            selected_action_ui: uiid,
                                            selected_targets_ui: Default::default(),
                                        };
                                        self.input_state.enter(
                                            ui_text,
                                            ui_image,
                                            ui_transform,
                                            &mut self.board_ui,
                                            board_game,
                                        );
                                        break;
                                    }
                                }
                            }
                            InputState::SelectingActionTarget {
                                selected_card_ui,
                                selected_action_ui,
                                selected_targets_ui,
                            } => {}
                            InputState::Blocked => {}
                        }
                    }
                    UiEventType::HoverStart => {
                        ui.set_hovering(ui_text, ui_image, true);
                    }
                    UiEventType::HoverStop => {
                        ui.set_hovering(ui_text, ui_image, false);
                    }
                    _ => (),
                }
            }
        }

        if ui_event.target == self.board_ui.end_turn_button.event_entity() {
            match ui_event.event_type {
                UiEventType::Click => match &self.input_state {
                    InputState::Normal => {
                        self.input_state.exit(
                            ui_text,
                            ui_image,
                            ui_transform,
                            &mut self.board_ui,
                            board_game,
                        );
                        self.input_state = InputState::Normal;
                        self.input_state.enter(
                            ui_text,
                            ui_image,
                            ui_transform,
                            &mut self.board_ui,
                            board_game,
                        );

                        return Some(GameMove::EndTurn);
                    }
                    InputState::SelectedCardInHand { selected_card_ui } => {}
                    InputState::SelectedCardInField { selected_card_ui } => {}
                    InputState::SelectingActionTarget {
                        selected_card_ui,
                        selected_action_ui,
                        selected_targets_ui,
                    } => {}
                    InputState::Blocked => {}
                },
                UiEventType::HoverStart => {
                    self.board_ui
                        .end_turn_button
                        .set_hovering(ui_text, ui_image, true);
                }
                UiEventType::HoverStop => {
                    self.board_ui
                        .end_turn_button
                        .set_hovering(ui_text, ui_image, false);
                }
                _ => (),
            }
        }
        return None;
    }
}
