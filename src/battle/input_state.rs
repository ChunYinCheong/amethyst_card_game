use super::{
    action_container::ActionContainer, action_ui::ActionUi, big_icon::BigIcon, card_ui::CardUi,
    end_turn_button::EndTurnButton, log_button::LogButton, message_log::MessageLog,
};
use crate::battle::board_ui::BoardUi;
use crate::board_game::{
    action::Action,
    board::Board,
    board_game::{BoardGame, Target},
    card::{Card, CardPosition},
    target_type::TargetType,
};

use amethyst::{
    assets::{Handle, Loader},
    ecs::{prelude::WorldExt, WriteStorage},
    prelude::*,
    ui::{FontAsset, TtfFormat, UiEvent, UiEventType, UiImage, UiText, UiTransform},
};

pub enum InputState {
    Normal,
    SelectedCardInHand {
        selected_card_ui: usize,
    },
    SelectedCardInField {
        selected_card_ui: usize,
    },
    SelectingActionTarget {
        selected_card_ui: usize,
        selected_action_ui: usize,
        selected_targets_ui: Vec<usize>,
    },
    Blocked,
}

impl InputState {
    pub fn enter(
        &self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        ui_transform: &mut WriteStorage<UiTransform>,
        board_ui: &mut BoardUi,
        board_game: &BoardGame,
    ) {
        match self {
            InputState::Normal => {
                board_ui
                    .message_log
                    .set_visible(ui_text, ui_image, ui_transform, true);
                board_ui
                    .action_container
                    .set_visible(ui_text, ui_image, ui_transform, false);
            }
            InputState::SelectedCardInHand { selected_card_ui } => {
                board_ui.card_uis[*selected_card_ui].set_selected(ui_text, ui_image, true);
                if let Some(card_id) = board_ui.card_uis[*selected_card_ui].card_id {
                    let board = board_game.current_board();
                    let actions = &board_game.board_resource.cards[board.cards[card_id]].actions;
                    for (i, ui) in board_ui.action_container.action_ui.iter_mut().enumerate() {
                        match actions.get(i) {
                            Some(action_id) => {
                                let action =
                                    &board_game.board_resource.actions[board.actions[*action_id]];
                                ui.set_action(ui_text, ui_image, &action);
                            }
                            None => {
                                ui.clear_action(ui_text, ui_image);
                            }
                        }
                    }
                    board_ui
                        .action_container
                        .set_visible(ui_text, ui_image, ui_transform, true);
                    board_ui
                        .message_log
                        .set_visible(ui_text, ui_image, ui_transform, false);

                    // Show field to place

                    let ids = [
                        board_ui.player1_back_cards.clone(),
                        board_ui.player1_center_cards.clone(),
                        board_ui.player1_front_cards.clone(),
                    ];

                    for id in ids.iter().flat_map(|v| v.iter()) {
                        let ui = &mut board_ui.card_uis[*id];
                        if let Some(card_id) = ui.card_id {
                            ui.set_possible_target(ui_text, ui_image, false);
                        } else {
                            ui.set_possible_target(ui_text, ui_image, true);
                        }
                    }
                }
            }
            InputState::SelectedCardInField { selected_card_ui } => {
                board_ui.card_uis[*selected_card_ui].set_selected(ui_text, ui_image, true);
                if let Some(card_id) = board_ui.card_uis[*selected_card_ui].card_id {
                    let board = board_game.current_board();
                    let actions = &board_game.board_resource.cards[board.cards[card_id]].actions;
                    for (i, ui) in board_ui.action_container.action_ui.iter_mut().enumerate() {
                        match actions.get(i) {
                            Some(action_id) => {
                                let action =
                                    &board_game.board_resource.actions[board.actions[*action_id]];
                                ui.set_action(ui_text, ui_image, &action);
                            }
                            None => {
                                ui.clear_action(ui_text, ui_image);
                            }
                        }
                    }
                    board_ui
                        .action_container
                        .set_visible(ui_text, ui_image, ui_transform, true);
                    board_ui
                        .message_log
                        .set_visible(ui_text, ui_image, ui_transform, false);
                }
            }
            InputState::SelectingActionTarget {
                selected_card_ui,
                selected_action_ui,
                selected_targets_ui,
            } => {
                board_ui.action_container.action_ui[*selected_action_ui]
                    .set_selected(ui_text, ui_image, true);
                board_ui
                    .action_container
                    .set_visible(ui_text, ui_image, ui_transform, true);
                board_ui
                    .message_log
                    .set_visible(ui_text, ui_image, ui_transform, false);

                let board = &board_game.histories.last().unwrap().board;
                if let Some(id) =
                    &board_ui.action_container.action_ui[*selected_action_ui].action_id
                {
                    let action = &board_game.board_resource.actions[board.actions[*id]];
                    let i = selected_targets_ui.len();
                    let target_type = &board_game.board_resource.target_types
                        [board.target_types[action.targets[i]]];

                    let possible_targets: Vec<usize> = target_type
                        .possible_targets(board, &board_game.board_resource)
                        .iter()
                        .filter_map(|p| match p {
                            Target::Card(id) => Some(*id),
                            _ => None,
                        })
                        .collect();
                    for ui in &mut board_ui.card_uis {
                        if let Some(card_id) = ui.card_id {
                            if possible_targets.contains(&card_id) {
                                ui.set_possible_target(ui_text, ui_image, true);
                            } else {
                                ui.set_possible_target(ui_text, ui_image, false);
                            }
                        }
                    }

                    for i in selected_targets_ui {
                        board_ui.card_uis[*i].set_selected(ui_text, ui_image, true);
                    }
                }
            }
            InputState::Blocked => {}
        }
    }
    pub fn exit(
        &self,
        ui_text: &mut WriteStorage<UiText>,
        ui_image: &mut WriteStorage<UiImage>,
        ui_transform: &mut WriteStorage<UiTransform>,
        board_ui: &mut BoardUi,
        board_game: &BoardGame,
    ) {
        match self {
            InputState::Normal => {}
            InputState::SelectedCardInHand { selected_card_ui } => {
                board_ui.card_uis[*selected_card_ui].set_selected(ui_text, ui_image, false);
                board_ui
                    .action_container
                    .set_visible(ui_text, ui_image, ui_transform, false);
                board_ui
                    .message_log
                    .set_visible(ui_text, ui_image, ui_transform, true);

                // Clear field highlight

                let ids = [
                    board_ui.player1_back_cards.clone(),
                    board_ui.player1_center_cards.clone(),
                    board_ui.player1_front_cards.clone(),
                ];

                for id in ids.iter().flat_map(|v| v.iter()) {
                    let ui = &mut board_ui.card_uis[*id];
                    ui.set_possible_target(ui_text, ui_image, false);
                }
            }
            InputState::SelectedCardInField { selected_card_ui } => {
                board_ui.card_uis[*selected_card_ui].set_selected(ui_text, ui_image, false);
                board_ui
                    .action_container
                    .set_visible(ui_text, ui_image, ui_transform, false);
                board_ui
                    .message_log
                    .set_visible(ui_text, ui_image, ui_transform, true);
            }
            InputState::SelectingActionTarget {
                selected_card_ui,
                selected_action_ui,
                selected_targets_ui,
            } => {
                board_ui.card_uis[*selected_card_ui].set_selected(ui_text, ui_image, false);
                board_ui.action_container.action_ui[*selected_action_ui]
                    .set_selected(ui_text, ui_image, false);
                for ui in &mut board_ui.card_uis {
                    ui.set_possible_target(ui_text, ui_image, false);
                }
                for i in selected_targets_ui {
                    board_ui.card_uis[*i].set_selected(ui_text, ui_image, false);
                }
            }
            InputState::Blocked => {}
        }
    }
}
