use super::{board_game::BoardGame, board_resource::BoardResource, game_move::GameMove};

#[derive(Clone, Default)]
pub struct Board {
    pub current_player_index: usize,
    pub players: Vec<usize>,
    pub cards: Vec<usize>,
    pub attributes: Vec<usize>,
    pub actions: Vec<usize>,
    pub target_types: Vec<usize>,
}

impl Board {
    pub fn possible_moves(&self, board_resource: &BoardResource) -> Vec<GameMove> {
        let player = &board_resource.players[self.players[self.current_player_index]];
        player.possible_moves(&self, board_resource)
    }
    pub fn is_terminal(&self, board_resource: &BoardResource) -> bool {
        board_resource.cards[board_resource.players[self.players[0]].player_card_id].hp < 0
            || board_resource.cards[board_resource.players[self.players[1]].player_card_id].hp < 0
    }
    pub fn heuristic_value(&self, board_resource: &BoardResource) -> i32 {
        let player1 = &board_resource.players[self.players[0]];
        if board_resource.cards[board_resource.players[self.players[0]].player_card_id].hp <= 0 {
            return i32::MAX;
        }
        let player2 = &board_resource.players[self.players[1]];
        if board_resource.cards[board_resource.players[self.players[1]].player_card_id].hp <= 0 {
            return i32::MIN;
        }
        let player1_score = player1.heuristic_value(&self, board_resource);
        let player2_score = player2.heuristic_value(&self, board_resource);
        player2_score - player1_score
    }
}
