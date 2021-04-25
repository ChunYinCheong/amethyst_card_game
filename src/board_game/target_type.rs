use super::{
    board::Board,
    board_game::{BoardGame, Target},
    board_resource::BoardResource,
};

#[derive(Clone)]
pub struct TargetType {
    pub id: usize,

    pub alliance: bool,
    pub enemy: bool,

    pub action_id: usize,
}

impl TargetType {
    pub fn possible_targets(&self, board: &Board, board_resource: &BoardResource) -> Vec<Target> {
        let mut result = Vec::new();
        let action = &board_resource.actions[board.actions[self.action_id]];
        let card = &board_resource.cards[board.cards[action.card_id]];
        let player = &board_resource.players[board.players[card.player_id]];

        if self.alliance {
            let mut alliance: Vec<Target> = [&player.backs, &player.centers, &player.fronts]
                .iter()
                .flat_map(|v| v.iter())
                .filter_map(|o| o.as_ref())
                .map(|i| Target::Card(*i))
                .collect();
            result.append(&mut alliance);
        }
        if self.enemy {
            let opponent =
                &board_resource.players[board.players[(player.id + 1) % board.players.len()]];
            let mut enemy: Vec<Target> = Vec::new();
            if opponent.fronts.iter().any(|o| o.is_some()) {
                let mut t: Vec<Target> = opponent
                    .fronts
                    .iter()
                    .filter_map(|o| o.as_ref())
                    .map(|i| Target::Card(*i))
                    .collect();
                enemy.append(&mut t);
            } else if opponent.centers.iter().any(|o| o.is_some()) {
                let mut t: Vec<Target> = opponent
                    .centers
                    .iter()
                    .filter_map(|o| o.as_ref())
                    .map(|i| Target::Card(*i))
                    .collect();
                enemy.append(&mut t);
            } else if opponent.backs.iter().any(|o| o.is_some()) {
                let mut t: Vec<Target> = opponent
                    .backs
                    .iter()
                    .filter_map(|o| o.as_ref())
                    .map(|i| Target::Card(*i))
                    .collect();
                enemy.append(&mut t);
            }
            result.append(&mut enemy);
        }

        result
    }
}
