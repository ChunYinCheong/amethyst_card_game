use super::{
    board::Board, board_game::BoardGame, board_resource::BoardResource, game_move::GameMove,
    status_effect::StatusEffect,
};
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum CardPosition {
    Deck,
    Hand,
    Front,
    Center,
    Back,
    Grave,
}

pub type CardId = usize;

#[derive(Clone)]
pub struct Card {
    pub id: CardId,
    pub name: String,
    pub attributes: Vec<usize>,
    pub actions: Vec<usize>,
    pub place_cost_hp: i8,
    pub place_cost_mp: i8,
    pub hp: i8,
    pub mp: i8,
    pub max_hp: i8,
    pub max_mp: i8,
    pub face_up: bool,
    pub card_position: CardPosition,
    pub position_index: Option<usize>,
    pub player_id: usize,
    pub texture_path: String,
    pub status_effects: HashMap<StatusEffect, i8>,
}

impl Card {
    pub fn possible_moves(&self, board: &Board, board_resource: &BoardResource) -> Vec<GameMove> {
        if let Some(d) = self.status_effects.get(&StatusEffect::Suppression) {
            if *d > 0 {
                return Vec::new();
            }
        }
        if let Some(d) = self.status_effects.get(&StatusEffect::Sleep) {
            if *d > 0 {
                return Vec::new();
            }
        }
        match &self.card_position {
            CardPosition::Deck => {}
            CardPosition::Hand => {
                let palyer = &board_resource.cards
                    [board_resource.players[board.players[self.player_id]].player_card_id];
                if palyer.hp >= self.place_cost_hp && palyer.mp >= self.place_cost_mp {
                    // Place card
                    // Find Possible field
                    return vec![];
                }
            }
            CardPosition::Front | CardPosition::Center | CardPosition::Back => {
                return self
                    .actions
                    .iter()
                    .map(|id| {
                        board_resource.actions[board.actions[*id]]
                            .possible_moves(board, board_resource)
                    })
                    .flatten()
                    .collect();
            }
            CardPosition::Grave => {}
        }

        return Vec::new();
    }
    pub fn heuristic_value(&self, board: &Board, board_resource: &BoardResource) -> i32 {
        // if card_position == CARD_POSITION.HAND:
        //     return 1
        // elif card_position == CARD_POSITION.FRONT:
        //     return 1 + place_cost_mp + place_cost_hp
        // return 0
        (self.hp + self.mp + 1) as i32
    }
}
