use super::{
    action::{ActionId, ActionType},
    board::Board,
    board_game::{Animation, History, Target},
    board_resource::BoardResource,
    card::{Card, CardId, CardPosition},
    player::PlayerId,
    status_effect::StatusEffect,
};
use crate::board_game::board_game::Event;
use std::collections::HashMap;

#[derive(Clone)]
pub enum GameMove {
    PlaceCard {
        card_id: CardId,
        position: CardPosition,
        position_index: usize,
    },
    DoAction {
        action: ActionId,
        targets: Vec<Target>,
    },
    EndTurn,
    // StartTurn,
}

impl GameMove {
    pub fn run(&self, board: &Board, board_resource: &mut BoardResource) -> Vec<History> {
        match self {
            Self::PlaceCard {
                card_id,
                position,
                position_index,
            } => place_card(board, board_resource, *card_id, *position, *position_index),
            Self::DoAction { action, targets } => {
                {
                    let action = &board_resource.actions[board.actions[*action]];
                    // Check enough hp/mp
                    let action_card = &board_resource.cards[board.cards[action.card_id]];
                    if action_card.hp < action.cost_hp || action_card.mp < action.cost_mp {
                        return Vec::new();
                    }
                }
                let mut v = Vec::new();
                match board_resource.actions[board.actions[*action]].action_type {
                    ActionType::Attack { damage } => {
                        if let Some(target) = targets.get(0) {
                            if let Target::Card(id) = target {
                                let mut board = board.clone();
                                {
                                    let action = &board_resource.actions[board.actions[*action]];
                                    let mut action_card =
                                        board_resource.cards[board.cards[action.card_id]].clone();
                                    action_card.hp -= action.cost_hp;
                                    action_card.mp -= action.cost_mp;
                                    let action_name = action.name.clone();
                                    let card_name = action_card.name.clone();
                                    let card_id = action_card.id;
                                    let rid = board_resource.add_card(action_card);
                                    board.cards[card_id] = rid;

                                    let mut message = card_name.clone();
                                    message.push_str(" - ");
                                    message.push_str(&action_name);
                                    let event = Event {
                                        message,
                                        animation: Animation::DoAction(card_id),
                                    };
                                    v.push(History {
                                        board: board.clone(),
                                        event,
                                    });
                                }
                                v.append(&mut take_damage(&mut board, board_resource, *id, damage));
                            }
                        }
                    }
                    ActionType::Heal { hp } => {
                        let mut board = board.clone();
                        {
                            let action = &board_resource.actions[board.actions[*action]];
                            let mut action_card =
                                (board_resource.cards[board.cards[action.card_id]]).clone();
                            action_card.hp -= action.cost_hp;
                            action_card.mp -= action.cost_mp;
                            let card_name = action_card.name.clone();
                            let action_name = action.name.clone();
                            let card_id = action_card.id;
                            let rid = board_resource.add_card(action_card);
                            board.cards[card_id] = rid;

                            let mut message = card_name.clone();
                            message.push_str(" - ");
                            message.push_str(&action_name);
                            let event = Event {
                                message,
                                animation: Animation::DoAction(card_id),
                            };
                            v.push(History {
                                board: board.clone(),
                                event,
                            });
                        }
                        {
                            let action = &board_resource.actions[board.actions[*action]];
                            let mut action_card =
                                (board_resource.cards[board.cards[action.card_id]]).clone();
                            if action_card.hp < action_card.max_hp {
                                action_card.hp += hp;
                                if action_card.hp > action_card.max_hp {
                                    action_card.hp = action_card.max_hp;
                                }
                            }
                            let card_name = action_card.name.clone();
                            let card_id = action_card.id;
                            let rid = board_resource.add_card(action_card);
                            board.cards[card_id] = rid;

                            let mut message = card_name.clone();
                            message.push_str(" recover hp: ");
                            message.push_str(&hp.to_string());
                            let event = Event {
                                message,
                                animation: Animation::Heal(card_id),
                            };
                            v.push(History {
                                board: board.clone(),
                                event,
                            });
                        }
                    }
                    ActionType::Lullaby { duration } => {
                        if let Some(target) = targets.get(0) {
                            if let Target::Card(id) = target {
                                let mut board = board.clone();
                                let mut card = board_resource.cards[board.cards[*id]].clone();

                                if let Some(x) = card.status_effects.get_mut(&StatusEffect::Sleep) {
                                    *x += duration;
                                } else {
                                    card.status_effects.insert(StatusEffect::Sleep, duration);
                                }

                                let card_id = card.id;
                                let rid = board_resource.add_card(card);
                                board.cards[card_id] = rid;

                                let mut message =
                                    board_resource.cards[board.cards[*id]].name.clone();
                                message.push_str(" Lullaby! ");
                                let event = Event {
                                    message,
                                    animation: Animation::Attack(card_id),
                                };
                                v.push(History {
                                    board: board.clone(),
                                    event,
                                });
                            }
                        }
                    }
                    ActionType::Toxin { damage, duration } => {}
                    ActionType::Bind { duration } => {
                        if let Some(target) = targets.get(0) {
                            if let Target::Card(id) = target {
                                let mut board = board.clone();
                                let mut card = board_resource.cards[board.cards[*id]].clone();

                                if let Some(x) =
                                    card.status_effects.get_mut(&StatusEffect::Suppression)
                                {
                                    *x += duration;
                                } else {
                                    card.status_effects
                                        .insert(StatusEffect::Suppression, duration);
                                }

                                let card_id = card.id;
                                let rid = board_resource.add_card(card);
                                board.cards[card_id] = rid;

                                let mut message =
                                    board_resource.cards[board.cards[*id]].name.clone();
                                message.push_str(" Bind! ");
                                let event = Event {
                                    message,
                                    animation: Animation::Attack(card_id),
                                };
                                v.push(History {
                                    board: board.clone(),
                                    event,
                                });
                            }
                        }
                    }
                    ActionType::DrawCard => {}
                    ActionType::ManaTransmission => {}
                    ActionType::Teleport => {}
                    ActionType::Burnout { mp } => {}
                    ActionType::LastStand => {}
                    ActionType::Regenerate { hp } => {}
                    ActionType::Counterattack { damage } => {}
                    ActionType::Defend => {}
                }

                v
            }
            Self::EndTurn => end_turn(board, board_resource),
        }
    }
}

fn take_damage(
    board: &mut Board,
    board_resource: &mut BoardResource,
    id: usize,
    damage: i8,
) -> Vec<History> {
    let mut v = Vec::new();
    {
        // Deduct hp
        let mut card = board_resource.cards[board.cards[id]].clone();
        card.hp -= damage;

        // Remove sleep
        card.status_effects.remove(&StatusEffect::Sleep);

        let card_id = card.id;
        let rid = board_resource.add_card(card);
        board.cards[card_id] = rid;

        let mut message = board_resource.cards[board.cards[id]].name.clone();
        message.push_str(" take damage: ");
        message.push_str(&damage.to_string());
        let event = Event {
            message,
            animation: Animation::Attack(card_id),
        };
        v.push(History {
            board: board.clone(),
            event,
        });
    }
    v.append(&mut check_alive(board, board_resource, id));

    v
}
fn check_alive(board: &mut Board, board_resource: &mut BoardResource, id: usize) -> Vec<History> {
    let mut v = Vec::new();
    {
        // Check alive
        let mut card = (board_resource.cards[board.cards[id]]).clone();
        if card.hp <= 0 {
            let mut player = board_resource.players[board.players[card.player_id]].clone();
            match &card.card_position {
                CardPosition::Front => {
                    player.fronts[card.position_index.unwrap()] = None;
                }
                CardPosition::Center => {
                    player.centers[card.position_index.unwrap()] = None;
                }
                CardPosition::Back => {
                    player.backs[card.position_index.unwrap()] = None;
                }
                _ => {
                    panic!("You should not attack card not in Front/Center/Back!");
                }
            }
            card.card_position = CardPosition::Grave;
            card.position_index = None;

            let card_id = card.id;
            let rid = board_resource.add_card(card);
            board.cards[card_id] = rid;

            let player_id = player.id;
            let rid = board_resource.add_player(player);
            board.players[player_id] = rid;

            let mut message = board_resource.cards[board.cards[id]].name.clone();
            message.push_str(" die!");
            let event = Event {
                message,
                animation: Animation::Die(card_id),
            };
            v.push(History {
                board: board.clone(),
                event,
            });
        }
    }
    {
        // Check player lose
        let card = &board_resource.cards[board.cards[id]];
        if card.hp <= 0 {
            let mut player = &board_resource.players[board.players[card.player_id]];
            if player.player_card_id == card.id {
                // player die
                let event = Event {
                    message: "Game end, player die".to_string(),
                    animation: Animation::EndGame,
                };
                v.push(History {
                    board: board.clone(),
                    event,
                });
            }
        }
    }
    v
}

fn place_card(
    board: &Board,
    board_resource: &mut BoardResource,
    card_id: CardId,
    position: CardPosition,
    position_index: usize,
) -> Vec<History> {
    let mut card = board_resource.cards[board.cards[card_id]].clone();
    let mut player = board_resource.players[board.players[card.player_id]].clone();
    let mut player_card = board_resource.cards[board.cards[player.player_card_id]].clone();
    // Cost hp/mp
    player_card.hp -= card.place_cost_hp;
    player_card.mp -= card.place_cost_mp;
    // Place card
    player.hands.retain(|id| *id != card_id);
    match position {
        CardPosition::Front => {
            card.card_position = CardPosition::Front;
            card.position_index = Some(position_index);
            player.fronts[position_index] = Some(card_id);
        }
        CardPosition::Center => {
            card.card_position = CardPosition::Center;
            card.position_index = Some(position_index);
            player.centers[position_index] = Some(card_id);
        }
        CardPosition::Back => {
            card.card_position = CardPosition::Back;
            card.position_index = Some(position_index);
            player.backs[position_index] = Some(card_id);
        }
        _ => (),
    }

    let mut board = board.clone();
    let pid = player.id;
    let rid = board_resource.add_player(player);
    board.players[pid] = rid;

    let cid = player_card.id;
    let rid = board_resource.add_card(player_card);
    board.cards[cid] = rid;

    let cid = card.id;
    let card_name = card.name.clone();
    let rid = board_resource.add_card(card);
    board.cards[cid] = rid;

    let mut message = String::from("Place Card - ");
    message.push_str(&card_name);
    let event = Event {
        message,
        animation: Animation::PlaceCard(cid),
    };
    let h = History { board, event };

    vec![h]
}
fn end_turn(board: &Board, board_resource: &mut BoardResource) -> Vec<History> {
    let mut h = Vec::new();
    let mut board = board.clone();
    {
        // end turn for current player
        let ids: Vec<usize> = {
            let player = &board_resource.players[board.players[board.current_player_index]];
            [&player.fronts, &player.centers, &player.backs]
                .iter()
                .flat_map(|a| a.iter())
                .filter(|o| o.is_some())
                .map(|o| o.unwrap())
                .collect()
        };

        for id in ids {
            let mut card = board_resource.cards[board.cards[id]].clone();
            for action_id in &card.actions {
                let action = &board_resource.actions[board.actions[*action_id]];
                match action.action_type {
                    ActionType::Attack { damage } => {}
                    ActionType::Heal { hp } => {}
                    ActionType::Lullaby { duration } => {}
                    ActionType::Toxin { damage, duration } => {}
                    ActionType::Bind { duration } => {}
                    ActionType::DrawCard => {}
                    ActionType::ManaTransmission => {}
                    ActionType::Teleport => {}
                    ActionType::Burnout { mp } => {}
                    ActionType::LastStand => {}
                    ActionType::Regenerate { hp } => {
                        if card.hp < card.max_hp {
                            card.hp += hp;
                            if card.hp > card.max_hp {
                                card.hp = card.max_hp;
                            }
                            // updates.insert(c.id, c);

                            let card_id = card.id;
                            let rid = board_resource.add_card(card.clone());
                            board.cards[card_id] = rid;
                        }
                    }
                    ActionType::Counterattack { damage } => {}
                    ActionType::Defend => {}
                }
            }
            if !card.status_effects.is_empty() {
                let mut hash = HashMap::new();
                for (se, duration) in &card.status_effects {
                    match se {
                        StatusEffect::Sleep => {}
                        StatusEffect::Poison => {}
                        StatusEffect::Suppression => {}
                        StatusEffect::Burn => {}
                    }
                    let d = duration - 1;
                    if d > 0 {
                        hash.insert(se.clone(), d);
                    }
                }
                card.status_effects = hash;
            }

            let card_id = card.id;
            let rid = board_resource.add_card(card);
            board.cards[card_id] = rid;
            // Check die....
        }

        let mut message = String::from("End Turn - ");
        message.push_str(
            &board_resource.players[board.players[board.current_player_index]]
                .name
                .clone(),
        );
        let history = History {
            board: board.clone(),
            event: Event {
                message,
                animation: Animation::EndTTurn(board.current_player_index),
            },
        };
        h.push(history);
    }
    // start turn for next player
    board.current_player_index = (board.current_player_index + 1) % board.players.len();
    let ids: Vec<usize> = {
        let player = &board_resource.players[board.players[board.current_player_index]];
        [&player.fronts, &player.centers, &player.backs]
            .iter()
            .map(|a| a.iter())
            .flatten()
            .filter_map(|o| o.as_ref())
            .map(|i| *i)
            .collect()
    };
    for id in ids {
        let mut card = board_resource.cards[board.cards[id]].clone();
        if card.mp < card.max_mp {
            card.mp += 1;

            let card_id = card.id;
            let rid = board_resource.add_card(card);
            board.cards[card_id] = rid;
        }
    }
    let player = &board_resource.players[board.players[board.current_player_index]];
    if !player.decks.is_empty() && player.hands.len() < 5 {
        let mut player = player.clone();
        let id = player.decks.remove(0);
        player.hands.push(id);

        let mut card = board_resource.cards[board.cards[id]].clone();
        card.card_position = CardPosition::Hand;

        let card_id = card.id;
        let rid = board_resource.add_card(card);
        board.cards[card_id] = rid;

        let player_id = player.id;
        let rid = board_resource.add_player(player);
        board.players[player_id] = rid;
    }

    let mut message = String::from("Start Turn - ");
    message.push_str(
        &board_resource.players[board.players[board.current_player_index]]
            .name
            .clone(),
    );
    let history = History {
        board: board.clone(),
        event: Event {
            message,
            animation: Animation::StartTurn(board.current_player_index),
        },
    };
    h.push(history);

    h
}
