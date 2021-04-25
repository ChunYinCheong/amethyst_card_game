use super::{board::Board, board_game::BoardGame, card::CardPosition, game_move::GameMove};
use crate::board_game::board_resource::BoardResource;

pub type PlayerId = usize;

#[derive(Clone)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub hands: Vec<usize>,
    pub fronts: [Option<usize>; 5],
    pub centers: [Option<usize>; 5],
    pub backs: [Option<usize>; 5],
    pub graves: Vec<usize>,
    pub decks: Vec<usize>,
    // pub hp: i32,
    // pub mp: i32,
    pub player_card_id: usize,
}
impl Player {
    pub fn possible_moves(&self, board: &Board, board_resource: &BoardResource) -> Vec<GameMove> {
        let mut moves = Vec::new();
        moves.push(GameMove::EndTurn);

        // Place card in hands
        let mut empty = Vec::new();
        for (i, o) in self.fronts.iter().enumerate() {
            if o.is_none() {
                empty.push((CardPosition::Front, i));
                break;
            }
        }
        for (i, o) in self.centers.iter().enumerate() {
            if o.is_none() {
                empty.push((CardPosition::Center, i));
                break;
            }
        }
        for (i, o) in self.backs.iter().enumerate() {
            if o.is_none() {
                empty.push((CardPosition::Back, i));
                break;
            }
        }
        let mut m = self
            .hands
            .iter()
            .filter(|i| {
                let c = &board_resource.cards[board.cards[**i]];
                let pc = &board_resource.cards[board.cards[self.player_card_id]];
                pc.hp >= c.place_cost_hp && pc.mp >= c.place_cost_mp
            })
            .map(|i| {
                empty.iter().map(move |m| GameMove::PlaceCard {
                    card_id: *i,
                    position: m.0.clone(),
                    position_index: m.1,
                })
            })
            .flatten()
            .collect();
        moves.append(&mut m);

        let mut fields: Vec<GameMove> = [
            self.fronts.clone(),
            self.centers.clone(),
            self.backs.clone(),
        ]
        .iter()
        .flatten()
        .filter_map(|o| *o)
        .map(|id| board_resource.cards[board.cards[id]].possible_moves(board, board_resource))
        .flatten()
        .collect();
        moves.append(&mut fields);

        moves
    }
    pub fn heuristic_value(&self, board: &Board, board_resource: &BoardResource) -> i32 {
        let mut score: i32 = 0;
        let card = &board_resource.cards[board.cards[self.player_card_id]];
        score += card.hp as i32 * 10;
        score += card.mp as i32;
        // score += self.hands.len() as i32;
        for c in self.fronts.iter() {
            if let Some(id) = c {
                score +=
                    board_resource.cards[board.cards[*id]].heuristic_value(board, board_resource);
            }
        }
        for c in self.centers.iter() {
            if let Some(id) = c {
                score +=
                    board_resource.cards[board.cards[*id]].heuristic_value(board, board_resource);
            }
        }
        for c in self.backs.iter() {
            if let Some(id) = c {
                score +=
                    board_resource.cards[board.cards[*id]].heuristic_value(board, board_resource);
            }
        }
        score
    }
}
